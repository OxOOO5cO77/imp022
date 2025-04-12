use crate::core::GameSubCommand;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, GameIdType, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndTurnRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameEndTurnRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::EndTurn as SubCommandType);
}

impl GameRequestMessage for GameEndTurnRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndTurnResponse {
    pub success: bool,
}

impl CommandMessage for GameEndTurnResponse {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::EndTurn as SubCommandType);
}

impl GameResponseMessage for GameEndTurnResponse {}

#[cfg(test)]
mod test {
    use super::{GameEndTurnRequest, GameEndTurnResponse};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameEndTurnRequest {
            game_id: 1234567890,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameEndTurnRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameEndTurnResponse {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameEndTurnResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
