use crate::data::core::{Instruction, PriorityType};
use crate::data::game::game_machine::GameMachineContext;
use crate::data::hall::HallCard;
use crate::data::player::PlayerCard;
use shared_net::bufferable_derive::Bufferable;
use shared_net::Bufferable;
use shared_net::VSizedBuffer;
use std::cmp::Ordering;

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

    pub(crate) fn launch(&mut self, context: &mut GameMachineContext) {
        for code in &self.launch_code {
            match code {
                Instruction::TTL(ttl) => self.ttl = *ttl as TTLType,
                _ => context.execute(*code),
            }
        }
    }

    pub(crate) fn run(&mut self, context: &mut GameMachineContext) {
        if self.ttl == 0 {
            return;
        }
        self.ttl -= 1;
        for code in &self.run_code {
            context.execute(*code);
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

#[derive(Default, Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameProcessPlayerView {
    pub player_card: PlayerCard,
    pub priority: PriorityType,
    pub local: bool,
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

#[cfg(test)]
impl GameProcessPlayerView {
    pub fn test_default() -> Self {
        Self {
            player_card: PlayerCard {
                rarity: crate::data::core::Rarity::Legendary,
                number: 123,
                set: 2,
            },
            priority: 5,
            local: true,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::game::game_process::GameProcessPlayerView;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_process_player_view() {
        let orig = GameProcessPlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameProcessPlayerView>();

        assert_eq!(orig.player_card, result.player_card);
        assert_eq!(orig.priority, result.priority);
        assert_eq!(orig.local, result.local);
    }
}
