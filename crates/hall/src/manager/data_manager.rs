use std::io::{Error, ErrorKind};
use std::path::Path;

use rand::prelude::*;

use hall::data::hall_build::HallBuild;
use hall::data::hall_card::HallCard;
use hall::data::hall_category::HallCategory;
use shared_data::player::build::{ANT, BRD, Build, CPU, DSC};
use shared_data::player::card::CardSlot;
use shared_data::player::category::{Category, Distro, Institution, Location, Role};

pub(crate) struct DataManager {
    build: Vec<HallBuild>,
    category: Vec<HallCategory>,
    card: Vec<HallCard>,
}

impl DataManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(DataManager {
            build: load_data_single("assets/data/hall_builds.ron")?,
            category: load_data_single("assets/data/hall_categories.ron")?,
            card: load_data_single("assets/data/hall_cards.ron")?,
        })
    }

    pub(crate) fn pick_build(&self, rng: &mut impl Rng) -> [HallBuild; 4] {
        let build = [
            self.build.iter()
                .filter(|o| o.is(&Build::ANT(ANT::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.build.iter()
                .filter(|o| o.is(&Build::BRD(BRD::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.build.iter()
                .filter(|o| o.is(&Build::CPU(CPU::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.build.iter()
                .filter(|o| o.is(&Build::DSC(DSC::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
        ];
        build
    }

    pub(crate) fn pick_category(&self, rng: &mut impl Rng) -> [HallCategory; 4] {
        let category = [
            self.category.iter()
                .filter(|o| o.is(&Category::Institution(Institution::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.category.iter()
                .filter(|o| o.is(&Category::Role(Role::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.category.iter()
                .filter(|o| o.is(&Category::Location(Location::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.category.iter()
                .filter(|o| o.is(&Category::Distro(Distro::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
        ];
        category
    }

    fn pick_card(&self, rng: &mut impl Rng, slot: &CardSlot) -> Option<HallCard> {
        self.card.iter()
            .filter(|o| o.matches(slot))
            .choose(rng)
            .cloned()
    }

    pub(crate) fn pick_cards(&self, rng: &mut impl Rng, from: &[CardSlot], count: u8) -> Vec<HallCard> {
        let slots = from
            .choose_multiple(rng, count as usize)
            .cloned()
            .collect::<Vec<_>>();
        slots.iter()
            .filter_map(|slot| self.pick_card(rng, slot))
            .collect()
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
