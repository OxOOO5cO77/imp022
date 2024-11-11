use crate::data::player::player_state::PlayerStatePlayerView;
use crate::message::CommandMessage;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateStateMessage {
    pub player_state: PlayerStatePlayerView,
}

impl CommandMessage for GameUpdateStateMessage {
    const COMMAND: op::Command = op::Command::GameUpdateState;
}

impl Bufferable for GameUpdateStateMessage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_state.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_state = PlayerStatePlayerView::pull_from(buf);
        Self {
            player_state,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_state.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::data::player::player_state::PlayerStatePlayerView;
    use crate::message::game_update_state::GameUpdateStateMessage;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_response() {
        let orig = GameUpdateStateMessage {
            player_state: PlayerStatePlayerView::default(),
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameUpdateStateMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
