use std::collections::VecDeque;
use chrono::NaiveDate;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::data::attribute::Attributes;
use crate::data::build::BuildInstance;
use crate::data::category::CategoryInstance;
use crate::data::data_manager::DataManager;
use crate::data::player::Player;

#[derive(Clone)]
pub(crate) struct PlayerPart {
    seed: u64,
    pub(crate) values: [u8; 4],
    pub(crate) build: [BuildInstance; 4],
    pub(crate) category: [CategoryInstance; 4],
}

impl PlayerPart {
    pub(crate) fn new(dm: &DataManager, seed: u64) -> Option<Self> {
        let mut rng = StdRng::seed_from_u64(seed);
        Some(
            PlayerPart {
                seed,
                values: DataManager::pick_values(&mut rng),
                build: dm.pick_build(&mut rng)?,
                category: dm.pick_category(&mut rng)?,
            }
        )
    }
}

#[derive(Clone, Default)]
pub(crate) struct PlayerBuilder {
    pub(crate) access: Option<PlayerPart>,
    pub(crate) breach: Option<PlayerPart>,
    pub(crate) compute: Option<PlayerPart>,
    pub(crate) disrupt: Option<PlayerPart>,
    pub(crate) build: Option<PlayerPart>,
    pub(crate) build_values: Option<PlayerPart>,
    pub(crate) category: Option<PlayerPart>,
    pub(crate) category_values: Option<PlayerPart>,
}

impl PlayerBuilder {
    pub(crate) fn build(&self, dm: &DataManager) -> Option<Player> {
        let this = self.clone();
        let mut player = Player {
            attributes: Attributes::from_parts(&this.access?, &this.breach?, &this.compute?, &this.disrupt?),
            build: BuildInstance::from_parts(&this.build?, &this.build_values?),
            category: CategoryInstance::from_parts(&this.category?, &this.category_values?),
            deq: VecDeque::with_capacity(60),
            seed: self.generate_seed(),
            id: String::default(),
            name: String::default(),
            birthplace: (String::default(),String::default(),String::default()),
            dob: NaiveDate::MIN,
        };

        let mut rng = StdRng::seed_from_u64(player.seed);
        player.fill_deq(&mut rng, dm);

        let country = dm.pick_country(&mut rng);
        let bp = dm.pick_city(&country, &mut rng);
        player.birthplace = (bp.0, bp.1, country.clone());
        let gender = dm.pick_gender(&country, &mut rng);
        let names = dm.pick_name(&country, gender, &mut rng);
        player.name = format!("{} {}", names.0, names.1);

        player.id = DataManager::make_id(&mut rng);
        player.dob = dm.pick_dob(&mut rng);

        Some(player)
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
}

#[cfg(test)]
mod player_builder_test
{
    use crate::data::data_manager::DataManager;
    use crate::data::player_builder::{PlayerBuilder, PlayerPart};

    #[test]
    fn test_player_builder_empty() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let empty_builder = PlayerBuilder::default();
        assert!(empty_builder.build(&dm).is_none());
        Ok(())
    }

    fn parts(dm: &DataManager) -> [PlayerPart; 8] {
        core::array::from_fn(|i| PlayerPart::new(dm, 1234567890 * i as u64).unwrap())
    }

    #[test]
    fn test_player_builder_partial() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let parts = parts(&dm);

        let partial_builder = PlayerBuilder {
            access: Some(parts[0].clone()),
            breach: Some(parts[1].clone()),
            compute: Some(parts[2].clone()),
            disrupt: Some(parts[3].clone()),
            build: None,
            build_values: None,
            category: None,
            category_values: None,
        };
        assert!(partial_builder.build(&dm).is_none());

        Ok(())
    }

    #[test]
    fn test_player_builder_full() -> Result<(), std::io::Error> {
        let dm = DataManager::new()?;
        let parts = parts(&dm);

        let full_builder = PlayerBuilder {
            access: Some(parts[0].clone()),
            breach: Some(parts[1].clone()),
            compute: Some(parts[2].clone()),
            disrupt: Some(parts[3].clone()),
            build: Some(parts[4].clone()),
            build_values: Some(parts[5].clone()),
            category: Some(parts[6].clone()),
            category_values: Some(parts[7].clone()),
        };
        let player = full_builder.build(&dm);
        assert!(player.is_some());

        assert_eq!(player.unwrap().deq.len(), 40);

        Ok(())
    }
}
