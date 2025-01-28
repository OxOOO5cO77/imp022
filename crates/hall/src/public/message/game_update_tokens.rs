use shared_net::{op, Bufferable, SizedBuffer, SizedBufferError};

use crate::core::Token;
use crate::message::CommandMessage;

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UpdateTokenMessage {
    Add(Token),
    Expire(Token),
    Convert(Token, Token),
}

impl Bufferable for UpdateTokenMessage {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        match self {
            UpdateTokenMessage::Add(token) => {
                pushed += 0u8.push_into(buf)?;
                pushed += token.push_into(buf)?;
            }
            UpdateTokenMessage::Expire(token) => {
                pushed += 1u8.push_into(buf)?;
                pushed += token.push_into(buf)?;
            }
            UpdateTokenMessage::Convert(token_from, token_to) => {
                pushed += 2u8.push_into(buf)?;
                pushed += token_from.push_into(buf)?;
                pushed += token_to.push_into(buf)?;
            }
        }
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let kind = u8::pull_from(buf)?;
        let token = Token::pull_from(buf)?;
        let result = match kind {
            0 => UpdateTokenMessage::Add(token),
            1 => UpdateTokenMessage::Expire(token),
            2 => {
                let token_to = Token::pull_from(buf)?;
                UpdateTokenMessage::Convert(token, token_to)
            }
            _ => return Err(SizedBufferError::UnexpectedEnum(kind)),
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        match self {
            UpdateTokenMessage::Add(token) => size_of::<u8>() + token.size_in_buffer(),
            UpdateTokenMessage::Expire(token) => size_of::<u8>() + token.size_in_buffer(),
            UpdateTokenMessage::Convert(token_from, token_to) => size_of::<u8>() + token_from.size_in_buffer() + token_to.size_in_buffer(),
        }
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateTokensMessage {
    pub messages: Vec<UpdateTokenMessage>,
}

impl CommandMessage for GameUpdateTokensMessage {
    const COMMAND: op::Command = op::Command::GameUpdateTokens;
}

impl GameUpdateTokensMessage {
    pub fn new(messages: Vec<UpdateTokenMessage>) -> Self {
        Self {
            messages,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::Token;
    use crate::message::game_update_tokens::{GameUpdateTokensMessage, UpdateTokenMessage};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
        let orig = GameUpdateTokensMessage::new(vec![
            //
            UpdateTokenMessage::Add(Token::test_default(0)),
            UpdateTokenMessage::Expire(Token::test_default(1)),
            UpdateTokenMessage::Convert(Token::test_default(0), Token::test_default(2)),
        ]);

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameUpdateTokensMessage>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
