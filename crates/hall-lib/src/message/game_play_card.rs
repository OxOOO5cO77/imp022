use crate::core::{GameSubCommand, PickedCardTarget};
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::op::SubCommandType;
use shared_net::{op, Bufferable, GameIdType, SizedBuffer, SizedBufferError};

pub type CardIdxType = u8;

pub type PicksType = Vec<(CardIdxType, PickedCardTarget)>;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GamePlayCardRequest {
    pub game_id: GameIdType,
    pub picks: PicksType,
}

impl CommandMessage for GamePlayCardRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::PlayCard as SubCommandType);
}

impl GameRequestMessage for GamePlayCardRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GamePlayCardResponse {
    pub success: [bool; 5],
}

impl CommandMessage for GamePlayCardResponse {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::PlayCard as SubCommandType);
}

impl GameResponseMessage for GamePlayCardResponse {}

#[cfg(test)]
mod test {
    use crate::core::PickedCardTarget;
    use crate::message::game_play_card::{GamePlayCardRequest, GamePlayCardResponse};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GamePlayCardRequest {
            game_id: 1234567890,
            picks: vec![(0, PickedCardTarget::MachineLocal), (1, PickedCardTarget::MachineRemote), (2, PickedCardTarget::Actor(4))],
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GamePlayCardRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GamePlayCardResponse {
            success: [true, false, true, false, true],
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GamePlayCardResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
