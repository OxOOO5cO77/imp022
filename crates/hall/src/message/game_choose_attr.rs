use crate::message::CommandMessage;
use shared_data::game::card::Kind;
use shared_data::types::GameIdType;

use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[derive(Copy, Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum AttrKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}

impl Bufferable for AttrKind {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            AttrKind::Analyze => 1u8,
            AttrKind::Breach => 2,
            AttrKind::Compute => 3,
            AttrKind::Disrupt => 4,
        }
        .push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let value = u8::pull_from(buf);
        match value {
            1 => AttrKind::Analyze,
            2 => AttrKind::Breach,
            3 => AttrKind::Compute,
            4 => AttrKind::Disrupt,
            _ => AttrKind::Analyze,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>()
    }
}

impl From<Kind> for AttrKind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Analyze => AttrKind::Analyze,
            Kind::Breach => AttrKind::Breach,
            Kind::Compute => AttrKind::Compute,
            Kind::Disrupt => AttrKind::Disrupt,
        }
    }
}

impl From<AttrKind> for Kind {
    fn from(value: AttrKind) -> Self {
        match value {
            AttrKind::Analyze => Kind::Analyze,
            AttrKind::Breach => Kind::Breach,
            AttrKind::Compute => Kind::Compute,
            AttrKind::Disrupt => Kind::Disrupt,
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrRequest {
    pub game_id: GameIdType,
    pub attr: AttrKind,
}

impl CommandMessage for GameChooseAttrRequest {
    const COMMAND: op::Command = op::Command::GameChooseAttr;
}

impl Bufferable for GameChooseAttrRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        self.attr.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let attr = AttrKind::pull_from(buf);
        Self {
            game_id,
            attr,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer() + self.attr.size_in_buffer()
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrResponse {
    pub success: bool,
}

impl CommandMessage for GameChooseAttrResponse {
    const COMMAND: op::Command = op::Command::GameChooseAttr;
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
    use crate::message::game_choose_attr::{AttrKind, GameChooseAttrRequest, GameChooseAttrResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn attr_kind() {
        let orig1 = AttrKind::Analyze;
        let orig2 = AttrKind::Disrupt;

        let mut buf1 = VSizedBuffer::new(32);
        buf1.push(&orig1);
        buf1.push(&orig2);

        assert_eq!(orig1, buf1.pull::<AttrKind>());

        let mut buf2 = VSizedBuffer::new(32);
        buf2.xfer::<AttrKind>(&mut buf1);

        assert_eq!(orig2, buf2.pull::<AttrKind>());
    }

    #[test]
    fn test_request() {
        let orig = GameChooseAttrRequest {
            game_id: 1234567890,
            attr: AttrKind::Compute,
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
