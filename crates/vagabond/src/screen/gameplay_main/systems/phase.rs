use bevy::color::Srgba;
use bevy::prelude::{Query, Sprite, Trigger};

use crate::screen::gameplay_main::components::PhaseIcon;
use crate::screen::gameplay_main::events::GamePhaseTrigger;

pub(super) fn on_phase_ui_update(
    // bevy system
    event: Trigger<GamePhaseTrigger>,
    mut sprite_q: Query<(&mut Sprite, &PhaseIcon)>,
) {
    for (mut sprite, icon) in sprite_q.iter_mut() {
        let color = if event.phase == icon.phase {
            bevy::color::palettes::css::CHARTREUSE
        } else {
            Srgba::new(0.2, 0.2, 0.2, 1.0)
        };
        sprite.color = color.into();
    }
}
