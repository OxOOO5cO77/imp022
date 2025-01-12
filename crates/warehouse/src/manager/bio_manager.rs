use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::path::Path;

use chrono::NaiveDate;
use rand::prelude::{IndexedRandom, StdRng};
use rand::{Rng, SeedableRng};
use serde::Deserialize;
use warehouse::data::geo_location::GeoLocation;
use warehouse::data::player_bio::PlayerBio;

#[derive(Default, Deserialize)]
pub struct City(String, String, usize);

#[derive(Default, Deserialize)]
struct Name {
    given: Vec<(String, usize)>,
    surname: Vec<(String, usize)>,
}

#[derive(Default, Deserialize)]
pub struct ByGender {
    gender_weight: Vec<(char, usize)>,
    gender: HashMap<char, Name>,
}

#[derive(Default, Deserialize)]
pub struct BioManager {
    pub country: Vec<(String, usize)>,
    pub city: HashMap<String, Vec<City>>,
    pub name: HashMap<String, ByGender>,
}

fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl BioManager {
    pub fn new() -> Result<Self, Error> {
        Ok(BioManager {
            country: load_data_single("assets/data/bio/country.ron")?,
            city: load_data_single("assets/data/bio/city.ron")?,
            name: load_data_single("assets/data/bio/name.ron")?,
        })
    }

    fn country(&self, rng: &mut impl Rng) -> String {
        self.country.choose_weighted(rng, |o| o.1).map_or("", |o| &o.0).to_owned()
    }

    fn city(&self, country: &String, rng: &mut impl Rng) -> (String, String) {
        let unknown = ("Unknown", "UN");
        let result = self.city.get(country).map_or(unknown, |o| o.choose_weighted(rng, |o| o.2).map_or(unknown, |o| (&o.0, &o.1)));
        (result.0.to_owned(), result.1.to_owned())
    }

    fn gender(&self, country: &String, rng: &mut impl Rng) -> char {
        self.name.get(country).map_or('N', |o| o.gender_weight.choose_weighted(rng, |o| o.1).map_or('N', |o| o.0))
    }

    fn dob(&self, rng: &mut impl Rng) -> NaiveDate {
        let year = 2049 - rng.random_range(16..50);
        let day_max = if is_leap_year(year) {
            366
        } else {
            365
        };
        NaiveDate::from_yo_opt(year, rng.random_range(1..=day_max)).unwrap_or(NaiveDate::MIN)
    }

    fn name(&self, country: &String, gender: char, rng: &mut impl Rng) -> (String, String) {
        let default_gender = ByGender::default();
        let default_name = Name::default();
        let by_gender = self.name.get(country).unwrap_or(&default_gender);
        let name = by_gender.gender.get(&gender).unwrap_or(&default_name);
        let given = name.given.choose_weighted(rng, |o| o.1).map_or("First", |o| &o.0);
        let surname = name.surname.choose_weighted(rng, |o| o.1).map_or("Last", |o| &o.0);
        (given.to_owned(), surname.to_owned())
    }

    fn make_id(rng: &mut impl Rng) -> String {
        format!("{:016X}", rng.next_u64()).chars().collect::<Vec<char>>().chunks(4).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join("-")
    }

    pub(crate) fn generate_player_bio(&self, seed: u64) -> Option<PlayerBio> {
        let mut rng = StdRng::seed_from_u64(seed);

        let country = self.country(&mut rng);
        let bp = self.city(&country, &mut rng);
        let birthplace = (bp.0, bp.1, country.clone());
        let gender = self.gender(&country, &mut rng);
        let names = self.name(&country, gender, &mut rng);
        let name = format!("{} {}", names.0, names.1);

        let id = Self::make_id(&mut rng);
        let dob = self.dob(&mut rng);

        Some(PlayerBio {
            seed,
            id,
            name,
            birthplace,
            dob,
        })
    }

    pub(crate) fn generate_geo_location(&self, seed: u64) -> Option<GeoLocation> {
        let mut rng = StdRng::seed_from_u64(seed);

        let country = self.country(&mut rng);
        let city = self.city(&country, &mut rng);
        let location_parts = (city.0, city.1, country.clone());

        Some(GeoLocation {
            seed,
            location_part: location_parts,
        })
    }
}

fn load_data_single<T, P>(source_file: P) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let ron = std::fs::read_to_string(source_file)?;
    ron::from_str::<T>(&ron).map_err(|o| Error::new(ErrorKind::Other, o))
}

#[cfg(test)]
mod bio_manager_test {
    use crate::manager::bio_manager::BioManager;

    #[test]
    fn test_load_data() -> Result<(), std::io::Error> {
        let bm = BioManager::new()?;
        assert!(!bm.city.is_empty());
        assert!(!bm.country.is_empty());
        assert!(!bm.name.is_empty());
        Ok(())
    }
}
