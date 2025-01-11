use std::collections::{HashMap, VecDeque};

use bevy::prelude::Resource;
use hall::core::{AttributeKind, TickType};
use hall::message::{CardIdxType, CardTarget};
use hall::player::PlayerStatePlayerView;
use hall::view::GameMissionPlayerView;
use vagabond::data::{VagabondCard, VagabondMachine};

use crate::screen::gameplay_main::nodes::MissionNodeAction;
use crate::screen::gameplay_main::{MachineKind, VagabondGamePhase};

#[derive(Resource, Default)]
pub(crate) struct GameplayContext {
    pub(crate) player_id: String,
    pub(crate) tick: TickType,
    pub(crate) phase: VagabondGamePhase,
    pub(crate) attr_pick: Option<AttributeKind>,
    pub(crate) card_picks: HashMap<CardIdxType, CardTarget>,
    pub(crate) node_action: MissionNodeAction,
    pub(crate) hand: Vec<VagabondCard>,
    pub(crate) tty: HashMap<MachineKind, VecDeque<String>>,
    pub(crate) cached_state: PlayerStatePlayerView,
    pub(crate) cached_local: VagabondMachine,
    pub(crate) cached_remote: VagabondMachine,
    pub(crate) cached_mission: GameMissionPlayerView,
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
            MachineKind::Remote => CardTarget::Remote(self.cached_mission.current().id),
        };
        self.card_picks.insert(card_idx, card_target);
    }
}
