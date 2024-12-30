use crate::screen::gameplay_main::{MachineKind, VagabondGamePhase};
use bevy::prelude::Resource;
use hall::data::core::{AttributeKind, MissionNodeIdType};
use hall::data::game::TickType;
use hall::data::player::PlayerStatePlayerView;
use hall::message::{CardIdxType, CardTarget};
use std::collections::{HashMap, VecDeque};
use vagabond::data::{VagabondCard, VagabondMachine};

#[derive(Resource)]
pub(crate) struct GameplayContext {
    pub(crate) tick: TickType,
    pub(crate) phase: VagabondGamePhase,
    pub(crate) attr_pick: Option<AttributeKind>,
    pub(crate) card_picks: HashMap<CardIdxType, CardTarget>,
    pub(crate) current_remote: MissionNodeIdType,
    pub(crate) hand: Vec<VagabondCard>,
    pub(crate) tty: HashMap<MachineKind, VecDeque<String>>,
    pub(crate) cached_state: PlayerStatePlayerView,
    pub(crate) cached_local: VagabondMachine,
    pub(crate) cached_remote: VagabondMachine,
}

impl Default for GameplayContext {
    fn default() -> Self {
        Self {
            tick: Default::default(),
            phase: Default::default(),
            attr_pick: None,
            card_picks: Default::default(),
            current_remote: 1,
            hand: Default::default(),
            tty: Default::default(),
            cached_state: Default::default(),
            cached_local: Default::default(),
            cached_remote: Default::default(),
        }
    }
}

impl GameplayContext {
    pub(crate) fn reset(&mut self, tick: TickType) {
        self.attr_pick = None;
        self.card_picks.clear();
        self.tick = tick;
    }

    pub(crate) fn add_card_pick(&mut self, index: usize, target: MachineKind) {
        let card_idx = index as CardIdxType;
        let card_target = match target {
            MachineKind::Local => CardTarget::Local,
            MachineKind::Remote => CardTarget::Remote(self.current_remote),
        };
        self.card_picks.insert(card_idx, card_target);
    }
}
