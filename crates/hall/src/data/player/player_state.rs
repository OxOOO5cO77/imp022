use std::collections::{HashMap, VecDeque};

use crate::data::hall::hall_card::HallCard;
use crate::data::player::player_card::PlayerCard;
use rand::{seq::SliceRandom, Rng};
use shared_data::game::card::{ErgType, Kind};
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[derive(Default)]
pub struct PlayerState {
    deck: VecDeque<HallCard>,
    discard: Vec<HallCard>,
    hand: Vec<HallCard>,
    erg: HashMap<Kind, ErgType>,
    play: Vec<HallCard>,
    pub last_command: Option<op::Command>,
    pub resolve_kind: Option<Kind>,
}

impl PlayerState {
    pub fn setup(&mut self, deck: Vec<HallCard>, rng: &mut impl Rng) {
        self.deck = deck.into();
        self.shuffle_deck(rng);
        self.fill_hand();
    }
    fn shuffle_deck(&mut self, rng: &mut impl Rng) {
        self.deck.make_contiguous().shuffle(rng)
    }

    fn fill_hand(&mut self) {
        let mut kinds = vec![Kind::Analyze, Kind::Breach, Kind::Compute, Kind::Disrupt];
        for card in &self.hand {
            if let Some(index) = kinds.iter().position(|kind| card.kind == *kind) {
                kinds.remove(index);
            }
        }
        for kind in kinds {
            if let Some(index) = self.deck.iter().position(|card| card.kind == kind) {
                if let Some(card) = self.deck.remove(index) {
                    self.hand.push(card);
                }
            }
        }
        while self.hand.len() < 6 {
            if let Some(card) = self.deck.pop_front() {
                self.hand.push(card);
            }
        }
    }

    pub fn play_card(&mut self, index: usize) -> bool {
        if let Some(card) = self.hand.get(index) {
            if card.cost <= *self.erg.get(&card.kind).unwrap_or(&0) {
                let card = self.hand.remove(index);
                self.play.push(card);
                return true;
            }
        }
        false
    }

    pub fn add_erg(&mut self, kind: Kind, erg: ErgType) {
        let entry = self.erg.entry(kind).or_insert(0);
        *entry += erg;
    }
}

type DeckCountType = u8;
type ErgArray = [ErgType; 4];

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PlayerStatePlayerView {
    pub deck: DeckCountType,
    pub heap: Vec<PlayerCard>,
    pub hand: Vec<PlayerCard>,
    pub erg: ErgArray,
}

impl From<&PlayerState> for PlayerStatePlayerView {
    fn from(player_state: &PlayerState) -> Self {
        Self {
            deck: player_state.deck.len() as DeckCountType,
            heap: player_state.discard.iter().map(HallCard::to_player_card).collect(),
            hand: player_state.hand.iter().map(HallCard::to_player_card).collect(),
            erg: [
                *player_state.erg.get(&Kind::Analyze).unwrap_or(&0),
                *player_state.erg.get(&Kind::Breach).unwrap_or(&0),
                *player_state.erg.get(&Kind::Compute).unwrap_or(&0),
                *player_state.erg.get(&Kind::Disrupt).unwrap_or(&0),
            ],

        }
    }
}

impl Bufferable for PlayerStatePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.deck.push_into(buf);
        self.heap.push_into(buf);
        self.hand.push_into(buf);
        self.erg.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let deck = DeckCountType::pull_from(buf);
        let heap = Vec::<PlayerCard>::pull_from(buf);
        let hand = Vec::<PlayerCard>::pull_from(buf);
        let erg = ErgArray::pull_from(buf);
        Self {
            deck,
            heap,
            hand,
            erg,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.deck.size_in_buffer() + self.heap.size_in_buffer() + self.hand.size_in_buffer() + self.erg.size_in_buffer()
    }
}

#[cfg(test)]
mod tests {
    use crate::data::player::player_card::PlayerCard;
    use crate::data::player::player_state::PlayerStatePlayerView;
    use shared_data::game::card::Rarity;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_player_state_player_view() {
        let mut buf = VSizedBuffer::new(64);

        let dummy_card = PlayerCard {
            rarity: Rarity::Legendary,
            number: 123,
            set: 1,
        };

        let orig_view = PlayerStatePlayerView {
            deck: 32,
            heap: vec![dummy_card; 12],
            hand: vec![dummy_card; 5],
            erg: [5, 6, 7, 8],
        };

        buf.push(&orig_view);
        let new_view = buf.pull::<PlayerStatePlayerView>();

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
