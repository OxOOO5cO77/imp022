use shared_net::{op, Bufferable, GameIdType, VSizedBuffer};

use crate::data::core::AttributeKind;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrRequest {
    pub game_id: GameIdType,
    pub attr: AttributeKind,
}

impl CommandMessage for GameChooseAttrRequest {
    const COMMAND: op::Command = op::Command::GameChooseAttr;
}

impl GameRequestMessage for GameChooseAttrRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameChooseAttrResponse {
    pub success: bool,
}

impl CommandMessage for GameChooseAttrResponse {
    const COMMAND: op::Command = op::Command::GameChooseAttr;
}

impl GameResponseMessage for GameChooseAttrResponse {}

#[cfg(test)]
mod test {
    use crate::message::game_choose_attr::{AttributeKind, GameChooseAttrRequest, GameChooseAttrResponse};
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_request() {
        let orig = GameChooseAttrRequest {
            game_id: 1234567890,
            attr: AttributeKind::Compute,
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
