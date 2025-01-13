use shared_net::{Bufferable, VSizedBuffer};

use crate::core::{AttributeArrays, DeckCountType, ErgArray};
use crate::player::PlayerCard;

#[derive(Default, Clone, Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUserStatePlayerView {
    pub attr: AttributeArrays,
    pub deck: DeckCountType,
    pub heap: Vec<PlayerCard>,
    pub hand: Vec<PlayerCard>,
    pub erg: ErgArray,
}

#[cfg(test)]
mod test {
    use crate::core::Rarity;
    use crate::player::PlayerCard;
    use crate::view::game_user_state::GameUserStatePlayerView;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_player_state_player_view() {
        let dummy_card = PlayerCard {
            rarity: Rarity::Legendary,
            number: 123,
            set: 1,
        };

        let orig_view = GameUserStatePlayerView {
            attr: [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]],
            deck: 32,
            heap: vec![dummy_card; 12],
            hand: vec![dummy_card; 5],
            erg: [5, 6, 7, 8],
        };

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameUserStatePlayerView>();

        assert_eq!(orig_view.deck, new_view.deck);
        assert_eq!(orig_view.heap.len(), new_view.heap.len());
        assert_eq!(orig_view.heap[0], new_view.heap[0]);
        assert_eq!(orig_view.heap[1], new_view.heap[1]);
        assert_eq!(orig_view.hand.len(), new_view.hand.len());
        assert_eq!(orig_view.hand[0], new_view.hand[0]);
        assert_eq!(orig_view.hand[1], new_view.hand[1]);
        assert_eq!(orig_view.erg, new_view.erg);
    }
}
