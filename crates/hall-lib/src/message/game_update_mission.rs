use crate::core::GameSubCommand;
use crate::message::CommandMessage;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateMissionMessage {
    pub new: bool,
}

impl CommandMessage for GameUpdateMissionMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::UpdateMission as SubCommandType);
}

impl GameUpdateMissionMessage {
    pub fn new(new: bool) -> Self {
        Self {
            new,
        }
    }
}

#[cfg(test)]
mod test {
    use super::GameUpdateMissionMessage;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameUpdateMissionMessage::new(true);

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameUpdateMissionMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
