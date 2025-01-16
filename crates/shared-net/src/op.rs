use num_enum::{FromPrimitive, IntoPrimitive};
use std::fmt;
use std::mem::size_of;
#[cfg(test)]
use strum_macros::EnumIter;

use crate::sizedbuffers::Bufferable;
use crate::types::NodeType;
use crate::VSizedBuffer;

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
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match *self {
            Route::None => Self::REPR_NONE.push_into(buf),
            Route::Local => Self::REPR_LOCAL.push_into(buf),
            Route::One(destination) => {
                Self::REPR_ONE.push_into(buf);
                destination.push_into(buf);
            }
            Route::Any(flavor) => {
                Self::REPR_ANY.push_into(buf);
                flavor.push_into(buf);
            }
            Route::All(flavor) => {
                Self::REPR_ALL.push_into(buf);
                flavor.push_into(buf);
            }
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let route = u8::pull_from(buf);
        match route {
            Self::REPR_LOCAL => Route::Local,
            Self::REPR_ONE => Route::One(u8::pull_from(buf)),
            Self::REPR_ANY => Route::Any(Flavor::pull_from(buf)),
            Self::REPR_ALL => Route::All(Flavor::pull_from(buf)),
            _ => Route::None,
        }
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
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let flavor: FlavorType = (*self).into();
        flavor.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let flavor = FlavorType::pull_from(buf);
        flavor.into()
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<FlavorType>()
    }
}

type CommandType = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Command {
    #[num_enum(default)]
    NoOp,
    Register,
    Authorize,
    Hello,
    UserAttr,
    Chat,
    DM,
    InvGen,
    InvList,
    GameActivate,
    GameBuild,
    GameStartGame,
    GameChooseIntent,
    GameRoll,
    GameChooseAttr,
    GameResources,
    GamePlayCard,
    GameResolveCards,
    GameEndTurn,
    GameTick,
    GameUpdateMission,
    GameUpdateTokens,
    GameUpdateState,
    GameEndGame,
}

impl Bufferable for Command {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let command: CommandType = (*self).into();
        command.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let command = CommandType::pull_from(buf);
        command.into()
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<CommandType>()
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use strum::IntoEnumIterator;

        use crate::op::Flavor;
        use crate::op::{Command, Route};
        use crate::VSizedBuffer;

        #[test]
        fn route() {
            let mut buf1 = VSizedBuffer::new(32);

            let route = Route::Any(Flavor::Courtyard);

            buf1.push(&route);
            buf1.push(&route);

            assert_eq!(route, buf1.pull::<Route>());

            let mut buf2 = VSizedBuffer::new(32);
            buf2.xfer::<Route>(&mut buf1);

            assert_eq!(route, buf2.pull::<Route>());
        }

        #[test]
        fn flavor() {
            for flavor in Flavor::iter() {
                let mut buf1 = VSizedBuffer::new(32);

                buf1.push(&flavor);
                buf1.push(&flavor);

                assert_eq!(flavor, buf1.pull::<Flavor>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<Flavor>(&mut buf1);

                assert_eq!(flavor, buf2.pull::<Flavor>());
            }
        }

        #[test]
        fn command() {
            for command in Command::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push(&command);
                buf1.push(&command);

                assert_eq!(command, buf1.pull::<Command>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<Command>(&mut buf1);

                assert_eq!(command, buf2.pull::<Command>());
            }
        }
    }
}
