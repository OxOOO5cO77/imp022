use crate::data::game::GameProcess;
use std::collections::VecDeque;

use crate::data::hall::hall_card::HallCard;
use std::fmt;

type MachineValueType = u16;

#[derive(Default)]
pub(crate) struct GameMachineContext {
    pub(crate) free_space: MachineValueType,
    pub(crate) thermal_capacity: MachineValueType,
    pub(crate) system_health: MachineValueType,
    pub(crate) open_ports: MachineValueType,
}

#[derive(Default)]
pub struct GameMachine {
    context: GameMachineContext,
    queue: VecDeque<GameProcess>,
    heap: Vec<GameProcess>,
    state: GameMachineState,
}

#[derive(Default, PartialEq)]
enum GameMachineState {
    #[default] Running,
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
    pub fn enqueue(&mut self, card: HallCard) {
        let process = GameProcess::new_from_card(card);
        self.queue.insert(process.get_delay() as usize, process);
    }

    pub(crate) fn tick(&mut self) {
        if self.state == GameMachineState::Running {
            return;
        }

        if let Some(mut process) = self.queue.pop_front() {
            process.launch(&mut self.context);
            self.heap.push(process);
        }

        self.heap.retain(|process| process.get_ttl() > 0);
        self.heap.sort();

        for process in self.heap.iter_mut() {
            process.run(&mut self.context);
        }

        self.state = self.context.check_termination();
    }
}

impl GameMachineContext {
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
            Running
        }
    }
}
