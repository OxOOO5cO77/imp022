use bevy::prelude::{Query, Text2d, On};

use crate::screen::gameplay_main::components::PlayerStateText;
use crate::screen::gameplay_main::events::PlayerErgTrigger;

pub(super) fn on_erg_ui_update(
    // bevy system
    event: On<PlayerErgTrigger>,
    mut erg_q: Query<(&mut Text2d, &PlayerStateText)>,
) {
    for (mut erg_text, state_text) in erg_q.iter_mut() {
        if let PlayerStateText::Erg(index) = state_text {
            *erg_text = format!("{}", event.erg[*index]).into();
        }
    }
}
