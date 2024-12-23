use shared_net::bufferable_derive::Bufferable;
use shared_net::{op, Bufferable, VSizedBuffer};

use crate::data::core::ErgType;
use crate::message::CommandMessage;

type RollArray = [ErgType; 4];

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameRollMessage {
    pub roll: RollArray,
}

impl CommandMessage for GameRollMessage {
    const COMMAND: op::Command = op::Command::GameRoll;
}

#[cfg(test)]
mod test {
    use crate::message::game_roll::GameRollMessage;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameRollMessage {
            roll: [1, 2, 3, 4],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameRollMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
