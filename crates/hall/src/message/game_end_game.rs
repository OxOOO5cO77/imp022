use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::GameIdType;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameEndGameRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameEndGameRequest {
    const COMMAND: op::Command = op::Command::GameEndGame;
}

impl GameRequestMessage for GameEndGameRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

impl Bufferable for GameEndGameRequest {
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
pub struct GameEndGameResponse {
    pub success: bool,
}

impl CommandMessage for GameEndGameResponse {
    const COMMAND: op::Command = op::Command::GameEndGame;
}

impl GameResponseMessage for GameEndGameResponse {}

impl Bufferable for GameEndGameResponse {
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
    use crate::message::game_end_game::{GameEndGameRequest, GameEndGameResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_request() {
        let orig = GameEndGameRequest {
            game_id: 1234567890,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameEndGameRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let orig = GameEndGameResponse {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameEndGameResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
