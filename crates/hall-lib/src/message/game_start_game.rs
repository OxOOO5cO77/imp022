use crate::core::GameSubCommand;
use crate::message::CommandMessage;
use shared_net::op::SubCommandType;
use shared_net::{op, Bufferable, SizedBuffer, SizedBufferError};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartGameMessage {
    pub success: bool,
}

impl CommandMessage for GameStartGameMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::StartGame as SubCommandType);
}

#[cfg(test)]
mod test {
    use crate::message::game_start_game::GameStartGameMessage;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameStartGameMessage {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameStartGameMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
