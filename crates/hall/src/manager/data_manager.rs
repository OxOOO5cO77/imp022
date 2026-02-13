use std::io::Error;
use std::path::Path;

use hall_lib::core::{Build, CardSlot, Detail, GeneralType, SpecificType};
use hall_lib::hall::{HallBuild, HallCard, HallDetail};
use hall_lib::player::PlayerCard;
use rand::prelude::*;

pub(crate) struct DataManager {
    build: Vec<HallBuild>,
    detail: Vec<HallDetail>,
    card: Vec<HallCard>,
}

impl DataManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(DataManager {
            build: load_data_single("assets/data/hall_builds.ron")?,
            detail: load_data_single("assets/data/hall_details.ron")?,
            card: load_data_single("assets/data/hall_cards.ron")?,
        })
    }

    pub(crate) fn pick_build(&self, rng: &mut impl Rng) -> [HallBuild; 4] {
        [
            //
            self.build.iter().filter(|o| o.is(Build::ANT)).choose(rng).unwrap().clone(),
            self.build.iter().filter(|o| o.is(Build::BRD)).choose(rng).unwrap().clone(),
            self.build.iter().filter(|o| o.is(Build::CPU)).choose(rng).unwrap().clone(),
            self.build.iter().filter(|o| o.is(Build::DSK)).choose(rng).unwrap().clone(),
        ]
    }

    pub(crate) fn pick_detail(&self, rng: &mut impl Rng) -> [HallDetail; 4] {
        [
            //
            self.detail.iter().filter(|o| o.is(Detail::Institution)).choose(rng).unwrap().clone(),
            self.detail.iter().filter(|o| o.is(Detail::Role)).choose(rng).unwrap().clone(),
            self.detail.iter().filter(|o| o.is(Detail::Location)).choose(rng).unwrap().clone(),
            self.detail.iter().filter(|o| o.is(Detail::Distro)).choose(rng).unwrap().clone(),
        ]
    }

    fn pick_card(&self, rng: &mut impl Rng, slot: &CardSlot) -> Option<&HallCard> {
        self.card.iter().filter(|o| o.matches(slot)).choose(rng)
    }

    pub(crate) fn pick_cards(&self, rng: &mut impl Rng, from: &[CardSlot], count: u8) -> Vec<&HallCard> {
        let slots = from.sample(rng, count as usize).cloned().collect::<Vec<_>>();
        slots.iter().filter_map(|slot| self.pick_card(rng, slot)).collect()
    }

    pub(crate) fn lookup_player_card(&self, player_card: &PlayerCard) -> Option<&HallCard> {
        self.card.iter().find(|o| o.set == player_card.set && o.rarity == player_card.rarity && o.number == player_card.number)
    }

    pub(crate) fn pick_institution(&self, rng: &mut impl Rng) -> Option<(GeneralType, SpecificType)> {
        self.detail.iter().filter(|detail| matches!(&detail.detail, Detail::Institution(_, _))).choose(rng).and_then(|detail| match detail.detail {
            Detail::Institution(general, specific) => Some((general, specific)),
            _ => None,
        })
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
