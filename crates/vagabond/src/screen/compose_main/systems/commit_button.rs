use bevy::prelude::{Commands, Entity, Query, Trigger, With};

use crate::screen::compose_main::{CommitButton, PopulatePlayerUi};
use crate::system::ui_effects::{Glower, UiFxTrackedColor};

const GLOWER_COMMIT_SPEED: f32 = 8.0;

pub(super) fn on_commit_button_ui(
    // bevy system
    event: Trigger<PopulatePlayerUi>,
    mut commands: Commands,
    mut glower_q: Query<(Entity, &UiFxTrackedColor, Option<&Glower>), With<CommitButton>>,
) {
    if let Ok((entity, source_color, glower)) = glower_q.get_single_mut() {
        match *event {
            PopulatePlayerUi::Hide => {
                if let Some(glower) = glower {
                    glower.remove(&mut commands, entity);
                }
            }
            PopulatePlayerUi::Show(_) => {
                commands.entity(entity).insert(Glower::new(source_color.color, bevy::color::palettes::basic::GREEN, GLOWER_COMMIT_SPEED));
            }
        };
    }
}
