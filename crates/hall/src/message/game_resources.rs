use crate::data::player::player_state::PlayerStatePlayerView;
use crate::message::CommandMessage;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResourcesMessage {
    pub player_state_view: PlayerStatePlayerView,
}

impl CommandMessage for GameResourcesMessage {
    const COMMAND: op::Command = op::Command::GameResources;
}

impl Bufferable for GameResourcesMessage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_state_view.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_state_view = PlayerStatePlayerView::pull_from(buf);
        Self {
            player_state_view,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_state_view.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use crate::data::player::player_state::PlayerStatePlayerView;
    use crate::message::game_resources::GameResourcesMessage;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_response() {
        let orig = GameResourcesMessage {
            player_state_view: PlayerStatePlayerView::default(),
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameResourcesMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
