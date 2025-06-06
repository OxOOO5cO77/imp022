use crate::core::{GameSubCommand, TickType};
use crate::message::CommandMessage;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameTickMessage {
    pub tick: TickType,
}

impl CommandMessage for GameTickMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Tick as SubCommandType);
}

#[cfg(test)]
mod test {
    use super::GameTickMessage;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameTickMessage {
            tick: 123,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameTickMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
