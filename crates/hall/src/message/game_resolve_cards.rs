use crate::message::CommandMessage;
use shared_net::bufferable_derive::Bufferable;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

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
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_response() {
        let orig = GameResolveCardsMessage {
            success: true,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameResolveCardsMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
