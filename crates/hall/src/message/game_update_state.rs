use crate::data::game::GameMachinePlayerView;
use crate::data::player::player_state::PlayerStatePlayerView;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::GameIdType;
use shared_net::{op, VSizedBuffer};

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

impl Bufferable for GameUpdateStateRequest {
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
pub struct GameUpdateStateResponse {
    pub player_state: PlayerStatePlayerView,
    pub local_machine: GameMachinePlayerView,
    pub remote_machine: GameMachinePlayerView,
}

impl CommandMessage for GameUpdateStateResponse {
    const COMMAND: op::Command = op::Command::GameUpdateState;
}

impl GameResponseMessage for GameUpdateStateResponse {}

impl Bufferable for GameUpdateStateResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_state.push_into(buf);
        self.local_machine.push_into(buf);
        self.remote_machine.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_state = PlayerStatePlayerView::pull_from(buf);
        let local_machine = GameMachinePlayerView::pull_from(buf);
        let remote_machine = GameMachinePlayerView::pull_from(buf);
        Self {
            player_state,
            local_machine,
            remote_machine,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_state.size_in_buffer() + self.local_machine.size_in_buffer() + self.remote_machine.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::data::game::GameMachinePlayerView;
    use crate::data::player::player_state::PlayerStatePlayerView;
    use crate::message::game_update_state::GameUpdateStateResponse;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

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
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameUpdateStateResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
