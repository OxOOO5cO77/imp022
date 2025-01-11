use std::collections::VecDeque;
use std::fmt;

use hall::core::{Attributes, DelayType, Instruction, MachineValueType, ValueTarget};
use hall::hall::HallCard;
use hall::player::PlayerCard;
use hall::view::{GameMachinePlayerView, GameProcessPlayerView};

use crate::private::game::GameProcess;

const QUEUE_SIZE: usize = 10;

pub(crate) struct GameMachineContext {
    pub(crate) free_space: MachineValueType,
    pub(crate) thermal_capacity: MachineValueType,
    pub(crate) system_health: MachineValueType,
    pub(crate) open_ports: MachineValueType,
}

pub struct GameMachine {
    context: GameMachineContext,
    queue: VecDeque<Option<GameProcess>>,
    running: Vec<GameProcess>,
    state: GameMachineState,
}

impl Default for GameMachine {
    fn default() -> Self {
        Self {
            context: GameMachineContext::new(),
            queue: VecDeque::from([const { None }; QUEUE_SIZE]),
            running: Default::default(),
            state: Default::default(),
        }
    }
}

#[derive(Default, PartialEq)]
enum GameMachineState {
    #[default]
    Active,
    Terminated(GameMachineTerminationReason),
}

#[derive(PartialEq)]
enum GameMachineTerminationReason {
    FreeSpace,
    ThermalCapacity,
    SystemHealth,
    OpenPorts,
}

impl fmt::Display for GameMachineTerminationReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GameMachineTerminationReason::*;
        let display = match self {
            FreeSpace => "FreeSpace",
            ThermalCapacity => "ThermalCapacity",
            SystemHealth => "SystemHealth",
            OpenPorts => "OpenPorts",
        };
        write!(f, "{}", display)
    }
}

impl GameMachine {
    pub fn enqueue(&mut self, card: HallCard, local: bool) -> Option<PlayerCard> {
        let (process, mut index) = GameProcess::new_from_card(card, local);
        while let Some(Some(q)) = self.queue.get(index) {
            if q.get_priority() < process.get_priority() {
                break;
            }
            index += 1;
        }

        if index < self.queue.len() {
            let player_card = process.get_card();
            self.queue.pop_back();
            self.queue.insert(index, Some(process));
            Some(player_card)
        } else {
            None
        }
    }

    pub(crate) fn tick(&mut self, attrs: &Attributes) {
        if self.state != GameMachineState::Active {
            return;
        }

        if let Some(Some(mut process)) = self.queue.pop_front() {
            process.launch(&mut self.context, attrs);
            self.running.push(process);
        }
        self.queue.push_back(None);

        self.running.retain(|process| process.get_ttl() > 0);
        self.running.sort();

        for process in self.running.iter_mut() {
            process.run(&mut self.context, attrs);
        }

        self.state = self.context.check_termination();
    }
}

impl GameMachineContext {
    pub(crate) fn new() -> Self {
        Self {
            free_space: 999,
            thermal_capacity: 999,
            system_health: 999,
            open_ports: 999,
        }
    }

    fn check_termination(&self) -> GameMachineState {
        use GameMachineState::*;
        if self.free_space == 0 {
            Terminated(GameMachineTerminationReason::FreeSpace)
        } else if self.thermal_capacity == 0 {
            Terminated(GameMachineTerminationReason::ThermalCapacity)
        } else if self.system_health == 0 {
            Terminated(GameMachineTerminationReason::SystemHealth)
        } else if self.open_ports == 0 {
            Terminated(GameMachineTerminationReason::OpenPorts)
        } else {
            Active
        }
    }

    pub(crate) fn execute(&mut self, instruction: Instruction, attrs: &Attributes) {
        match instruction {
            Instruction::INC(target, amount) => match target {
                ValueTarget::None => {}
                ValueTarget::FreeSpace => self.free_space = self.free_space.saturating_add(amount.resolve(attrs)),
                ValueTarget::ThermalCapacity => self.thermal_capacity = self.thermal_capacity.saturating_add(amount.resolve(attrs)),
                ValueTarget::SystemHealth => self.system_health = self.system_health.saturating_add(amount.resolve(attrs)),
                ValueTarget::OpenPorts => self.open_ports = self.open_ports.saturating_add(amount.resolve(attrs)),
            },
            Instruction::DEC(target, amount) => match target {
                ValueTarget::None => {}
                ValueTarget::FreeSpace => self.free_space = self.free_space.saturating_sub(amount.resolve(attrs)),
                ValueTarget::ThermalCapacity => self.thermal_capacity = self.thermal_capacity.saturating_sub(amount.resolve(attrs)),
                ValueTarget::SystemHealth => self.system_health = self.system_health.saturating_sub(amount.resolve(attrs)),
                ValueTarget::OpenPorts => self.open_ports = self.open_ports.saturating_sub(amount.resolve(attrs)),
            },
            _ => {}
        }
    }
}

impl From<&GameMachine> for GameMachinePlayerView {
    fn from(value: &GameMachine) -> Self {
        let vitals = [value.context.free_space, value.context.thermal_capacity, value.context.system_health, value.context.open_ports];
        let queue = value.queue.iter().enumerate().filter_map(|(idx, item)| item.as_ref().map(|item| (GameProcessPlayerView::from(item), idx as DelayType))).collect::<Vec<_>>();
        let running = value.running.iter().map(GameProcessPlayerView::from).collect::<Vec<_>>();
        Self {
            vitals,
            queue,
            running,
        }
    }
}
