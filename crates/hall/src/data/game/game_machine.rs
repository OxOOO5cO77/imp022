use crate::data::game::GameProcess;
use std::collections::VecDeque;

use crate::data::hall_card::HallCard;
use std::fmt;

type MachineValueType = u16;

#[derive(Default)]
pub struct GameMachine {
    free_space: MachineValueType,
    thermal_capacity: MachineValueType,
    system_health: MachineValueType,
    open_ports: MachineValueType,
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
    pub(crate) fn enqueue(&mut self, card: HallCard) {
        let process = GameProcess::new_from_card(card);
        self.queue.insert(process.get_delay() as usize, process);
    }

    pub(crate) fn tick(&mut self) {
        if self.state == GameMachineState::Running {
            return;
        }

        if let Some(mut process) = self.queue.pop_front() {
            process.launch();
            self.heap.push(process);
        }

        self.heap.retain(|process| process.get_ttl() > 0);
        self.heap.sort();

        for process in self.heap.iter_mut() {
            process.run();
        }

        self.state = self.check_state();
    }

    fn check_state(&self) -> GameMachineState {
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

    pub(crate) fn adjust_free_space(&mut self, amount: i16) {
        self.free_space = self.free_space.saturating_add_signed(amount);
    }
    pub(crate) fn adjust_thermal_capacity(&mut self, amount: i16) {
        self.thermal_capacity = self.thermal_capacity.saturating_add_signed(amount);
    }
    pub(crate) fn adjust_system_health(&mut self, amount: i16) {
        self.system_health = self.system_health.saturating_add_signed(amount);
    }
    pub(crate) fn adjust_open_ports(&mut self, amount: i16) {
        self.open_ports = self.open_ports.saturating_add_signed(amount);
    }
}
