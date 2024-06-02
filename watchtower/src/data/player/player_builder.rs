use std::collections::VecDeque;
use std::iter::zip;

use shared_data::player::attribute::Attributes;
use shared_data::player::card::Card;
use shared_data::player::Player;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::data::data_manager::DataManager;
use crate::data::player::build_instance::BuildInstance;
use crate::data::player::category_instance::CategoryInstance;

#[derive(Clone)]
pub(crate) struct PlayerPartBuilder {
    seed: u64,
    pub(crate) values: [u8; 4],
    pub(crate) build: [BuildInstance; 4],
    pub(crate) category: [CategoryInstance; 4],
}

impl PlayerPartBuilder {
    pub(crate) fn new(dm: &DataManager, seed: u64) -> Option<Self> {
        let mut rng = StdRng::seed_from_u64(seed);
        Some(PlayerPartBuilder { seed, values: DataManager::pick_values(&mut rng), build: dm.pick_build(&mut rng)?, category: dm.pick_category(&mut rng)? })
    }
}

#[derive(Clone, Default)]
pub(crate) struct PlayerBuilder {
    pub(crate) access: Option<PlayerPartBuilder>,
    pub(crate) breach: Option<PlayerPartBuilder>,
    pub(crate) compute: Option<PlayerPartBuilder>,
    pub(crate) disrupt: Option<PlayerPartBuilder>,
    pub(crate) build: Option<PlayerPartBuilder>,
    pub(crate) build_values: Option<PlayerPartBuilder>,
    pub(crate) category: Option<PlayerPartBuilder>,
    pub(crate) category_values: Option<PlayerPartBuilder>,
}

impl PlayerBuilder {
    pub(crate) fn build_player(&self, dm: &DataManager) -> Option<Player> {
        let seed = self.generate_seed();

        let mut rng = StdRng::seed_from_u64(seed);
        let deck = self.fill_deck(&mut rng, dm)?;

        let country = dm.pick_country(&mut rng);
        let bp = dm.pick_city(&country, &mut rng);
        let birthplace = (bp.0, bp.1, country.clone());
        let gender = dm.pick_gender(&country, &mut rng);
        let names = dm.pick_name(&country, gender, &mut rng);
        let name = format!("{} {}", names.0, names.1);

        let id = DataManager::make_id(&mut rng);
        let dob = dm.pick_dob(&mut rng);

        Some(
            Player {
                seed,
                attributes: Attributes::from_values(
                    &self.access.as_ref().map(|o| o.values),
                    &self.breach.as_ref().map(|o| o.values),
                    &self.compute.as_ref().map(|o| o.values),
                    &self.disrupt.as_ref().map(|o| o.values),
                )?,
                build: BuildInstance::from_parts(&self.build, &self.build_values)?,
                category: CategoryInstance::from_parts(&self.category, &self.category_values)?,
                deck,
                id,
                name,
                birthplace,
                dob,
            }
        )
    }

    fn generate_seed(&self) -> u64 {
        0x00000000000000FF & &self.access.clone().map(|o| o.seed).unwrap_or(0)
            | 0x000000000000FF00 & &self.breach.clone().map(|o| o.seed).unwrap_or(0)
            | 0x0000000000FF0000 & &self.compute.clone().map(|o| o.seed).unwrap_or(0)
            | 0x00000000FF000000 & &self.disrupt.clone().map(|o| o.seed).unwrap_or(0)
            | 0x000000FF00000000 & &self.build.clone().map(|o| o.seed).unwrap_or(0)
            | 0x0000FF0000000000 & &self.build_values.clone().map(|o| o.seed).unwrap_or(0)
            | 0x00FF000000000000 & &self.category.clone().map(|o| o.seed).unwrap_or(0)
            | 0xFF00000000000000 & &self.category_values.clone().map(|o| o.seed).unwrap_or(0)
    }

    fn fill_deck(&self, rng: &mut impl Rng, dm: &DataManager) -> Option<VecDeque<Card>> {
        let mut deck = VecDeque::new();

        let build_zip = zip(&self.build.as_ref()?.build, self.build_values.as_ref()?.values);
        let build_cards = build_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value));
        deck.extend(build_cards);

        let category_zip = zip(&self.category.as_ref()?.category, self.category_values.as_ref()?.values);
        let category_cards = category_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value));
        deck.extend(category_cards);

        deck.make_contiguous().sort_by(|a, b| b.id.1.cmp(&a.id.1));  // sort by rarity

        Some(deck)
    }
}

#[cfg(test)]
mod player_builder_test {
    use crate::data::data_manager::DataManager;
    use crate::data::player::player_builder::{PlayerBuilder, PlayerPartBuilder};

    #[test]
    fn test_player_builder_empty() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let empty_builder = PlayerBuilder::default();
        assert!(empty_builder.build_player(&dm).is_none());
        Ok(())
    }

    fn parts(dm: &DataManager) -> [PlayerPartBuilder; 8] {
        core::array::from_fn(|i| PlayerPartBuilder::new(dm, 1234567890 * i as u64).unwrap())
    }

    #[test]
    fn test_player_builder_partial() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let parts = parts(&dm);

        let partial_builder = PlayerBuilder { access: Some(parts[0].clone()), breach: Some(parts[1].clone()), compute: Some(parts[2].clone()), disrupt: Some(parts[3].clone()), build: None, build_values: None, category: None, category_values: None };
        assert!(partial_builder.build_player(&dm).is_none());

        Ok(())
    }

    #[test]
    fn test_player_builder_full() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let parts = parts(&dm);

        let full_builder = PlayerBuilder { access: Some(parts[0].clone()), breach: Some(parts[1].clone()), compute: Some(parts[2].clone()), disrupt: Some(parts[3].clone()), build: Some(parts[4].clone()), build_values: Some(parts[5].clone()), category: Some(parts[6].clone()), category_values: Some(parts[7].clone()) };
        let player = full_builder.build_player(&dm);
        assert!(player.is_some());

        assert_eq!(player.unwrap().deck.len(), 40);

        Ok(())
    }
}
