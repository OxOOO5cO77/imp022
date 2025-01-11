use hall::core::MissionNodeIntent;

use crate::private::game::{GameMission, GameUser};

mod link;
mod node_a;
mod node_b;
mod node_c;
mod node_d;
mod node_e;
mod node_f;
mod node_g;
mod node_h;

pub(crate) fn process_intent(mission: &mut GameMission, user: &mut GameUser, intent: MissionNodeIntent) -> bool {
    match intent {
        MissionNodeIntent::None => false,
        MissionNodeIntent::Link(dir) => link::process_intent(mission, user, dir),
        MissionNodeIntent::AccessPoint(intent) => node_a::process_intent(mission, user, intent),
        MissionNodeIntent::Backend(intent) => node_b::process_intent(mission, user, intent),
        MissionNodeIntent::Control(intent) => node_c::process_intent(mission, user, intent),
        MissionNodeIntent::Database(intent) => node_d::process_intent(mission, user, intent),
        MissionNodeIntent::Engine(intent) => node_e::process_intent(mission, user, intent),
        MissionNodeIntent::Frontend(intent) => node_f::process_intent(mission, user, intent),
        MissionNodeIntent::Gateway(intent) => node_g::process_intent(mission, user, intent),
        MissionNodeIntent::Hardware(intent) => node_h::process_intent(mission, user, intent),
    }
}
