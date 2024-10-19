use shared_data::types::GameIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

type CardIdxType = u8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartTurnRequest {
    pub game_id: GameIdType,
    pub card_idx: CardIdxType,
}

impl Bufferable for GameStartTurnRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        self.card_idx.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let card_idx = CardIdxType::pull_from(buf);
        Self {
            game_id,
            card_idx,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer() + self.card_idx.size_in_buffer()
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartTurnResponse {
    pub success: bool,
}

impl Bufferable for GameStartTurnResponse {
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
    use crate::message::game_start_turn::{GameStartTurnRequest, GameStartTurnResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_request() {
        let orig = GameStartTurnRequest {
            game_id: 1234567890,
            card_idx: 0,
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
