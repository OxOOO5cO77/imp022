use crate::core::{GameSubCommand, MissionNodeIntent};
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, GameIdType, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseIntentRequest {
    pub game_id: GameIdType,
    pub intent: MissionNodeIntent,
}

impl CommandMessage for GameChooseIntentRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::ChooseIntent as SubCommandType);
}

impl GameRequestMessage for GameChooseIntentRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseIntentResponse {
    pub success: bool,
}

impl CommandMessage for GameChooseIntentResponse {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::ChooseIntent as SubCommandType);
}

impl GameResponseMessage for GameChooseIntentResponse {}

#[cfg(test)]
mod test {
    use super::{GameChooseIntentRequest, GameChooseIntentResponse};
    use crate::core::{MissionNodeIntent, MissionNodeLinkDir};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameChooseIntentRequest {
            game_id: 1234567890,
            intent: MissionNodeIntent::Link(MissionNodeLinkDir::North),
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameChooseIntentRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameChooseIntentResponse {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameChooseIntentResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
