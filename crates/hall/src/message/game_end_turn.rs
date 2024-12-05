use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::GameIdType;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndTurnRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameEndTurnRequest {
    const COMMAND: op::Command = op::Command::GameEndTurn;
}

impl GameRequestMessage for GameEndTurnRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

impl Bufferable for GameEndTurnRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        Self {
            game_id,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer()
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndTurnResponse {
    pub success: bool,
}

impl CommandMessage for GameEndTurnResponse {
    const COMMAND: op::Command = op::Command::GameEndTurn;
}

impl GameResponseMessage for GameEndTurnResponse {}

impl Bufferable for GameEndTurnResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.success.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let success = bool::pull_from(buf);
        Self {
            success,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.success.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::message::game_end_turn::{GameEndTurnRequest, GameEndTurnResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_request() {
        let orig = GameEndTurnRequest {
            game_id: 1234567890,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameEndTurnRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let orig = GameEndTurnResponse {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameEndTurnResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
