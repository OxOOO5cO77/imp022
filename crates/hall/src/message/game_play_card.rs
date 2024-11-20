use crate::message::CommandMessage;
use shared_net::types::GameIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

pub type CardIdxType = u8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GamePlayCardRequest {
    pub game_id: GameIdType,
    pub card_idx: CardIdxType,
}

impl CommandMessage for GamePlayCardRequest {
    const COMMAND: op::Command = op::Command::GamePlayCard;
}

impl Bufferable for GamePlayCardRequest {
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
pub struct GamePlayCardResponse {
    pub success: bool,
}

impl CommandMessage for GamePlayCardResponse {
    const COMMAND: op::Command = op::Command::GamePlayCard;
}

impl Bufferable for GamePlayCardResponse {
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
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::message::game_play_card::{GamePlayCardRequest, GamePlayCardResponse};

    #[test]
    fn test_request() {
        let orig = GamePlayCardRequest {
            game_id: 1234567890,
            card_idx: 0,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GamePlayCardRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let orig = GamePlayCardResponse {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GamePlayCardResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
