use crate::message::CommandMessage;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};
use crate::data::game::TickType;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameTickMessage {
    pub tick: TickType,
}

impl CommandMessage for GameTickMessage {
    const COMMAND: op::Command = op::Command::GameTick;
}

impl Bufferable for GameTickMessage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.tick.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let tick = TickType::pull_from(buf);
        Self {
            tick,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.tick.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::message::game_tick::GameTickMessage;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

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
