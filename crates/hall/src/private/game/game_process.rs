use std::cmp::Ordering;

use hall::core::{Attributes, Instruction, PriorityType};
use hall::hall::HallCard;
use hall::player::PlayerCard;
use hall::view::GameProcessPlayerView;

use crate::private::game::game_machine::GameMachineContext;

type TTLType = u8;

pub struct GameProcess {
    player_card: PlayerCard,
    launch_code: Vec<Instruction>,
    run_code: Vec<Instruction>,
    priority: PriorityType,
    ttl: TTLType,
    local: bool,
}

impl GameProcess {
    pub(crate) fn new_from_card(card: HallCard, local: bool) -> (Self, usize) {
        (
            Self {
                player_card: (&card).into(),
                launch_code: card.launch_code,
                run_code: card.run_code,
                priority: card.priority,
                ttl: 0,
                local,
            },
            card.delay as usize,
        )
    }

    pub(crate) fn launch(&mut self, context: &mut GameMachineContext, attrs: &Attributes) {
        for code in &self.launch_code {
            match code {
                Instruction::TTL(ttl) => self.ttl = ttl.resolve(attrs) as TTLType,
                _ => context.execute(*code, attrs),
            }
        }
    }

    pub(crate) fn run(&mut self, context: &mut GameMachineContext, attrs: &Attributes) {
        if self.ttl == 0 {
            return;
        }
        self.ttl -= 1;
        for code in &self.run_code {
            context.execute(*code, attrs);
        }
    }
    pub(crate) fn get_ttl(&self) -> TTLType {
        self.ttl
    }
    pub(crate) fn get_card(&self) -> PlayerCard {
        self.player_card
    }
    pub(crate) fn get_priority(&self) -> PriorityType {
        self.priority
    }
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

impl From<&GameProcess> for GameProcessPlayerView {
    fn from(process: &GameProcess) -> Self {
        Self {
            player_card: process.player_card,
            priority: process.priority,
            local: process.local,
        }
    }
}
