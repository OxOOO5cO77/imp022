use std::collections::{HashMap, VecDeque};

use rand::{seq::SliceRandom, Rng};

use hall_lib::core::{AttributeKind, Attributes, DeckCountType, ErgArray, ErgType, GameSubCommand, MissionNodeIntent, PickedCardTarget};
use hall_lib::hall::HallCard;
use hall_lib::message::PicksType;
use hall_lib::view::GameUserStatePlayerView;

const HAND_SIZE: usize = 5;

#[derive(Default, PartialEq, Copy, Clone, Debug)]
pub enum GameUserCommandState {
    #[default]
    Invalid,
    Expected(GameSubCommand),
    Actual(GameSubCommand),
}

impl GameUserCommandState {
    pub fn is(&self, command: GameSubCommand) -> bool {
        matches!(self, GameUserCommandState::Actual(c) if *c == command)
    }
    pub fn should_be(&mut self, command: GameSubCommand) {
        *self = GameUserCommandState::Expected(command);
    }
    pub fn try_set(&mut self, command: GameSubCommand) -> Result<(), Self> {
        match self {
            GameUserCommandState::Invalid => *self = GameUserCommandState::Actual(command),
            GameUserCommandState::Expected(expected) if command == *expected => *self = GameUserCommandState::Actual(command),
            GameUserCommandState::Actual(actual) if command == *actual => *self = GameUserCommandState::Actual(command),
            _ => return Err(*self),
        }
        Ok(())
    }
}

#[derive(Default)]
pub struct GameUserState {
    attr: Attributes,
    deck: VecDeque<HallCard>,
    heap: Vec<HallCard>,
    hand: [Option<HallCard>; HAND_SIZE],
    erg: HashMap<AttributeKind, ErgType>,
    play: Vec<(HallCard, PickedCardTarget)>,
    pub command: GameUserCommandState,
    pub resolve_kind: Option<AttributeKind>,
    pub intent: MissionNodeIntent,
}

impl GameUserState {
    pub fn set_attr(&mut self, attr: Attributes) {
        self.attr = attr;
    }
    pub fn setup_deck(&mut self, deck: Vec<&HallCard>, rng: &mut impl Rng) {
        self.deck = deck.into_iter().cloned().collect::<Vec<_>>().into();
        self.shuffle_deck(rng);
        self.fill_hand();
    }
    fn shuffle_deck(&mut self, rng: &mut impl Rng) {
        self.deck.make_contiguous().shuffle(rng)
    }
    pub(crate) fn add_to_deck(&mut self, card: HallCard, pos: usize) {
        let pos = pos.min(self.deck.len());
        self.deck.insert(pos, card);
    }

    pub fn fill_hand(&mut self) {
        for hand in self.hand.iter_mut() {
            if hand.is_none() {
                *hand = self.deck.pop_front();
            }
        }
    }

    fn play_card(&mut self, index: usize, target: PickedCardTarget, erg: &mut HashMap<AttributeKind, ErgType>) -> bool {
        if let Some(Some(card)) = self.hand.get(index) {
            let erg = erg.entry(card.kind).or_insert(0);
            if *erg >= card.cost {
                *erg -= card.cost;
                self.play.push((card.clone(), target));
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
            result[index] = result[index] && self.play_card(index, *target, &mut temp_erg);
        }

        result
    }

    pub fn played_cards(&mut self) -> Vec<(HallCard, PickedCardTarget)> {
        self.play.drain(..).collect::<Vec<_>>()
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

impl GameUserState {
    pub(crate) fn to_player_view(&self) -> GameUserStatePlayerView {
        GameUserStatePlayerView {
            attr: self.attr.to_arrays(),
            deck: self.deck.len() as DeckCountType,
            heap: self.heap.iter().map(HallCard::to_player_card).collect(),
            hand: self.hand.iter().flatten().map(HallCard::to_player_card).collect(),
            erg: [
                //
                *self.erg.get(&AttributeKind::Analyze).unwrap_or(&0),
                *self.erg.get(&AttributeKind::Breach).unwrap_or(&0),
                *self.erg.get(&AttributeKind::Compute).unwrap_or(&0),
                *self.erg.get(&AttributeKind::Disrupt).unwrap_or(&0),
            ],
        }
    }
}
