use crate::data::core::{AttributeKind, AttributeValueType, Attributes, ErgType};
use crate::data::hall::HallCard;
use crate::data::player::PlayerCard;
use crate::message::{CardTarget, PicksType};
use rand::{seq::SliceRandom, Rng};
use shared_net::bufferable_derive::Bufferable;
use shared_net::Bufferable;
use shared_net::{op, VSizedBuffer};
use std::collections::{HashMap, VecDeque};

const HAND_SIZE: usize = 5;

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum PlayerCommandState {
    #[default]
    Invalid,
    Expected(op::Command),
    Actual(op::Command),
}

impl PlayerCommandState {
    pub fn is(&self, command: op::Command) -> bool {
        matches!(self, PlayerCommandState::Actual(c) if *c == command)
    }
    pub fn should_be(&mut self, command: op::Command) {
        *self = PlayerCommandState::Expected(command);
    }
    pub fn try_set(&mut self, command: op::Command) -> Result<(), Self> {
        match self {
            PlayerCommandState::Invalid => *self = PlayerCommandState::Actual(command),
            PlayerCommandState::Expected(expected) if command == *expected => *self = PlayerCommandState::Actual(command),
            PlayerCommandState::Actual(actual) if command == *actual => *self = PlayerCommandState::Actual(command),
            _ => return Err(*self),
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct PlayerState {
    attr: Attributes,
    deck: VecDeque<HallCard>,
    heap: Vec<HallCard>,
    hand: [Option<HallCard>; HAND_SIZE],
    erg: HashMap<AttributeKind, ErgType>,
    pub play: Vec<(HallCard, CardTarget)>,
    pub command: PlayerCommandState,
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

    pub(crate) fn fill_hand(&mut self) {
        for hand in self.hand.iter_mut() {
            if hand.is_none() {
                *hand = self.deck.pop_front();
            }
        }
    }

    fn play_card(&mut self, index: usize, target: &CardTarget, erg: &mut HashMap<AttributeKind, ErgType>) -> bool {
        if let Some(Some(card)) = self.hand.get(index) {
            let erg = erg.entry(card.kind).or_insert(0);
            if *erg >= card.cost {
                *erg -= card.cost;
                self.play.push((card.clone(), *target));
                self.hand[index] = None;
                return true;
            }
        }
        false
    }

    pub fn play_cards(&mut self, picks: &PicksType) -> [bool; HAND_SIZE] {
        let mut result = [true; HAND_SIZE];
        let mut temp_erg = self.erg.clone();

        for (index, target) in picks {
            let index = *index as usize;
            result[index] = result[index] && self.play_card(index, target, &mut temp_erg);
        }

        result
    }

    pub fn add_erg(&mut self, kind: AttributeKind, erg_array: ErgArray) {
        const KIND_MAP: [AttributeKind; 4] = [AttributeKind::Analyze, AttributeKind::Breach, AttributeKind::Compute, AttributeKind::Disrupt];
        for (idx, erg) in erg_array.iter().enumerate() {
            let entry = self.erg.entry(KIND_MAP[idx]).or_insert(0);
            *entry += erg;
            if kind == KIND_MAP[idx] {
                *entry += 1;
            }
        }
    }

    pub fn remove_erg(&mut self, kind: AttributeKind, cost: ErgType) {
        self.erg.entry(kind).and_modify(|entry| *entry = entry.saturating_sub(cost));
    }

    pub fn add_to_heap(&mut self, card: HallCard) {
        self.heap.push(card);
    }
}

type DeckCountType = u8;
type AttributeArrays = [[AttributeValueType; 4]; 4];
type ErgArray = [ErgType; 4];

#[derive(Default, Clone, Bufferable)]
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
            hand: player_state.hand.iter().flatten().map(HallCard::to_player_card).collect(),
            erg: [
                //
                *player_state.erg.get(&AttributeKind::Analyze).unwrap_or(&0),
                *player_state.erg.get(&AttributeKind::Breach).unwrap_or(&0),
                *player_state.erg.get(&AttributeKind::Compute).unwrap_or(&0),
                *player_state.erg.get(&AttributeKind::Disrupt).unwrap_or(&0),
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::data::core::Rarity;
    use crate::data::player::player_card::PlayerCard;
    use crate::data::player::player_state::PlayerStatePlayerView;
    use shared_net::{Bufferable, VSizedBuffer};

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
