use shared_net::bufferable_derive::Bufferable;
use shared_net::{op, Bufferable, GameIdType, PartType, SeedType, VSizedBuffer};

use crate::data::player::PlayerCard;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};

type PartsArray = [PartType; 8];

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameBuildRequest {
    pub game_id: GameIdType,
    pub parts: PartsArray,
    pub commit: bool,
}

impl CommandMessage for GameBuildRequest {
    const COMMAND: op::Command = op::Command::GameBuild;
}

impl GameRequestMessage for GameBuildRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
pub struct GameBuildResponse {
    pub seed: SeedType,
    pub deck: Vec<PlayerCard>,
}

impl CommandMessage for GameBuildResponse {
    const COMMAND: op::Command = op::Command::GameBuild;
}

impl GameResponseMessage for GameBuildResponse {}

#[cfg(test)]
mod test {
    use crate::data::core::CardNumberType;
    use crate::data::core::Rarity::Legendary;
    use crate::data::player::PlayerCard;
    use crate::message::game_build::{GameBuildRequest, GameBuildResponse};
    use shared_net::{Bufferable, VSizedBuffer};

    pub const CARD_COUNT: usize = 40;

    #[test]
    fn test_request() {
        let orig = GameBuildRequest {
            game_id: 1234567890,
            parts: [1234567890, 1234567891, 1234567892, 1234567893, 1234567894, 1234567895, 1234567896, 1234567897],
            commit: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameBuildRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let mut orig = GameBuildResponse {
            seed: 1234567890,
            deck: Vec::default(),
        };
        for i in 0..CARD_COUNT {
            orig.deck.push(PlayerCard {
                rarity: Legendary,
                number: i as CardNumberType,
                set: 1,
            });
        }

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameBuildResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig.seed, result.seed);
        assert_eq!(orig.deck.len(), result.deck.len());
        assert_eq!(orig.deck.iter().last().unwrap().number, result.deck.iter().last().unwrap().number)
    }
}
