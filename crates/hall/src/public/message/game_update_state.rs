use shared_net::{op, Bufferable, GameIdType, SizedBuffer, SizedBufferError};

use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use crate::view::{GameMachinePlayerView, GameMissionPlayerView, GameUserStatePlayerView};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateStateRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameUpdateStateRequest {
    const COMMAND: op::Command = op::Command::GameUpdateState;
}

impl GameRequestMessage for GameUpdateStateRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateStateResponse {
    pub player_state: GameUserStatePlayerView,
    pub local_machine: GameMachinePlayerView,
    pub remote_machine: GameMachinePlayerView,
    pub mission: GameMissionPlayerView,
}

impl CommandMessage for GameUpdateStateResponse {
    const COMMAND: op::Command = op::Command::GameUpdateState;
}

impl GameResponseMessage for GameUpdateStateResponse {}

#[cfg(test)]
mod test {
    use crate::message::{GameUpdateStateRequest, GameUpdateStateResponse};
    use crate::view::{GameMachinePlayerView, GameMissionPlayerView, GameUserStatePlayerView};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameUpdateStateRequest {
            game_id: 1234567890,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameUpdateStateRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameUpdateStateResponse {
            player_state: GameUserStatePlayerView {
                attr: [[1, 2, 3, 4], [5, 6, 7, 8], [9, 1, 2, 3], [4, 5, 6, 7]],
                deck: 23,
                heap: vec![],
                hand: vec![],
                erg: [14, 13, 12, 11],
            },
            local_machine: GameMachinePlayerView::test_default(),
            remote_machine: GameMachinePlayerView::test_default(),
            mission: GameMissionPlayerView::test_default(),
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameUpdateStateResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
