use bevy::prelude::Component;

#[derive(Component)]
pub(crate) struct PlayerBioGroup;

#[derive(Component)]
pub(crate) enum InfoKind {
    Name,
    ID,
    Birthplace,
    Age,
}
