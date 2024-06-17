use std::fmt;
use std::mem::size_of;

#[cfg(test)]
use strum_macros::EnumIter;

use crate::sizedbuffers::Bufferable;
use crate::VSizedBuffer;

#[derive(PartialEq)]
pub enum Route {
    None,
    Local,
    One(u8),
    Any(Flavor),
    All(Flavor),
}

impl Bufferable for Route {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match *self {
            Route::None => 0_u8.push_into(buf),
            Route::Local => 1_u8.push_into(buf),
            Route::One(destination) => {
                2_u8.push_into(buf);
                destination.push_into(buf);
            }
            Route::Any(flavor) => {
                3_u8.push_into(buf);
                flavor.push_into(buf);
            }
            Route::All(flavor) => {
                4_u8.push_into(buf);
                flavor.push_into(buf);
            }
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let route = u8::pull_from(buf);
        match route {
            1 => Route::Local,
            2 => Route::One(u8::pull_from(buf)),
            3 => Route::Any(Flavor::pull_from(buf)),
            4 => Route::All(Flavor::pull_from(buf)),
            _ => Route::None
        }
    }

    fn size_in_buffer(&self) -> usize {
        match *self {
            Route::None => size_of::<u8>(),
            Route::Local => size_of::<u8>(),
            Route::One(_) => size_of::<u8>() + size_of::<u8>(),
            Route::Any(_) => size_of::<u8>() + size_of::<u8>(),
            Route::All(_) => size_of::<u8>() + size_of::<u8>(),
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

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Flavor {
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
    Watchtower = 23,
}

impl Bufferable for Flavor {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        (*self as u8).push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let flavor = u8::pull_from(buf);
        match flavor {
            c if c == Flavor::Archive as u8 => Flavor::Archive,
            c if c == Flavor::Bazaar as u8 => Flavor::Bazaar,
            c if c == Flavor::Courtyard as u8 => Flavor::Courtyard,
            c if c == Flavor::Drawbridge as u8 => Flavor::Drawbridge,
            c if c == Flavor::Forum as u8 => Flavor::Forum,
            c if c == Flavor::Gate as u8 => Flavor::Gate,
            c if c == Flavor::Hall as u8 => Flavor::Hall,
            c if c == Flavor::Jail as u8 => Flavor::Jail,
            c if c == Flavor::Lookout as u8 => Flavor::Lookout,
            c if c == Flavor::Vagabond as u8 => Flavor::Vagabond,
            c if c == Flavor::Watchtower as u8 => Flavor::Watchtower,
            _ => Flavor::NoOp
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>()
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Command {
    NoOp,
    Register,
    Authorize,
    Hello,
    UserAttr,
    Chat,
    DM,
    InvGen,
    InvList,
    GameStart,
    GameBuild,
    GameEnd,
}

impl Bufferable for Command {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        (*self as u8).push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let command = u8::pull_from(buf);
        match command {
            c if c == Command::Register as u8 => Command::Register,
            c if c == Command::Authorize as u8 => Command::Authorize,
            c if c == Command::Hello as u8 => Command::Hello,
            c if c == Command::UserAttr as u8 => Command::UserAttr,
            c if c == Command::Chat as u8 => Command::Chat,
            c if c == Command::DM as u8 => Command::DM,
            c if c == Command::InvGen as u8 => Command::InvGen,
            c if c == Command::InvList as u8 => Command::InvList,
            c if c == Command::GameStart as u8 => Command::GameStart,
            c if c == Command::GameBuild as u8 => Command::GameBuild,
            c if c == Command::GameEnd as u8 => Command::GameEnd,
            _ => Command::NoOp
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<u8>()
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use strum::IntoEnumIterator;

        use crate::op::{Command, Route};
        use crate::op::Flavor;
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
