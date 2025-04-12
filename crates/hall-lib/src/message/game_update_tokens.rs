use crate::core::{GameSubCommand, Token};
use crate::message::CommandMessage;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, SizedBuffer, SizedBufferError, op};

#[derive(Clone)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum UpdateTokenMessage {
    Add(Token),
    Expire(Token),
    Extend(Token),
    Convert(Token, Token),
}

const UTM_ADD: u8 = 0;
const UTM_EXPIRE: u8 = 1;
const UTM_EXTEND: u8 = 2;
const UTM_CONVERT: u8 = 3;

impl Bufferable for UpdateTokenMessage {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        match self {
            UpdateTokenMessage::Add(token) => {
                pushed += UTM_ADD.push_into(buf)?;
                pushed += token.push_into(buf)?;
            }
            UpdateTokenMessage::Expire(token) => {
                pushed += UTM_EXPIRE.push_into(buf)?;
                pushed += token.push_into(buf)?;
            }
            UpdateTokenMessage::Extend(token) => {
                pushed += UTM_EXTEND.push_into(buf)?;
                pushed += token.push_into(buf)?;
            }
            UpdateTokenMessage::Convert(token_from, token_to) => {
                pushed += UTM_CONVERT.push_into(buf)?;
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
            UTM_ADD => UpdateTokenMessage::Add(token),
            UTM_EXPIRE => UpdateTokenMessage::Expire(token),
            UTM_EXTEND => UpdateTokenMessage::Extend(token),
            UTM_CONVERT => {
                let token_to = Token::pull_from(buf)?;
                UpdateTokenMessage::Convert(token, token_to)
            }
            _ => return Err(SizedBufferError::UnexpectedEnum(kind)),
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        match self {
            UpdateTokenMessage::Add(token) => UTM_ADD.size_in_buffer() + token.size_in_buffer(),
            UpdateTokenMessage::Expire(token) => UTM_EXPIRE.size_in_buffer() + token.size_in_buffer(),
            UpdateTokenMessage::Extend(token) => UTM_EXTEND.size_in_buffer() + token.size_in_buffer(),
            UpdateTokenMessage::Convert(token_from, token_to) => UTM_CONVERT.size_in_buffer() + token_from.size_in_buffer() + token_to.size_in_buffer(),
        }
    }
}

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameUpdateTokensMessage {
    pub messages: Vec<UpdateTokenMessage>,
}

impl CommandMessage for GameUpdateTokensMessage {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::UpdateTokens as SubCommandType);
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
    use super::{GameUpdateTokensMessage, UpdateTokenMessage};
    use crate::core::Token;
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
