use crate::core::{ErgArray, GameSubCommand};
use crate::message::CommandMessage;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameRollMessage {
    pub roll: ErgArray,
}

impl CommandMessage for GameRollMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Roll as SubCommandType);
}

#[cfg(test)]
mod test {
    use super::GameRollMessage;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameRollMessage {
            roll: [1, 2, 3, 4],
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameRollMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
