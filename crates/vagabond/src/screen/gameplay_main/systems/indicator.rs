use bevy::prelude::{Commands, Entity, Query, Trigger};

use crate::screen::gameplay_main::components::Indicator;
use crate::screen::gameplay_main::events::GamePhaseTrigger;
use crate::screen::gameplay_main::{cleanup_indicator, VagabondGamePhase};

pub(super) fn on_indicator_ui_update(
    // bevy system
    event: Trigger<GamePhaseTrigger>,
    mut commands: Commands,
    indicator_q: Query<(Entity, &Indicator)>,
) {
    match event.phase {
        VagabondGamePhase::Start => {}
        VagabondGamePhase::Play => {}
        VagabondGamePhase::Draw => indicator_q.iter().for_each(|(e, i)| cleanup_indicator(&mut commands, e, i.parent)),
        _ => {}
    }
}
