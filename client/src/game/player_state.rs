use std::collections::{HashMap, VecDeque};
use imp022_shared::player::card::{Card, Kind};
use imp022_shared::player::Player;

use rand::{Rng, seq::SliceRandom};

pub struct PlayerState {
    pub player: Player,
    deck: VecDeque<Card>,
    discard: Vec<Card>,
    hand: Vec<Card>,
    pub erg: HashMap<Kind, u32>,
    pub pick: Option<Kind>,
    play: Vec<Card>,
}

impl PlayerState {
    pub fn from_player(player: Player) -> Self {
        let deck = player.deck.clone();

        PlayerState {
            player,
            deck,
            discard: Vec::new(),
            hand: Vec::new(),
            erg: HashMap::new(),
            pick: None,
            play: Vec::new(),
        }
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

    pub fn get_pick_attr(&self) -> Option<[u8; 4]> {
        Some(match self.pick? {
            Kind::Analyze => self.player.attributes.analyze.to_array(),
            Kind::Breach => self.player.attributes.breach.to_array(),
            Kind::Compute => self.player.attributes.compute.to_array(),
            Kind::Disrupt => self.player.attributes.disrupt.to_array(),
        })
    }

    fn play(&mut self, index: usize) -> bool {
        if let Some(card) = self.deck.remove(index) {
            if card.cost <= *self.erg.get(&card.kind).unwrap_or(&0) {
                self.play.push(card);
                return true;
            }
        }
        false
    }
}
