use crate::message::CommandMessage;
use shared_net::{op, Bufferable, VSizedBuffer};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateMissionMessage {
    pub new: bool,
}

impl CommandMessage for GameUpdateMissionMessage {
    const COMMAND: op::Command = op::Command::GameUpdateMission;
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
    use crate::message::game_update_mission::GameUpdateMissionMessage;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameUpdateMissionMessage::new(true);

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameUpdateMissionMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
