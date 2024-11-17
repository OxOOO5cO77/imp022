use crate::data::game::GameProcess;

use crate::data::game::game_process::GameProcessPlayerView;
use crate::data::hall::HallCard;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;
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
    queue: Vec<GameProcess>,
    running: Vec<GameProcess>,
    state: GameMachineState,
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
    pub fn enqueue(&mut self, card: HallCard, local: bool) {
        let process = GameProcess::new_from_card(card, local);
        self.queue.insert(process.get_delay() as usize, process);
    }

    pub(crate) fn tick(&mut self) {
        if self.state == GameMachineState::Active {
            return;
        }

        if !self.queue.is_empty() {
            let mut process = self.queue.remove(0);
            process.launch(&mut self.context);
            self.running.push(process);
        }

        self.running.retain(|process| process.get_ttl() > 0);
        self.running.sort();

        for process in self.running.iter_mut() {
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
            Active
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMachinePlayerView {
    pub stats: [MachineValueType; 4],
    pub queue: Vec<GameProcessPlayerView>,
    pub running: Vec<GameProcessPlayerView>,
}

impl From<&GameMachine> for GameMachinePlayerView {
    fn from(value: &GameMachine) -> Self {
        let stats = [value.context.free_space, value.context.thermal_capacity, value.context.system_health, value.context.open_ports];
        let queue = value.queue.iter().map(GameProcessPlayerView::from).collect::<Vec<_>>();
        let running = value.running.iter().map(GameProcessPlayerView::from).collect::<Vec<_>>();
        Self {
            stats,
            queue,
            running,
        }
    }
}

impl Bufferable for GameMachinePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.stats.push_into(buf);
        self.queue.push_into(buf);
        self.running.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let stats = <[MachineValueType; 4]>::pull_from(buf);
        let queue = <Vec<GameProcessPlayerView>>::pull_from(buf);
        let running = <Vec<GameProcessPlayerView>>::pull_from(buf);
        Self {
            stats,
            queue,
            running,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.stats.size_in_buffer() + self.queue.size_in_buffer() + self.running.size_in_buffer()
    }
}

#[cfg(test)]
impl GameMachinePlayerView {
    pub fn test_default() -> Self {
        Self {
            stats: [1, 2, 3, 4],
            queue: vec![GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default()],
            running: vec![GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default()],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game::game_machine::GameMachinePlayerView;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_machine_player_view() {
        let mut buf = VSizedBuffer::new(64);

        let orig = GameMachinePlayerView::test_default();

        buf.push(&orig);
        let result = buf.pull::<GameMachinePlayerView>();

        assert_eq!(orig.stats, result.stats);
        assert_eq!(orig.queue, result.queue);
        assert_eq!(orig.running, result.running);
    }
}
