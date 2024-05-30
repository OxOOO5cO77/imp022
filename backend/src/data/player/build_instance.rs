use std::mem::discriminant;

use imp022_shared::player::build::Build;
use imp022_shared::player::card::CardSlot;
use imp022_shared::player::PlayerBuild;
use serde::Deserialize;

use crate::data::player::player_builder::PlayerPartBuilder;

#[derive(Clone, Deserialize)]
pub(crate) struct BuildInstance {
    pub(crate) build: Build,
    pub(crate) title: String,
    pub(crate) cards: Vec<CardSlot>,
}

impl BuildInstance {
    pub(crate) fn is(&self, other: &Build) -> bool {
        discriminant(&self.build) == discriminant(other)
    }

    fn to_player(&self, value: &u8) -> PlayerBuild {
        PlayerBuild {
            build: self.build,
            title: self.title.clone(),
            value: *value,
        }
    }

    pub(crate) fn from_parts(build: &Option<PlayerPartBuilder>, values: &Option<PlayerPartBuilder>) -> Option<[PlayerBuild; 4]> {
        let build = build.as_ref()?;
        let values = values.as_ref()?;
        Some([
            build.build[0].to_player(&values.values[0]),
            build.build[1].to_player(&values.values[1]),
            build.build[2].to_player(&values.values[2]),
            build.build[3].to_player(&values.values[3]),
        ])
    }
}
