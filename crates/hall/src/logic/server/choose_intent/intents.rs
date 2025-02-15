use hall_lib::core::{MissionNodeIntent, TickType};
use hall_lib::message::UpdateTokenMessage;

use crate::game::{GameMission, GameUser};
use crate::manager::data_manager::DataManager;

mod link;
mod node_a;
mod node_b;
mod node_c;
mod node_d;
mod node_e;
mod node_f;
mod node_g;
mod node_h;

pub(crate) enum IntentResult {
    Node,
    Token(Vec<UpdateTokenMessage>),
    Deck,
}

pub(crate) fn process_intent(intent: MissionNodeIntent, mission: &mut GameMission, user: &mut GameUser, tick: TickType, dm: &DataManager) -> Option<Vec<IntentResult>> {
    match intent {
        MissionNodeIntent::None => None,
        MissionNodeIntent::Link(dir) => link::process_intent(dir, user, mission),
        MissionNodeIntent::AccessPoint(intent) => node_a::process_intent(intent, mission, user, tick, dm),
        MissionNodeIntent::Backend(intent) => node_b::process_intent(intent, mission, user),
        MissionNodeIntent::Control(intent) => node_c::process_intent(intent, mission, user),
        MissionNodeIntent::Database(intent) => node_d::process_intent(intent, mission, user),
        MissionNodeIntent::Engine(intent) => node_e::process_intent(intent, mission, user),
        MissionNodeIntent::Frontend(intent) => node_f::process_intent(intent, mission, user),
        MissionNodeIntent::Gateway(intent) => node_g::process_intent(intent, mission, user),
        MissionNodeIntent::Hardware(intent) => node_h::process_intent(intent, mission, user),
    }
}
