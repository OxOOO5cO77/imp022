use crate::core::{AttributeKind, GameSubCommand};
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, GameIdType, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrRequest {
    pub game_id: GameIdType,
    pub attr: AttributeKind,
}

impl CommandMessage for GameChooseAttrRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::ChooseAttr as SubCommandType);
}

impl GameRequestMessage for GameChooseAttrRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrResponse {
    pub success: bool,
}

impl CommandMessage for GameChooseAttrResponse {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::ChooseAttr as SubCommandType);
}

impl GameResponseMessage for GameChooseAttrResponse {}

#[cfg(test)]
mod test {
    use crate::message::game_choose_attr::{AttributeKind, GameChooseAttrRequest, GameChooseAttrResponse};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameChooseAttrRequest {
            game_id: 1234567890,
            attr: AttributeKind::Compute,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameChooseAttrRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameChooseAttrResponse {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameChooseAttrResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
