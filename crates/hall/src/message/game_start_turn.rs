use shared_net::{op, Bufferable, GameIdType, VSizedBuffer};

use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartTurnRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameStartTurnRequest {
    const COMMAND: op::Command = op::Command::GameStartTurn;
}

impl GameRequestMessage for GameStartTurnRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartTurnResponse {
    pub success: bool,
}

impl CommandMessage for GameStartTurnResponse {
    const COMMAND: op::Command = op::Command::GameStartTurn;
}

impl GameResponseMessage for GameStartTurnResponse {}

#[cfg(test)]
mod test {
    use crate::message::game_start_turn::{GameStartTurnRequest, GameStartTurnResponse};
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_request() {
        let orig = GameStartTurnRequest {
            game_id: 1234567890,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameStartTurnRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let orig = GameStartTurnResponse {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameStartTurnResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
