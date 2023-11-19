use std::io::{Error, ErrorKind};
use std::path::Path;
use bevy::prelude::Resource;
use chrono::NaiveDate;
use rand::distributions::Uniform;
use rand::prelude::*;
use walkdir::WalkDir;
use crate::data::build::{ANT, BRD, Build, BuildInstance, CPU, DSC};
use crate::data::card::{Card, CardSlot};
use crate::data::category::{Category, CategoryInstance, Institution, Location, Distro, Role};
use crate::data::bio::BioManager;

#[derive(Resource)]
pub(crate) struct DataManager {
    build: Vec<BuildInstance>,
    category: Vec<CategoryInstance>,
    card: Vec<Card>,
    bio: BioManager,
}

impl DataManager {
    pub(crate) fn new() -> Result<Self, Error> {
        Ok(
            DataManager {
                build: load_data("assets/data/build")?,
                category: load_data("assets/data/category")?,
                card: load_data("assets/data/card")?,
                bio: BioManager {
                    country: load_data_single("assets/data/bio/country.ron")?,
                    city: load_data_single("assets/data/bio/city.ron")?,
                    name: load_data_single("assets/data/bio/name.ron")?,
                },
            }
        )
    }

    pub(crate) fn pick_values(rng: &mut impl Rng) -> [u8; 4] {
        let v1 = Uniform::new_inclusive(1, 9).sample(rng);
        let v2 = Uniform::new_inclusive(1, 9).sample(rng);
        let remain = 20 - v1 - v2;
        let v3_lower = remain.max(10) - 9;
        let v3_upper = (remain - 1).min(9);
        let v3 = Uniform::new_inclusive(v3_lower, v3_upper).sample(rng);
        let v4 = remain - v3;

        [v1, v2, v3, v4]
    }

    pub(crate) fn pick_build(&self, rng: &mut impl Rng) -> Option<[BuildInstance; 4]> {
        Some([
            self.build.iter().filter(|o| o.is(&Build::ANT(ANT::Any))).choose(rng)?.clone(),
            self.build.iter().filter(|o| o.is(&Build::BRD(BRD::Any))).choose(rng)?.clone(),
            self.build.iter().filter(|o| o.is(&Build::CPU(CPU::Any))).choose(rng)?.clone(),
            self.build.iter().filter(|o| o.is(&Build::DSC(DSC::Any))).choose(rng)?.clone(),
        ])
    }

    pub(crate) fn pick_category(&self, rng: &mut impl Rng) -> Option<[CategoryInstance; 4]> {
        Some([
            self.category.iter().filter(|o| o.is(&Category::Institution(Institution::Any))).choose(rng)?.clone(),
            self.category.iter().filter(|o| o.is(&Category::Role(Role::Any))).choose(rng)?.clone(),
            self.category.iter().filter(|o| o.is(&Category::Location(Location::Any))).choose(rng)?.clone(),
            self.category.iter().filter(|o| o.is(&Category::Distro(Distro::Any))).choose(rng)?.clone(),
        ])
    }

    fn pick_card(&self, rng: &mut impl Rng, slot: &CardSlot) -> Option<Card> {
        self.card.iter().filter(|o| o.matches(slot)).choose(rng).cloned()
    }

    pub(crate) fn pick_cards(&self, rng: &mut impl Rng, from: &[CardSlot], count: u8) -> Vec<Card> {
        let slots = from.choose_multiple(rng, count as usize).cloned().collect::<Vec<_>>();
        slots.iter().filter_map(|slot| self.pick_card(rng, slot)).collect()
    }

    pub(crate) fn pick_country(&self, rng: &mut impl Rng) -> String {
        self.bio.country(rng)
    }
    pub(crate) fn pick_city(&self, country: &String, rng: &mut impl Rng) -> (String,String) {
        self.bio.city(country, rng)
    }
    pub(crate) fn pick_gender(&self, country: &String, rng: &mut impl Rng) -> char {
        self.bio.gender(country, rng)
    }
    pub(crate) fn pick_name(&self, country: &String, gender: char, rng: &mut impl Rng) -> (String,String) {
        self.bio.name(country, gender, rng)
    }
    pub(crate) fn pick_dob(&self, rng: &mut impl Rng) -> NaiveDate {
        self.bio.dob(rng)
    }

    pub (crate) fn make_id(rng: &mut impl Rng) -> String {
        format!("{:016X}", rng.next_u64())
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("-")
    }
}

fn load_data_single<T: serde::de::DeserializeOwned, P: AsRef<Path>>(source_file: P) -> Result<T, Error> {
    let ron = std::fs::read_to_string(source_file)?;
    let parsed = ron::from_str::<T>(&ron).map_err(|o| Error::new(ErrorKind::Other, o))?;
    Ok(parsed)
}

fn load_data<T: serde::de::DeserializeOwned, P: AsRef<Path>>(source_dir: P) -> Result<Vec<T>, Error> {
    let mut result = Vec::new();
    for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()).filter(|e| !e.file_type().is_dir()) {
        result.push(load_data_single(entry.path())?);
    }

    Ok(result)
}


#[cfg(test)]
mod data_manager_test {
    use rand::prelude::StdRng;
    use rand::SeedableRng;
    use crate::data::data_manager::DataManager;

    #[test]
    fn test_load_data() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        assert!(!dm.card.is_empty());
        assert!(!dm.build.is_empty());
        assert!(!dm.category.is_empty());
        Ok(())
    }

    #[test]
    fn test_pick_values() {
        let mut rng = StdRng::seed_from_u64(0x1234567890ABCDEF);

        let values = DataManager::pick_values(&mut rng);
        assert_eq!(values.iter().sum::<u8>(), 20);
    }
}


