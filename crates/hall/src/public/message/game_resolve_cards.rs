use shared_net::{op, Bufferable, SizedBuffer, SizedBufferError};

use crate::message::CommandMessage;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameResolveCardsMessage {
    pub success: bool,
}

impl CommandMessage for GameResolveCardsMessage {
    const COMMAND: op::Command = op::Command::GameResolveCards;
}

#[cfg(test)]
mod test {
    use crate::message::game_resolve_cards::GameResolveCardsMessage;
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameResolveCardsMessage {
            success: true,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameResolveCardsMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
