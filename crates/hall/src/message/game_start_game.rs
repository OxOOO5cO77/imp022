use shared_net::{op, Bufferable, VSizedBuffer};

use crate::message::CommandMessage;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartGameMessage {
    pub success: bool,
}

impl CommandMessage for GameStartGameMessage {
    const COMMAND: op::Command = op::Command::GameStartGame;
}

#[cfg(test)]
mod test {
    use crate::message::game_start_game::GameStartGameMessage;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameStartGameMessage {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameStartGameMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
