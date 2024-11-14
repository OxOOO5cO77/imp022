use crate::data::player::player_state::PlayerStatePlayerView;
use crate::message::CommandMessage;
use shared_data::game::card::ErgType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResourcesMessage {
    pub player_state_view: PlayerStatePlayerView,
    pub p_erg: [ErgType; 4],
    pub a_erg: [ErgType; 4],
}

impl CommandMessage for GameResourcesMessage {
    const COMMAND: op::Command = op::Command::GameResources;
}

impl Bufferable for GameResourcesMessage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_state_view.push_into(buf);
        self.p_erg.push_into(buf);
        self.a_erg.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_state_view = PlayerStatePlayerView::pull_from(buf);
        let p_erg = <[ErgType; 4]>::pull_from(buf);
        let a_erg = <[ErgType; 4]>::pull_from(buf);
        Self {
            player_state_view,
            p_erg,
            a_erg,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_state_view.size_in_buffer() + self.p_erg.size_in_buffer() + self.a_erg.size_in_buffer()
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
            p_erg: [0, 6, 0, 2],
            a_erg: [5, 0, 1, 0],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameResourcesMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
