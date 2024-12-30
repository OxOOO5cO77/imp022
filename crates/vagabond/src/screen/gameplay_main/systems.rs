mod erg;
mod hand;
mod indicator;
mod local;
mod machine;
mod mission;
mod phase;
mod remote;
mod roll;
mod tty;

use bevy::prelude::{ChildBuild, ChildBuilder, Observer};

pub(super) struct GameplaySystems;

impl GameplaySystems {
    pub(super) fn observe(parent: &mut ChildBuilder) {
        parent.spawn(Observer::new(tty::on_tty_update));
        parent.spawn(Observer::new(roll::on_roll_ui_update_roll));
        parent.spawn(Observer::new(roll::on_roll_ui_update_resources));
        parent.spawn(Observer::new(indicator::on_indicator_ui_update));
        parent.spawn(Observer::new(hand::on_hand_ui_update));
        parent.spawn(Observer::new(erg::on_erg_ui_update));
        parent.spawn(Observer::new(phase::on_phase_ui_update));
        parent.spawn(Observer::new(mission::on_mission_ui_update));
        parent.spawn(Observer::new(local::on_local_state_update_player));
        parent.spawn(Observer::new(local::on_local_ui_update_attr));
        parent.spawn(Observer::new(local::on_local_ui_update_player));
        parent.spawn(Observer::new(remote::on_remote_ui_update_roll));
        parent.spawn(Observer::new(remote::on_remote_ui_update_resources));
        parent.spawn(Observer::new(machine::on_machine_ui_update_info));
        parent.spawn(Observer::new(machine::on_machine_ui_update_state));
    }
}
