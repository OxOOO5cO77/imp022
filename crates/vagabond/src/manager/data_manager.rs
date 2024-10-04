use std::collections::VecDeque;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::Resource;

use hall::data::player::player_build::PlayerBuild;
use hall::data::player::player_card::PlayerCard;
use hall::data::player::player_category::PlayerCategory;
use hall::data::player::player_part::PlayerPart;
use vagabond::data::vagabond_build::VagabondBuild;
use vagabond::data::vagabond_card::VagabondCard;
use vagabond::data::vagabond_category::VagabondCategory;
use vagabond::data::vagabond_part::VagabondPart;

#[derive(Resource)]
pub(crate) struct DataManager {
    build: Vec<VagabondBuild>,
    category: Vec<VagabondCategory>,
    card: Vec<VagabondCard>,
}

impl DataManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(DataManager {
            build: load_data_single("assets/data/vagabond_builds.ron")?,
            category: load_data_single("assets/data/vagabond_categories.ron")?,
            card: load_data_single("assets/data/vagabond_cards.ron")?,
        })
    }

    fn convert_card(&self, in_card: &PlayerCard) -> Option<VagabondCard> {
        self.card.iter().find(|card| card.set == in_card.set && card.rarity == in_card.rarity && card.number == in_card.number).cloned()
    }

    pub(crate) fn convert_deck(&self, in_deck: VecDeque<PlayerCard>) -> VecDeque<VagabondCard> {
        in_deck.iter().filter_map(|o| self.convert_card(o)).collect::<VecDeque<_>>()
    }

    fn convert_build(&self, in_build: &PlayerBuild) -> Option<VagabondBuild> {
        self.build.iter().find(|build| build.build == in_build.build && build.number == in_build.number).cloned()
    }

    fn convert_category(&self, in_build: &PlayerCategory) -> Option<VagabondCategory> {
        self.category.iter().find(|category| category.category == in_build.category && category.number == in_build.number).cloned()
    }

    pub(crate) fn convert_part(&self, in_part: &PlayerPart) -> Option<VagabondPart> {
        let part = VagabondPart {
            seed: in_part.seed,
            values: in_part.values,
            build: [
                self.convert_build(&in_part.build[0])?,
                self.convert_build(&in_part.build[1])?,
                self.convert_build(&in_part.build[2])?,
                self.convert_build(&in_part.build[3])?,
            ],
            category: [
                self.convert_category(&in_part.category[0])?,
                self.convert_category(&in_part.category[1])?,
                self.convert_category(&in_part.category[2])?,
                self.convert_category(&in_part.category[3])?,
            ],
        };
        Some(part)
    }
}

fn load_data_single<T, P>(source_file: P) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let ron = std::fs::read_to_string(source_file)?;
    let parsed = ron::from_str::<T>(&ron).map_err(|o| Error::new(ErrorKind::Other, o))?;
    Ok(parsed)
}

#[cfg(test)]
mod data_manager_test {
    use crate::manager::data_manager::DataManager;

    #[test]
    fn test_load_data() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        assert!(!dm.card.is_empty());
        assert!(!dm.build.is_empty());
        assert!(!dm.category.is_empty());
        Ok(())
    }
}
