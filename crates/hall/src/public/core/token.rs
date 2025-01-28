use std::fmt::{Display, Formatter};

use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::TickType;

pub const DEFAULT_TOKEN_EXPIRY: TickType = 10;

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug))]
pub enum AuthLevel {
    #[default]
    Anonymous,
    Guest,
    User,
    Admin,
    Root,
}

impl AuthLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            AuthLevel::Anonymous => "Anonymous",
            AuthLevel::Guest => "Guest",
            AuthLevel::User => "User",
            AuthLevel::Admin => "Admin",
            AuthLevel::Root => "Root",
        }
    }
}

impl Display for AuthLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(Debug))]
pub enum TokenKind {
    #[default]
    Invalid,
    Authorization(AuthLevel),
    Credentials(AuthLevel),
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(test, derive(Debug))]
pub struct Token {
    pub kind: TokenKind,
    pub expiry: TickType,
}

impl Token {
    pub fn new(kind: TokenKind, expiry: TickType) -> Self {
        Self {
            kind,
            expiry,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Invalid => write!(f, "[Invalid]"),
            TokenKind::Authorization(level) => write!(f, "{level} Auth"),
            TokenKind::Credentials(level) => write!(f, "{level} Creds"),
        }
    }
}

impl TokenKind {
    pub fn level(&self) -> AuthLevel {
        match self {
            TokenKind::Invalid => AuthLevel::Anonymous,
            TokenKind::Authorization(level) => *level,
            TokenKind::Credentials(level) => *level,
        }
    }
}

type PackedTokenType = u32;

const BITS_KIND: PackedTokenType = 4;
const BITS_KIND_VALUE: PackedTokenType = 8;
const BITS_EXPIRY: PackedTokenType = 16;

const SHIFT_KIND: PackedTokenType = 0;
const SHIFT_KIND_VALUE: PackedTokenType = SHIFT_KIND + BITS_KIND;
const SHIFT_EXPIRY: PackedTokenType = SHIFT_KIND_VALUE + BITS_KIND_VALUE;

const MASK_KIND: PackedTokenType = (1 << BITS_KIND) - 1;
const MASK_KIND_VALUE: PackedTokenType = (1 << BITS_KIND_VALUE) - 1;
const MASK_EXPIRY: PackedTokenType = (1 << BITS_EXPIRY) - 1;

impl Token {
    fn pack_auth_level(level: AuthLevel) -> PackedTokenType {
        let value: u8 = level.into();
        value as PackedTokenType
    }
    fn unpack_auth_level(packed: PackedTokenType) -> AuthLevel {
        let value = packed as u8;
        value.into()
    }

    fn pack_kind(kind: &TokenKind) -> PackedTokenType {
        let (k, v) = match kind {
            TokenKind::Invalid => (0, 0),
            TokenKind::Authorization(level) => (1, Self::pack_auth_level(*level)),
            TokenKind::Credentials(level) => (2, Self::pack_auth_level(*level)),
        };
        (k << SHIFT_KIND) | (v << SHIFT_KIND_VALUE)
    }
    fn unpack_kind(packed: PackedTokenType) -> TokenKind {
        let k = (packed >> SHIFT_KIND) & MASK_KIND;
        let v = (packed >> SHIFT_KIND_VALUE) & MASK_KIND_VALUE;
        match k {
            1 => TokenKind::Authorization(Self::unpack_auth_level(v)),
            2 => TokenKind::Credentials(Self::unpack_auth_level(v)),
            _ => TokenKind::Invalid,
        }
    }

    fn pack_expiry(expiry: TickType) -> PackedTokenType {
        (expiry as PackedTokenType) << SHIFT_EXPIRY
    }
    fn unpack_expiry(packed: PackedTokenType) -> TickType {
        let unpacked = (packed >> SHIFT_EXPIRY) & MASK_EXPIRY;
        unpacked as TickType
    }

    fn pack(&self) -> PackedTokenType {
        Self::pack_kind(&self.kind) | Self::pack_expiry(self.expiry)
    }
    fn unpack(packed: PackedTokenType) -> Self {
        let kind = Self::unpack_kind(packed);
        let expiry = Self::unpack_expiry(packed);
        Self {
            kind,
            expiry,
        }
    }
}

impl Bufferable for Token {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let packed = self.pack();
        packed.push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let packed = PackedTokenType::pull_from(buf)?;
        Ok(Self::unpack(packed))
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedTokenType>()
    }
}

#[cfg(test)]
impl Token {
    pub(crate) fn test_default(idx: usize) -> Self {
        match idx {
            1 => Token::new(TokenKind::Credentials(AuthLevel::Admin), 12345),
            2 => Token::new(TokenKind::Authorization(AuthLevel::User), 54321),
            _ => Token::new(TokenKind::Authorization(AuthLevel::Guest), 0),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::Token;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_token() -> Result<(), SizedBufferError> {
        let orig = vec![Token::test_default(1), Token::test_default(2)];

        let mut buf = SizedBuffer::from(&orig)?;
        let new = buf.pull::<Vec<Token>>()?;

        assert_eq!(orig, new);
        Ok(())
    }
}
