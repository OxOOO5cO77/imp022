use std::io::Error;
use std::path::Path;

use bevy::prelude::Resource;

use hall_lib::core::{Detail, GeneralType, SpecificType};
use hall_lib::player::{PlayerBuild, PlayerCard, PlayerDetail, PlayerPart};
use vagabond_lib::data::{VagabondBuild, VagabondCard, VagabondDetail, VagabondPart};

#[derive(Resource)]
pub(crate) struct DataManager {
    build: Vec<VagabondBuild>,
    detail: Vec<VagabondDetail>,
    card: Vec<VagabondCard>,
}

impl DataManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(DataManager {
            build: load_data_single("assets/data/vagabond_builds.ron")?,
            detail: load_data_single("assets/data/vagabond_details.ron")?,
            card: load_data_single("assets/data/vagabond_cards.ron")?,
        })
    }

    pub(crate) fn convert_card(&self, in_card: &PlayerCard) -> Option<VagabondCard> {
        self.card.iter().find(|card| card.set == in_card.set && card.rarity == in_card.rarity && card.number == in_card.number).cloned()
    }

    pub(crate) fn convert_deck(&self, in_deck: Vec<PlayerCard>) -> Vec<VagabondCard> {
        in_deck.iter().filter_map(|o| self.convert_card(o)).collect::<Vec<_>>()
    }

    fn convert_build(&self, in_build: &PlayerBuild) -> Option<VagabondBuild> {
        self.build.iter().find(|build| build.build == in_build.build && build.number == in_build.number).cloned()
    }

    fn convert_detail(&self, in_build: &PlayerDetail) -> Option<VagabondDetail> {
        self.detail.iter().find(|detail| detail.detail == in_build.detail && detail.number == in_build.number).cloned()
    }

    pub(crate) fn convert_institution(&self, (general, specific): (GeneralType, SpecificType)) -> Option<&VagabondDetail> {
        self.detail.iter().find(|detail| detail.detail == Detail::Institution(general, specific))
    }

    pub(crate) fn convert_part(&self, in_part: &PlayerPart) -> Option<VagabondPart> {
        let part = VagabondPart {
            seed: in_part.seed,
            values: in_part.values,
            build: [
                //
                self.convert_build(&in_part.build[0])?,
                self.convert_build(&in_part.build[1])?,
                self.convert_build(&in_part.build[2])?,
                self.convert_build(&in_part.build[3])?,
            ],
            detail: [
                //
                self.convert_detail(&in_part.detail[0])?,
                self.convert_detail(&in_part.detail[1])?,
                self.convert_detail(&in_part.detail[2])?,
                self.convert_detail(&in_part.detail[3])?,
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
    ron::from_str::<T>(&ron).map_err(Error::other)
}

#[cfg(test)]
mod data_manager_test {
    use crate::manager::data_manager::DataManager;

    #[test]
    fn test_load_data() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        assert!(!dm.card.is_empty());
        assert!(!dm.build.is_empty());
        assert!(!dm.detail.is_empty());
        Ok(())
    }
}
