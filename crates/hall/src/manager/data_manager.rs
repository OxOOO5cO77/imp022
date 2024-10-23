use std::io::{Error, ErrorKind};
use std::path::Path;

use rand::prelude::*;

use hall::data::hall::hall_build::HallBuild;
use hall::data::hall::hall_card::HallCard;
use hall::data::hall::hall_detail::HallDetail;
use hall::data::player::player_card::PlayerCard;
use shared_data::game::card::CardSlot;
use shared_data::player::build::{Build, ANT, BRD, CPU, DSC};
use shared_data::player::detail::{Detail, Distro, Institution, Location, Role};

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

    pub(crate) fn pick_detail(&self, rng: &mut impl Rng) -> [HallDetail; 4] {
        let detail = [
            self.detail.iter()
                .filter(|o| o.is(&Detail::Institution(Institution::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.detail.iter()
                .filter(|o| o.is(&Detail::Role(Role::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.detail.iter()
                .filter(|o| o.is(&Detail::Location(Location::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
            self.detail.iter()
                .filter(|o| o.is(&Detail::Distro(Distro::Any)))
                .choose(rng)
                .unwrap()
                .clone(),
        ];
        detail
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
