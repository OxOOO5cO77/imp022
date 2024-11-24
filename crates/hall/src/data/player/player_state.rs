use std::collections::{HashMap, VecDeque};

use crate::data::hall::HallCard;
use crate::data::player::PlayerCard;
use rand::{seq::SliceRandom, Rng};
use shared_data::attribute::{AttributeKind, AttributeValueType, Attributes};
use shared_data::card::ErgType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};
use crate::message::{CardIdxType, CardTarget};

#[derive(Default)]
pub struct PlayerState {
    attr: Attributes,
    deck: VecDeque<HallCard>,
    heap: Vec<HallCard>,
    hand: Vec<HallCard>,
    erg: HashMap<AttributeKind, ErgType>,
    play: Vec<(HallCard,CardTarget)>,
    pub last_command: Option<op::Command>,
    pub resolve_kind: Option<AttributeKind>,
}

impl PlayerState {
    pub fn set_attr(&mut self, attr: Attributes) {
        self.attr = attr;
    }
    pub fn setup_deck(&mut self, deck: Vec<HallCard>, rng: &mut impl Rng) {
        self.deck = deck.into();
        self.shuffle_deck(rng);
        self.fill_hand();
    }
    fn shuffle_deck(&mut self, rng: &mut impl Rng) {
        self.deck.make_contiguous().shuffle(rng)
    }

    fn fill_hand(&mut self) {
        const HAND_SIZE: usize = 5;

        let mut kinds = vec![AttributeKind::Analyze, AttributeKind::Breach, AttributeKind::Compute, AttributeKind::Disrupt];
        kinds.retain(|&kind| !self.hand.iter().any(|card| card.kind == kind));
        for kind in kinds {
            if let Some(index) = self.deck.iter().position(|card| card.kind == kind) {
                if let Some(card) = self.deck.remove(index) {
                    self.hand.push(card);
                }
            }
        }
        while self.hand.len() < HAND_SIZE {
            if let Some(card) = self.deck.pop_front() {
                self.hand.push(card);
            }
        }
    }

    pub fn play_card(&mut self, index: CardIdxType, target: CardTarget) -> bool {
        let index = index as usize;
        if let Some(card) = self.hand.get(index) {
            if card.cost <= *self.erg.get(&card.kind).unwrap_or(&0) {
                let card = self.hand.remove(index);
                self.play.push((card, target));
                return true;
            }
        }
        false
    }

    const KIND_MAP: [AttributeKind; 4] = [AttributeKind::Analyze, AttributeKind::Breach, AttributeKind::Compute, AttributeKind::Disrupt];

    pub fn add_erg(&mut self, kind: AttributeKind, erg_array: ErgArray) {
        for (idx, erg) in erg_array.iter().enumerate() {
            let entry = self.erg.entry(Self::KIND_MAP[idx]).or_insert(0);
            *entry += erg;
            if kind == Self::KIND_MAP[idx] {
                *entry += 1;
            }
        }
    }
}

type DeckCountType = u8;
type AttributeArrays = [[AttributeValueType; 4]; 4];
type ErgArray = [ErgType; 4];

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PlayerStatePlayerView {
    pub attr: AttributeArrays,
    pub deck: DeckCountType,
    pub heap: Vec<PlayerCard>,
    pub hand: Vec<PlayerCard>,
    pub erg: ErgArray,
}

impl From<&PlayerState> for PlayerStatePlayerView {
    fn from(player_state: &PlayerState) -> Self {
        Self {
            attr: player_state.attr.to_arrays(),
            deck: player_state.deck.len() as DeckCountType,
            heap: player_state.heap.iter().map(HallCard::to_player_card).collect(),
            hand: player_state.hand.iter().map(HallCard::to_player_card).collect(),
            erg: [*player_state.erg.get(&AttributeKind::Analyze).unwrap_or(&0), *player_state.erg.get(&AttributeKind::Breach).unwrap_or(&0), *player_state.erg.get(&AttributeKind::Compute).unwrap_or(&0), *player_state.erg.get(&AttributeKind::Disrupt).unwrap_or(&0)],
        }
    }
}

impl Bufferable for PlayerStatePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.attr.push_into(buf);
        self.deck.push_into(buf);
        self.heap.push_into(buf);
        self.hand.push_into(buf);
        self.erg.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let attr = AttributeArrays::pull_from(buf);
        let deck = DeckCountType::pull_from(buf);
        let heap = Vec::<PlayerCard>::pull_from(buf);
        let hand = Vec::<PlayerCard>::pull_from(buf);
        let erg = ErgArray::pull_from(buf);
        Self {
            attr,
            deck,
            heap,
            hand,
            erg,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.attr.size_in_buffer() + self.deck.size_in_buffer() + self.heap.size_in_buffer() + self.hand.size_in_buffer() + self.erg.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::data::player::player_card::PlayerCard;
    use crate::data::player::player_state::PlayerStatePlayerView;
    use shared_data::card::Rarity;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_player_state_player_view() {
        let dummy_card = PlayerCard {
            rarity: Rarity::Legendary,
            number: 123,
            set: 1,
        };

        let orig_view = PlayerStatePlayerView {
            attr: [[1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4], [1, 2, 3, 4]],
            deck: 32,
            heap: vec![dummy_card; 12],
            hand: vec![dummy_card; 5],
            erg: [5, 6, 7, 8],
        };

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
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
