use bevy::prelude::Event;
use warehouse_lib::data::player_bio::PlayerBio;

#[derive(Event)]
pub(crate) enum PopulatePlayerUi {
    Hide,
    Show(PlayerBio),
}
