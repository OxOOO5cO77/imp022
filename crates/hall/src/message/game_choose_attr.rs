use shared_data::types::GameIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

type CardIdxType = u8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrRequest {
    pub game_id: GameIdType,
    pub card_idx: CardIdxType,
}

impl Bufferable for GameChooseAttrRequest {
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
pub struct GameChooseAttrResponse {
    pub success: bool,
}

impl Bufferable for GameChooseAttrResponse {
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
    use crate::message::game_choose_attr::{GameChooseAttrRequest, GameChooseAttrResponse};

    #[test]
    fn test_request() {
        let orig = GameChooseAttrRequest {
            game_id: 1234567890,
            card_idx: 0,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameChooseAttrRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let orig = GameChooseAttrResponse {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameChooseAttrResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
