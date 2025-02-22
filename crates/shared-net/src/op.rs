use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt;
use std::mem::size_of;
#[cfg(test)]
use strum_macros::EnumIter;

use crate::SizedBuffer;
use crate::sizedbuffers::{Bufferable, SizedBufferError};
use crate::types::NodeType;

type RouteType = u8;

#[derive(Clone, PartialEq)]
pub enum Route {
    None,
    Local,
    One(NodeType),
    Any(Flavor),
    All(Flavor),
}

impl Route {
    const REPR_NONE: RouteType = 0;
    const REPR_LOCAL: RouteType = 1;
    const REPR_ONE: RouteType = 2;
    const REPR_ANY: RouteType = 3;
    const REPR_ALL: RouteType = 4;
}

impl Bufferable for Route {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        match *self {
            Route::None => pushed += Self::REPR_NONE.push_into(buf)?,
            Route::Local => pushed += Self::REPR_LOCAL.push_into(buf)?,
            Route::One(destination) => {
                pushed += Self::REPR_ONE.push_into(buf)?;
                pushed += destination.push_into(buf)?;
            }
            Route::Any(flavor) => {
                pushed += Self::REPR_ANY.push_into(buf)?;
                pushed += flavor.push_into(buf)?;
            }
            Route::All(flavor) => {
                pushed += Self::REPR_ALL.push_into(buf)?;
                pushed += flavor.push_into(buf)?;
            }
        };
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let route = u8::pull_from(buf)?;
        let result = match route {
            Self::REPR_LOCAL => Route::Local,
            Self::REPR_ONE => Route::One(u8::pull_from(buf)?),
            Self::REPR_ANY => Route::Any(Flavor::pull_from(buf)?),
            Self::REPR_ALL => Route::All(Flavor::pull_from(buf)?),
            _ => Route::None,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        match *self {
            Route::None => size_of::<RouteType>(),
            Route::Local => size_of::<RouteType>(),
            Route::One(one) => size_of::<RouteType>() + one.size_in_buffer(),
            Route::Any(any) => size_of::<RouteType>() + any.size_in_buffer(),
            Route::All(all) => size_of::<RouteType>() + all.size_in_buffer(),
        }
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Route::None => write!(f, "None"),
            Route::Local => write!(f, "Local"),
            Route::One(id) => write!(f, "One({})", id),
            Route::Any(flavor) => write!(f, "Any({:?})", flavor),
            Route::All(flavor) => write!(f, "All({:?})", flavor),
        }
    }
}

type FlavorType = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Flavor {
    #[num_enum(default)]
    NoOp = 0,
    Archive = 1,
    Bazaar = 2,
    Courtyard = 3,
    Drawbridge = 4,
    Forum = 6,
    Gate = 7,
    Hall = 8,
    Jail = 10,
    Lookout = 12,
    Vagabond = 22,
    Warehouse = 23,
}

impl Bufferable for Flavor {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let flavor: FlavorType = (*self).into();
        flavor.push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let flavor = FlavorType::pull_from(buf)?;
        Ok(flavor.into())
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<FlavorType>()
    }
}

type CommandType = u8;
pub type SubCommandType = u8;

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Command {
    NoOp,
    Register,
    Authorize,
    Hello,
    UserAttr,
    Message(SubCommandType),
    Inventory(SubCommandType),
    Game(SubCommandType),
}

impl Command {
    const REPR_NOOP: CommandType = 0;
    const REPR_REGISTER: CommandType = 1;
    const REPR_AUTHORIZE: CommandType = 2;
    const REPR_HELLO: CommandType = 3;
    const REPR_USERATTR: CommandType = 4;
    const REPR_MESSAGE: CommandType = 5;
    const REPR_INVENTORY: CommandType = 6;
    const REPR_GAME: CommandType = 7;
}

impl Bufferable for Command {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        match *self {
            Command::NoOp => Command::REPR_NOOP.push_into(buf),
            Command::Register => Command::REPR_REGISTER.push_into(buf),
            Command::Authorize => Command::REPR_AUTHORIZE.push_into(buf),
            Command::Hello => Command::REPR_HELLO.push_into(buf),
            Command::UserAttr => Command::REPR_USERATTR.push_into(buf),
            Command::Message(sub) => (Command::REPR_MESSAGE, sub).push_into(buf),
            Command::Inventory(sub) => (Command::REPR_INVENTORY, sub).push_into(buf),
            Command::Game(sub) => (Command::REPR_GAME, sub).push_into(buf),
        }
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let command = CommandType::pull_from(buf)?;
        let result = match command {
            Command::REPR_NOOP => Command::NoOp,
            Command::REPR_REGISTER => Command::Register,
            Command::REPR_AUTHORIZE => Command::Authorize,
            Command::REPR_HELLO => Command::Hello,
            Command::REPR_USERATTR => Command::UserAttr,
            Command::REPR_MESSAGE => Command::Message(SubCommandType::pull_from(buf)?),
            Command::REPR_INVENTORY => Command::Inventory(SubCommandType::pull_from(buf)?),
            Command::REPR_GAME => Command::Game(SubCommandType::pull_from(buf)?),
            _ => return Err(SizedBufferError::UnexpectedEnum(command)),
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        match self {
            Command::NoOp => size_of::<CommandType>(),
            Command::Register => size_of::<CommandType>(),
            Command::Authorize => size_of::<CommandType>(),
            Command::Hello => size_of::<CommandType>(),
            Command::UserAttr => size_of::<CommandType>(),
            Command::Message(sub) => size_of::<CommandType>() + sub.size_in_buffer(),
            Command::Inventory(sub) => size_of::<CommandType>() + sub.size_in_buffer(),
            Command::Game(sub) => size_of::<CommandType>() + sub.size_in_buffer(),
        }
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use strum::IntoEnumIterator;

        use crate::SizedBuffer;
        use crate::op::Flavor;
        use crate::op::{Command, Route};
        use crate::sizedbuffers::SizedBufferError;

        #[test]
        fn test_route() -> Result<(), SizedBufferError> {
            let mut buf1 = SizedBuffer::new(32);

            let route = Route::Any(Flavor::Courtyard);

            buf1.push(&route)?;
            buf1.push(&route)?;

            assert_eq!(route, buf1.pull::<Route>()?);

            let mut buf2 = SizedBuffer::new(32);
            buf2.xfer::<Route>(&mut buf1)?;

            assert_eq!(route, buf2.pull::<Route>()?);
            Ok(())
        }

        #[test]
        fn test_flavor() -> Result<(), SizedBufferError> {
            for flavor in Flavor::iter() {
                let mut buf1 = SizedBuffer::new(32);

                buf1.push(&flavor)?;
                buf1.push(&flavor)?;

                assert_eq!(flavor, buf1.pull::<Flavor>()?);

                let mut buf2 = SizedBuffer::new(32);
                buf2.xfer::<Flavor>(&mut buf1)?;

                assert_eq!(flavor, buf2.pull::<Flavor>()?);
            }
            Ok(())
        }

        #[test]
        fn test_command() -> Result<(), SizedBufferError> {
            for command in Command::iter() {
                let mut buf1 = SizedBuffer::new(32);
                buf1.push(&command)?;
                buf1.push(&command)?;

                assert_eq!(command, buf1.pull::<Command>()?);

                let mut buf2 = SizedBuffer::new(32);
                buf2.xfer::<Command>(&mut buf1)?;

                assert_eq!(command, buf2.pull::<Command>()?);
            }
            Ok(())
        }
    }
}
