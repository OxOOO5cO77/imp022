use bevy::prelude::{Commands, Entity, Query, Text2d, On, Visibility, With, Without};

use crate::screen::compose_main::{InfoKind, PlayerBioGroup, PopulatePlayerUi};
use crate::screen::shared::CardLayout;

pub(super) fn on_populate_bio_ui(
    // bevy system
    event: On<PopulatePlayerUi>,
    mut commands: Commands,
    mut info_q: Query<(&mut Text2d, &InfoKind), Without<CardLayout>>,
    bio_q: Query<Entity, With<PlayerBioGroup>>,
) {
    let visibility = match &*event {
        PopulatePlayerUi::Hide => Visibility::Hidden,
        PopulatePlayerUi::Show(data) => {
            for (mut info, info_kind) in info_q.iter_mut() {
                match info_kind {
                    InfoKind::Name => *info = data.name.clone().into(),
                    InfoKind::ID => *info = data.id.clone().into(),
                    InfoKind::Birthplace => *info = data.birthplace().clone().into(),
                    InfoKind::Age => *info = data.age().to_string().into(),
                }
            }
            Visibility::Visible
        }
    };
    if let Ok(bio) = bio_q.single() {
        commands.entity(bio).insert(visibility);
    }
}
