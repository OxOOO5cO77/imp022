use std::collections::VecDeque;
use std::iter::zip;

use hall::core::{AttributeArray, Attributes};
use hall::hall::{HallBuild, HallDetail};
use hall::player::{Player, PlayerBuild, PlayerCard, PlayerDetail, PlayerPart};
use hall::util;
use rand::prelude::*;
use shared_net::{PartType, SeedType};

use crate::private::manager::data_manager::DataManager;

#[derive(Clone)]
pub(crate) struct PlayerPartBuilder {
    seed: u64,
    pub(crate) values: AttributeArray,
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
            build: self.build.each_ref().map(|b| b.to_player(0)),
            detail: self.detail.each_ref().map(|d| d.to_player(0)),
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
        let deck = self.fill_deck(dm, &mut rng)?;

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
        core::array::from_fn(|i| build.build[i].to_player(values.values[i]))
    }

    pub(crate) fn detail_from_parts(detail: &PlayerPartBuilder, values: &PlayerPartBuilder) -> [PlayerDetail; 4] {
        core::array::from_fn(|i| detail.detail[i].to_player(values.values[i]))
    }

    #[rustfmt::skip]
    fn generate_seed(&self) -> SeedType {
          0x00000000000000FF & self.access.seed
        | 0x000000000000FF00 & self.breach.seed
        | 0x0000000000FF0000 & self.compute.seed
        | 0x00000000FF000000 & self.disrupt.seed
        | 0x000000FF00000000 & self.build.seed
        | 0x0000FF0000000000 & self.build_values.seed
        | 0x00FF000000000000 & self.detail.seed
        | 0xFF00000000000000 & self.detail_values.seed
    }

    fn fill_deck(&self, dm: &DataManager, rng: &mut impl Rng) -> Option<VecDeque<PlayerCard>> {
        let mut deck = VecDeque::new();

        let build_zip = zip(&self.build.build, self.build_values.values);
        let build_cards = build_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value)).map(|hall_card| PlayerCard::from(&hall_card));
        deck.extend(build_cards);

        let detail_zip = zip(&self.detail.detail, self.detail_values.values);
        let detail_cards = detail_zip.flat_map(|(item, value)| dm.pick_cards(rng, &item.cards, value)).map(|hall_card| PlayerCard::from(&hall_card));

        deck.extend(detail_cards);

        deck.make_contiguous().sort_by(|a, b| b.rarity.cmp(&a.rarity));

        Some(deck)
    }
}

#[cfg(test)]
mod player_builder_test {
    use crate::private::manager::data_manager::DataManager;
    use crate::private::manager::player_builder::{PlayerBuilder, PlayerPartBuilder};

    fn parts(dm: &DataManager) -> [PlayerPartBuilder; 8] {
        core::array::from_fn(|i| PlayerPartBuilder::new(1234567890 * i as u64, dm))
    }

    #[test]
    fn test_player_builder_full() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let [access, breach, compute, disrupt, build, build_values, detail, detail_values] = parts(&dm);

        let full_builder = PlayerBuilder {
            access,
            breach,
            compute,
            disrupt,
            build,
            build_values,
            detail,
            detail_values,
        };
        let player = full_builder.create_player(&dm);
        assert!(player.is_some());

        assert_eq!(player.unwrap().deck.len(), 40);

        Ok(())
    }
}
