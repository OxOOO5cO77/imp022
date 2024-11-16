use std::collections::VecDeque;
use std::iter::zip;

use rand::prelude::*;

use crate::manager::data_manager::DataManager;
use hall::data::hall::hall_build::HallBuild;
use hall::data::hall::hall_card::HallCard;
use hall::data::hall::hall_detail::HallDetail;
use hall::data::player::player_build::PlayerBuild;
use hall::data::player::player_card::PlayerCard;
use hall::data::player::player_detail::PlayerDetail;
use hall::data::player::player_part::PlayerPart;
use hall::data::player::Player;
use hall::data::util;
use shared_data::player::attribute::{Attributes, AttributeValueType};
use shared_data::types::{PartType, SeedType};

#[derive(Clone)]
pub(crate) struct PlayerPartBuilder {
    seed: u64,
    pub(crate) values: [AttributeValueType; 4],
    pub(crate) build: [HallBuild; 4],
    pub(crate) detail: [HallDetail; 4],
}

impl PlayerPartBuilder {
    pub(crate) fn new(seed: u64, dm: &DataManager) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        PlayerPartBuilder {
            seed,
            values: util::pick_values(&mut rng),
            build: dm.pick_build(&mut rng),
            detail: dm.pick_detail(&mut rng),
        }
    }

    pub(crate) fn convert_to_player_part(&self) -> PlayerPart {
        PlayerPart {
            seed: self.seed,
            values: self.values,
            build: [self.build[0].to_player(&0), self.build[1].to_player(&0), self.build[2].to_player(&0), self.build[3].to_player(&0)],
            detail: [self.detail[0].to_player(&0), self.detail[1].to_player(&0), self.detail[2].to_player(&0), self.detail[3].to_player(&0)],
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
    pub(crate) detail: PlayerPartBuilder,
    pub(crate) detail_values: PlayerPartBuilder,
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
            detail: PlayerPartBuilder::new(seeds[6], dm),
            detail_values: PlayerPartBuilder::new(seeds[7], dm),
        }
    }
    pub(crate) fn create_player(&self, dm: &DataManager) -> Option<Player> {
        let seed = self.generate_seed();
        let mut rng = StdRng::seed_from_u64(seed);
        let deck = self.fill_deck(&mut rng, dm)?;

        let player = Player {
            seed,
            attributes: Attributes::from_arrays([self.access.values, self.breach.values, self.compute.values, self.disrupt.values]),
            build: Self::build_from_parts(&self.build, &self.build_values),
            detail: Self::detail_from_parts(&self.detail, &self.detail_values),
            deck,
        };
        Some(player)
    }

    pub(crate) fn build_from_parts(build: &PlayerPartBuilder, values: &PlayerPartBuilder) -> [PlayerBuild; 4] {
        [build.build[0].to_player(&values.values[0]), build.build[1].to_player(&values.values[1]), build.build[2].to_player(&values.values[2]), build.build[3].to_player(&values.values[3])]
    }

    pub(crate) fn detail_from_parts(detail: &PlayerPartBuilder, values: &PlayerPartBuilder) -> [PlayerDetail; 4] {
        [detail.detail[0].to_player(&values.values[0]), detail.detail[1].to_player(&values.values[1]), detail.detail[2].to_player(&values.values[2]), detail.detail[3].to_player(&values.values[3])]
    }

    fn generate_seed(&self) -> SeedType {
        0x00000000000000FF & self.access.seed | 0x000000000000FF00 & &self.breach.seed | 0x0000000000FF0000 & &self.compute.seed | 0x00000000FF000000 & &self.disrupt.seed | 0x000000FF00000000 & &self.build.seed | 0x0000FF0000000000 & &self.build_values.seed | 0x00FF000000000000 & &self.detail.seed | 0xFF00000000000000 & &self.detail_values.seed
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

        let detail_zip = zip(&self.detail.detail, self.detail_values.values);
        let detail_cards = detail_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value)).map(Self::map_card);

        deck.extend(detail_cards);

        deck.make_contiguous().sort_by(|a, b| b.rarity.cmp(&a.rarity));

        Some(deck)
    }
}

#[cfg(test)]
mod player_builder_test {
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
            detail: parts[6].clone(),
            detail_values: parts[7].clone(),
        };
        let player = full_builder.create_player(&dm);
        assert!(player.is_some());

        assert_eq!(player.unwrap().deck.len(), 40);

        Ok(())
    }
}
