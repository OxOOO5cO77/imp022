use crate::core::GameSubCommand;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::op::SubCommandType;
use shared_net::{op, Bufferable, GameIdType, SizedBuffer, SizedBufferError};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndGameRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameEndGameRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::EndGame as SubCommandType);
}

impl GameRequestMessage for GameEndGameRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndGameResponse {
    pub success: bool,
}

impl CommandMessage for GameEndGameResponse {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::EndGame as SubCommandType);
}

impl GameResponseMessage for GameEndGameResponse {}

#[cfg(test)]
mod test {
    use crate::message::game_end_game::{GameEndGameRequest, GameEndGameResponse};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameEndGameRequest {
            game_id: 1234567890,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameEndGameRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameEndGameResponse {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameEndGameResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
