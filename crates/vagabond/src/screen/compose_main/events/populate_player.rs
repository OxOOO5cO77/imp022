use bevy::prelude::Event;
use warehouse::data::player_bio::PlayerBio;

#[derive(Event)]
pub(crate) enum PopulatePlayerUi {
    Hide,
    Show(PlayerBio),
}
