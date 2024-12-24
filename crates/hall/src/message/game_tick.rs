use shared_net::{op, Bufferable, VSizedBuffer};

use crate::data::game::TickType;
use crate::message::CommandMessage;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameTickMessage {
    pub tick: TickType,
}

impl CommandMessage for GameTickMessage {
    const COMMAND: op::Command = op::Command::GameTick;
}

#[cfg(test)]
mod test {
    use crate::message::game_tick::GameTickMessage;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameTickMessage {
            tick: 123,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameTickMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
