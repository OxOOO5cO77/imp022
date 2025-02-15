use crate::core::{AttributeArray, AttributeKind, ErgArray, GameSubCommand};
use crate::message::CommandMessage;
use crate::view::GameUserStatePlayerView;
use shared_net::op::SubCommandType;
use shared_net::{op, Bufferable, SizedBuffer, SizedBufferError};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResourcesMessage {
    pub player_state_view: GameUserStatePlayerView,
    pub remote_attr: AttributeArray,
    pub local_erg: ErgArray,
    pub remote_erg: ErgArray,
    pub remote_kind: AttributeKind,
}

impl CommandMessage for GameResourcesMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Resources as SubCommandType);
}

#[cfg(test)]
mod test {
    use crate::core::AttributeKind;
    use crate::message::game_resources::GameResourcesMessage;
    use crate::view::GameUserStatePlayerView;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameResourcesMessage {
            player_state_view: GameUserStatePlayerView::default(),
            remote_attr: [4, 5, 6, 5],
            local_erg: [0, 6, 0, 2],
            remote_erg: [5, 0, 1, 0],
            remote_kind: AttributeKind::Compute,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameResourcesMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
