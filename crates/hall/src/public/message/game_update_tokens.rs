use shared_net::{op, Bufferable, VSizedBuffer};

use crate::core::Token;
use crate::message::CommandMessage;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateTokensMessage {
    pub token: Token,
}

impl CommandMessage for GameUpdateTokensMessage {
    const COMMAND: op::Command = op::Command::GameUpdateTokens;
}

impl GameUpdateTokensMessage {
    pub fn new(new: Token) -> Self {
        Self {
            token: new,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::Token;
    use crate::message::game_update_tokens::GameUpdateTokensMessage;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_response() {
        let orig = GameUpdateTokensMessage::new(Token::test_default(0));

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameUpdateTokensMessage>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
