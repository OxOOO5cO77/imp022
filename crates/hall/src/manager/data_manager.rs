use std::io::{Error, ErrorKind};
use std::path::Path;

use hall::data::hall::{HallBuild, HallCard, HallDetail};
use hall::data::player::PlayerCard;
use rand::prelude::*;
use shared_data::game::card::CardSlot;
use shared_data::player::build::Build;
use shared_data::player::detail::Detail;

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
        let build = [self.build.iter().filter(|o| o.is(Build::ANT)).choose(rng).unwrap().clone(), self.build.iter().filter(|o| o.is(Build::BRD)).choose(rng).unwrap().clone(), self.build.iter().filter(|o| o.is(Build::CPU)).choose(rng).unwrap().clone(), self.build.iter().filter(|o| o.is(Build::DSK)).choose(rng).unwrap().clone()];
        build
    }

    pub(crate) fn pick_detail(&self, rng: &mut impl Rng) -> [HallDetail; 4] {
        let detail = [self.detail.iter().filter(|o| o.is(Detail::Institution)).choose(rng).unwrap().clone(), self.detail.iter().filter(|o| o.is(Detail::Role)).choose(rng).unwrap().clone(), self.detail.iter().filter(|o| o.is(Detail::Location)).choose(rng).unwrap().clone(), self.detail.iter().filter(|o| o.is(Detail::Distro)).choose(rng).unwrap().clone()];
        detail
    }

    fn pick_card(&self, rng: &mut impl Rng, slot: &CardSlot) -> Option<HallCard> {
        self.card.iter().filter(|o| o.matches(slot)).choose(rng).cloned()
    }

    pub(crate) fn pick_cards(&self, rng: &mut impl Rng, from: &[CardSlot], count: u8) -> Vec<HallCard> {
        let slots = from.choose_multiple(rng, count as usize).cloned().collect::<Vec<_>>();
        slots.iter().filter_map(|slot| self.pick_card(rng, slot)).collect()
    }

    pub(crate) fn lookup_player_card(&self, player_card: &PlayerCard) -> Option<HallCard> {
        self.card.iter().find(|o| o.set == player_card.set && o.rarity == player_card.rarity && o.number == player_card.number).cloned()
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
        assert!(!dm.detail.is_empty());
        Ok(())
    }
}
