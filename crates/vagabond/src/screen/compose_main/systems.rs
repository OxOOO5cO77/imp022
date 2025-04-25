mod commit_button;
mod finish_player;
mod populate_bio;
mod populate_deck;

use bevy::prelude::{ChildSpawnerCommands, Observer};

pub(super) struct ComposeSystems;

impl ComposeSystems {
    pub(super) fn observe(parent: &mut ChildSpawnerCommands) {
        parent.spawn(Observer::new(finish_player::on_finish_player));
        parent.spawn(Observer::new(populate_bio::on_populate_bio_ui));
        parent.spawn(Observer::new(populate_deck::on_populate_deck_ui));
        parent.spawn(Observer::new(commit_button::on_commit_button_ui));
    }
}
