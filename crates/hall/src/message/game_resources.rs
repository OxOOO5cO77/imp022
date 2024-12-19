use crate::data::core::{AttributeKind, AttributeValueType, ErgType};
use crate::data::player::PlayerStatePlayerView;
use crate::message::CommandMessage;
use shared_net::bufferable_derive::Bufferable;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResourcesMessage {
    pub player_state_view: PlayerStatePlayerView,
    pub remote_attr: [AttributeValueType; 4],
    pub local_erg: [ErgType; 4],
    pub remote_erg: [ErgType; 4],
    pub remote_kind: AttributeKind,
}

impl CommandMessage for GameResourcesMessage {
    const COMMAND: op::Command = op::Command::GameResources;
}

#[cfg(test)]
mod test {
    use crate::data::core::AttributeKind;
    use crate::data::player::PlayerStatePlayerView;
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
            remote_kind: AttributeKind::Compute,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameResourcesMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
