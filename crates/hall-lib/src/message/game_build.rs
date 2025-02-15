use crate::core::GameSubCommand;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use crate::player::PlayerCard;
use shared_net::op::SubCommandType;
use shared_net::{op, Bufferable, GameIdType, PartType, SeedType, SizedBuffer, SizedBufferError};

type PartsArray = [PartType; 8];

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameBuildRequest {
    pub game_id: GameIdType,
    pub parts: PartsArray,
    pub commit: bool,
}

impl CommandMessage for GameBuildRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Build as SubCommandType);
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
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Build as SubCommandType);
}

impl GameResponseMessage for GameBuildResponse {}

#[cfg(test)]
mod test {
    use crate::core::CardNumberType;
    use crate::core::Rarity::Legendary;
    use crate::message::game_build::{GameBuildRequest, GameBuildResponse};
    use crate::player::PlayerCard;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    pub const CARD_COUNT: usize = 40;

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameBuildRequest {
            game_id: 1234567890,
            parts: [1234567890, 1234567891, 1234567892, 1234567893, 1234567894, 1234567895, 1234567896, 1234567897],
            commit: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameBuildRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
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

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameBuildResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig.seed, result.seed);
        assert_eq!(orig.deck.len(), result.deck.len());
        assert_eq!(orig.deck.iter().last().unwrap().number, result.deck.iter().last().unwrap().number);

        Ok(())
    }
}
