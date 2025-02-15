use std::collections::VecDeque;

use hall_lib::core::{DelayType, RuntimeAmountType};
use hall_lib::player::PlayerCard;
use hall_lib::view::{GameMachinePlayerView, GameProcessPlayerView};
use shared_net::UserIdType;

use crate::game::game_process::{GameProcessExecutor, ProcessForPlayer};
use crate::game::game_state::CardResolve;
use crate::game::GameProcess;

const QUEUE_SIZE: usize = 10;

pub(crate) struct GameMachineContext {
    pub(crate) free_space: RuntimeAmountType,
    pub(crate) thermal_capacity: RuntimeAmountType,
    pub(crate) system_health: RuntimeAmountType,
    pub(crate) open_ports: RuntimeAmountType,
}

pub struct GameMachine {
    pub(crate) context: GameMachineContext,
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

impl GameMachine {
    pub(crate) fn enqueue(&mut self, resolve: &CardResolve) -> Option<PlayerCard> {
        let (process, mut index) = GameProcess::new_from_card_resolve(resolve);
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

    pub(crate) fn is_active(&self) -> bool {
        self.state == GameMachineState::Active
    }

    pub(crate) fn tick(&mut self) {
        if let Some(Some(mut process)) = self.queue.pop_front() {
            let launched = process.launch();
            if launched {
                self.running.push(process);
            }
        }
        self.queue.push_back(None);

        self.running.retain(|process| process.get_loop() > 0);
        for process in &mut self.running {
            process.tick();
        }
        self.running.sort();

        self.state = self.context.check_termination();
    }

    pub(crate) fn run(&mut self) -> Vec<GameProcessExecutor> {
        self.running.iter().map(|run| run.build_executable()).collect()
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
}

impl GameMachine {
    pub(crate) fn to_player_view(&self, user_id: UserIdType) -> GameMachinePlayerView {
        let vitals = [self.context.free_space, self.context.thermal_capacity, self.context.system_health, self.context.open_ports];
        let queue = self.queue.iter().enumerate().filter_map(|(idx, item)| item.as_ref().map(|item| (GameProcessPlayerView::process_for_player(item, user_id), idx as DelayType))).collect::<Vec<_>>();
        let running = self.running.iter().map(|item| GameProcessPlayerView::process_for_player(item, user_id)).collect::<Vec<_>>();
        GameMachinePlayerView {
            vitals,
            queue,
            running,
        }
    }
}
