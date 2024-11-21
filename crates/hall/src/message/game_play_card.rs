use crate::message::CommandMessage;
use shared_data::mission::MissionNodeIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::GameIdType;
use shared_net::{op, VSizedBuffer};

pub type CardIdxType = u8;

#[derive(Copy, Clone, Default)]
#[repr(u8)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum CardTarget {
    #[default]
    Local,
    Remote(MissionNodeIdType),
}

type PicksType = Vec<(CardIdxType, CardTarget)>;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GamePlayCardRequest {
    pub game_id: GameIdType,
    pub picks: PicksType,
}

impl CommandMessage for GamePlayCardRequest {
    const COMMAND: op::Command = op::Command::GamePlayCard;
}

impl Bufferable for CardTarget {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            CardTarget::Local => 0,
            CardTarget::Remote(node) => *node,
        }
        .push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        match MissionNodeIdType::pull_from(buf) {
            0 => CardTarget::Local,
            node => CardTarget::Remote(node),
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<MissionNodeIdType>()
    }
}

impl Bufferable for GamePlayCardRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        self.picks.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let picks = PicksType::pull_from(buf);
        Self {
            game_id,
            picks,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer() + self.picks.size_in_buffer()
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GamePlayCardResponse {
    pub success: Vec<bool>,
}

impl CommandMessage for GamePlayCardResponse {
    const COMMAND: op::Command = op::Command::GamePlayCard;
}

impl Bufferable for GamePlayCardResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.success.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let success = <Vec<bool>>::pull_from(buf);
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
    use crate::message::game_play_card::{CardTarget, GamePlayCardRequest, GamePlayCardResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_request() {
        let orig = GamePlayCardRequest {
            game_id: 1234567890,
            picks: vec![(0, CardTarget::Local), (1, CardTarget::Remote(1)), (2, CardTarget::Remote(1))],
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
            success: vec![true,false,true,false],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GamePlayCardResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
