use std::collections::VecDeque;
use std::iter::zip;

use rand::distributions::Uniform;
use rand::prelude::*;

use hall::data::hall_build::HallBuild;
use hall::data::hall_card::HallCard;
use hall::data::hall_category::HallCategory;
use hall::data::player::Player;
use hall::data::player_build::PlayerBuild;
use hall::data::player_card::PlayerCard;
use hall::data::player_category::PlayerCategory;
use hall::data::player_part::PlayerPart;
use shared_data::player::attribute::Attributes;
use shared_data::types::{PartType, SeedType};

use crate::manager::data_manager::DataManager;

#[derive(Clone)]
pub(crate) struct PlayerPartBuilder {
    seed: u64,
    pub(crate) values: [u8; 4],
    pub(crate) build: [HallBuild; 4],
    pub(crate) category: [HallCategory; 4],
}

impl PlayerPartBuilder {
    fn pick_values(rng: &mut impl Rng) -> [u8; 4] {
        let v1 = Uniform::new_inclusive(1, 9).unwrap().sample(rng);
        let v2 = Uniform::new_inclusive(1, 9).unwrap().sample(rng);
        let remain = 20 - v1 - v2;
        let v3_lower = remain.max(10) - 9;
        let v3_upper = (remain - 1).min(9);
        let v3 = Uniform::new_inclusive(v3_lower, v3_upper).unwrap().sample(rng);
        let v4 = remain - v3;

        [v1, v2, v3, v4]
    }

    pub(crate) fn new(seed: u64, dm: &DataManager) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        PlayerPartBuilder {
            seed,
            values: Self::pick_values(&mut rng),
            build: dm.pick_build(&mut rng),
            category: dm.pick_category(&mut rng),
        }
    }

    pub(crate) fn convert_to_player_part(&self) -> PlayerPart {
        PlayerPart {
            seed: self.seed,
            values: self.values,
            build: [
            self.build[0].to_player(&0),
                self.build[1].to_player(&0),
                self.build[2].to_player(&0),
                self.build[3].to_player(&0),
            ],
            category: [
                self.category[0].to_player(&0),
                self.category[1].to_player(&0),
                self.category[2].to_player(&0),
                self.category[3].to_player(&0),
            ],
        }
    }
}

pub(crate) struct PlayerBuilder {
    pub(crate) access: PlayerPartBuilder,
    pub(crate) breach: PlayerPartBuilder,
    pub(crate) compute: PlayerPartBuilder,
    pub(crate) disrupt: PlayerPartBuilder,
    pub(crate) build: PlayerPartBuilder,
    pub(crate) build_values: PlayerPartBuilder,
    pub(crate) category: PlayerPartBuilder,
    pub(crate) category_values: PlayerPartBuilder,
}

impl PlayerBuilder {
    pub(crate) fn new(seeds: &[PartType], dm: &DataManager) -> Self {
        Self {
            access: PlayerPartBuilder::new(seeds[0], dm),
            breach: PlayerPartBuilder::new(seeds[1], dm),
            compute: PlayerPartBuilder::new(seeds[2], dm),
            disrupt: PlayerPartBuilder::new(seeds[3], dm),
            build: PlayerPartBuilder::new(seeds[4], dm),
            build_values: PlayerPartBuilder::new(seeds[5], dm),
            category: PlayerPartBuilder::new(seeds[6], dm),
            category_values: PlayerPartBuilder::new(seeds[7], dm),
        }
    }
    pub(crate) fn create_player(&self, dm: &DataManager) -> Option<Player> {
        let seed = self.generate_seed();
        let mut rng = StdRng::seed_from_u64(seed);
        let deck = self.fill_deck(&mut rng, dm)?;

        let player = Player {
            seed,
            attributes: Attributes::from_values(
                &self.access.values,
                &self.breach.values,
                &self.compute.values,
                &self.disrupt.values,
            ),
            build: Self::build_from_parts(&self.build, &self.build_values),
            category: Self::category_from_parts(&self.category, &self.category_values),
            deck,
        };
        Some(player)
    }

    pub(crate) fn build_from_parts(build: &PlayerPartBuilder, values: &PlayerPartBuilder) -> [PlayerBuild; 4] {
        [
            build.build[0].to_player(&values.values[0]),
            build.build[1].to_player(&values.values[1]),
            build.build[2].to_player(&values.values[2]),
            build.build[3].to_player(&values.values[3]),
        ]
    }

    pub(crate) fn category_from_parts(category: &PlayerPartBuilder, values: &PlayerPartBuilder) -> [PlayerCategory; 4] {
        [
            category.category[0].to_player(&values.values[0]),
            category.category[1].to_player(&values.values[1]),
            category.category[2].to_player(&values.values[2]),
            category.category[3].to_player(&values.values[3]),
        ]
    }


    fn generate_seed(&self) -> SeedType {
        0x00000000000000FF & self.access.seed
            | 0x000000000000FF00 & &self.breach.seed
            | 0x0000000000FF0000 & &self.compute.seed
            | 0x00000000FF000000 & &self.disrupt.seed
            | 0x000000FF00000000 & &self.build.seed
            | 0x0000FF0000000000 & &self.build_values.seed
            | 0x00FF000000000000 & &self.category.seed
            | 0xFF00000000000000 & &self.category_values.seed
    }

    fn map_card(card: HallCard) -> PlayerCard {
        PlayerCard {
            rarity: card.rarity,
            number: card.number,
            set: card.set,
        }
    }

    fn fill_deck(&self, rng: &mut impl Rng, dm: &DataManager) -> Option<VecDeque<PlayerCard>> {
        let mut deck = VecDeque::new();

        let build_zip = zip(&self.build.build, self.build_values.values);
        let build_cards = build_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value)).map(Self::map_card);
        deck.extend(build_cards);

        let category_zip = zip(&self.category.category, self.category_values.values);
        let category_cards = category_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value)).map(Self::map_card);

        deck.extend(category_cards);

        deck.make_contiguous().sort_by(|a, b| b.rarity.cmp(&a.rarity));

        Some(deck)
    }
}

#[cfg(test)]
mod player_builder_test {
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    use crate::manager::data_manager::DataManager;
    use crate::manager::player_builder::{PlayerBuilder, PlayerPartBuilder};

    fn parts(dm: &DataManager) -> [PlayerPartBuilder; 8] {
        core::array::from_fn(|i| PlayerPartBuilder::new(1234567890 * i as u64, dm))
    }

    #[test]
    fn test_player_builder_full() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let parts = parts(&dm);

        let full_builder = PlayerBuilder {
            access: parts[0].clone(),
            breach: parts[1].clone(),
            compute: parts[2].clone(),
            disrupt: parts[3].clone(),
            build: parts[4].clone(),
            build_values: parts[5].clone(),
            category: parts[6].clone(),
            category_values: parts[7].clone(),
        };
        let player = full_builder.create_player(&dm);
        assert!(player.is_some());

        assert_eq!(player.unwrap().deck.len(), 40);

        Ok(())
    }

    #[test]
    fn test_pick_values() -> Result<(), String> {
        let mut rng = StdRng::seed_from_u64(0x1234567890ABCDEF);

        let values = PlayerPartBuilder::pick_values(&mut rng);
        assert_eq!(values.iter().sum::<u8>(), 20);
        Ok(())
    }
}
