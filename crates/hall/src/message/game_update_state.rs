use shared_net::bufferable_derive::Bufferable;
use shared_net::{op, Bufferable, GameIdType, VSizedBuffer};

use crate::data::game::{GameMachine, GameMachinePlayerView, GameMission, GameMissionPlayerView, GameUser};
use crate::data::player::PlayerStatePlayerView;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};

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
    pub player_state: PlayerStatePlayerView,
    pub local_machine: GameMachinePlayerView,
    pub remote_machine: GameMachinePlayerView,
    pub mission: GameMissionPlayerView,
}

impl CommandMessage for GameUpdateStateResponse {
    const COMMAND: op::Command = op::Command::GameUpdateState;
}

impl GameUpdateStateResponse {
    pub fn new(user: &GameUser, remote: &GameMachine, mission: &GameMission) -> Self {
        Self {
            player_state: PlayerStatePlayerView::from(&user.state),
            local_machine: GameMachinePlayerView::from(&user.machine),
            remote_machine: GameMachinePlayerView::from(remote),
            mission: GameMissionPlayerView::new(mission, &user.mission_state),
        }
    }
}

impl GameResponseMessage for GameUpdateStateResponse {}

#[cfg(test)]
mod test {
    use crate::data::game::{GameMachinePlayerView, GameMissionPlayerView};
    use crate::data::player::PlayerStatePlayerView;
    use crate::message::game_update_state::GameUpdateStateResponse;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameUpdateStateResponse {
            player_state: PlayerStatePlayerView {
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

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameUpdateStateResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
