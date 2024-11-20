use crate::data::player::player_state::PlayerStatePlayerView;
use crate::message::CommandMessage;
use shared_data::attribute::AttributeValueType;
use shared_data::card::ErgType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResourcesMessage {
    pub player_state_view: PlayerStatePlayerView,
    pub remote_attr: [AttributeValueType; 4],
    pub local_erg: [ErgType; 4],
    pub remote_erg: [ErgType; 4],
}

impl CommandMessage for GameResourcesMessage {
    const COMMAND: op::Command = op::Command::GameResources;
}

impl Bufferable for GameResourcesMessage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.player_state_view.push_into(buf);
        self.remote_attr.push_into(buf);
        self.local_erg.push_into(buf);
        self.remote_erg.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let player_state_view = PlayerStatePlayerView::pull_from(buf);
        let remote_attr = <[AttributeValueType; 4]>::pull_from(buf);
        let local_erg = <[ErgType; 4]>::pull_from(buf);
        let remote_erg = <[ErgType; 4]>::pull_from(buf);
        Self {
            player_state_view,
            remote_attr,
            local_erg,
            remote_erg,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.player_state_view.size_in_buffer() + self.remote_attr.size_in_buffer() + self.local_erg.size_in_buffer() + self.remote_erg.size_in_buffer()
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
            remote_attr: [4, 5, 6, 5],
            local_erg: [0, 6, 0, 2],
            remote_erg: [5, 0, 1, 0],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameResourcesMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
