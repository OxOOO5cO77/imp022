use shared_net::Bufferable;
use shared_net::VSizedBuffer;

use crate::core::PriorityType;
use crate::player::PlayerCard;

#[derive(Default, Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameProcessPlayerView {
    pub player_card: PlayerCard,
    pub priority: PriorityType,
    pub local: bool,
}

#[cfg(test)]
impl GameProcessPlayerView {
    pub fn test_default() -> Self {
        Self {
            player_card: PlayerCard {
                rarity: crate::core::Rarity::Legendary,
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
    use crate::view::GameProcessPlayerView;
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
