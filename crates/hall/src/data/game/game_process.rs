use crate::data::game::game_machine::GameMachineContext;
use crate::data::hall::hall_card::HallCard;
use crate::data::player::PlayerCard;
use shared_data::card::{DelayType, PriorityType};
use shared_data::instruction::Instruction;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;
use std::cmp::Ordering;

type TTLType = u8;

pub struct GameProcess {
    player_card: PlayerCard,
    delay: DelayType,
    launch_code: Vec<Instruction>,
    run_code: Vec<Instruction>,
    priority: PriorityType,
    ttl: TTLType,
    local: bool,
}

impl GameProcess {
    pub(crate) fn new_from_card(card: HallCard, local: bool) -> Self {
        Self {
            player_card: (&card).into(),
            launch_code: card.launch_code,
            run_code: card.run_code,
            delay: card.delay,
            priority: card.priority,
            ttl: 0,
            local,
        }
    }

    pub(crate) fn launch(&mut self, context: &mut GameMachineContext) {
        for code in &self.launch_code {
            if !context.execute(*code) {
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
            if !context.execute(*code) {
                break;
            }
        }
    }
    pub(crate) fn get_ttl(&self) -> TTLType {
        self.ttl
    }
    pub(crate) fn get_delay(&self) -> DelayType {
        self.delay
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

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameProcessPlayerView {
    pub player_card: PlayerCard,
    pub delay: DelayType,
    pub priority: PriorityType,
    pub local: bool,
}

impl From<&GameProcess> for GameProcessPlayerView {
    fn from(process: &GameProcess) -> Self {
        Self {
            player_card: process.player_card,
            delay: process.delay,
            priority: process.priority,
            local: process.local,
        }
    }
}

impl Bufferable for GameProcessPlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_card.push_into(buf);
        self.delay.push_into(buf);
        self.priority.push_into(buf);
        self.local.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_card = PlayerCard::pull_from(buf);
        let delay = DelayType::pull_from(buf);
        let priority = PriorityType::pull_from(buf);
        let local = bool::pull_from(buf);
        Self {
            player_card,
            delay,
            priority,
            local,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_card.size_in_buffer() + self.delay.size_in_buffer() + self.priority.size_in_buffer() + self.local.size_in_buffer()
    }
}

#[cfg(test)]
impl GameProcessPlayerView {
    pub fn test_default() -> Self {
        Self {
            player_card: PlayerCard {
                rarity: shared_data::card::Rarity::Legendary,
                number: 123,
                set: 2,
            },
            delay: 5,
            priority: 10,
            local: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game::game_process::GameProcessPlayerView;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_process_player_view() {
        let orig = GameProcessPlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameProcessPlayerView>();

        assert_eq!(orig.player_card, result.player_card);
        assert_eq!(orig.delay, result.delay);
        assert_eq!(orig.local, result.local);
    }
}
