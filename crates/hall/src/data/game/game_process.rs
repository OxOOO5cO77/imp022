use crate::data::game::game_machine::GameMachineContext;
use crate::data::game::GameCode;
use crate::data::hall::hall_card::HallCard;
use shared_data::game::card::{DelayType, PriorityType};
use std::cmp::Ordering;

type TTLType = u8;

pub struct GameProcess {
    launch_code: Vec<GameCode>,
    run_code: Vec<GameCode>,
    delay: DelayType,
    priority: PriorityType,
    ttl: TTLType,
}

impl GameProcess {
    pub(crate) fn new_from_card(card: HallCard) -> Self {
        Self {
            launch_code: card.launch_code.iter().map(|op| GameCode(op.clone())).collect(),
            run_code: card.run_code.iter().map(|op| GameCode(op.clone())).collect(),
            delay: card.delay,
            priority: card.priority,
            ttl: 0,
        }
    }

    pub(crate) fn launch(&mut self, context: &mut GameMachineContext) {
        for code in &self.launch_code {
            if !code.execute(context) {
                break;
            }
        }
    }
    pub(crate) fn run(&mut self, context: &mut GameMachineContext) {
        if self.ttl == 0 {
            return;
        }
        self.ttl -= 1;
        for code in &self.run_code {
            if !code.execute(context) {
                break;
            }
        }
    }
    pub(crate) fn get_ttl(&self) -> TTLType { self.ttl }
    pub(crate) fn get_delay(&self) -> DelayType { self.delay }
}

impl Eq for GameProcess {}

impl PartialEq for GameProcess {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl Ord for GameProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for GameProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
