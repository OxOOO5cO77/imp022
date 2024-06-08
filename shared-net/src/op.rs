#[cfg(test)]
use strum_macros::EnumIter;

use crate::VSizedBuffer;

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

impl From<u8> for Flavor {
    fn from(flavor: u8) -> Self {
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
}

impl VSizedBuffer {
    pub fn push_flavor(&mut self, push: Flavor) -> &mut Self {
        self.push_u8(&(push as u8))
    }
    pub fn pull_flavor(&mut self) -> Flavor {
        Flavor::from(self.pull_u8())
    }
    pub fn xfer_flavor(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u8(&push.pull_u8())
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Route {
    NoOp = 0,
    Local = 1,
    One = 2,
    Any = 3,
    All = 4,
}

impl From<u8> for Route {
    fn from(route: u8) -> Self {
        match route {
            c if c == Route::Local as u8 => Route::Local,
            c if c == Route::One as u8 => Route::One,
            c if c == Route::Any as u8 => Route::Any,
            c if c == Route::All as u8 => Route::All,
            _ => Route::NoOp
        }
    }
}

impl VSizedBuffer {
    pub fn push_route(&mut self, push: Route) -> &mut Self {
        self.push_u8(&(push as u8))
    }
    pub fn pull_route(&mut self) -> Route {
        Route::from(self.pull_u8())
    }
    pub fn xfer_route(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u8(&push.pull_u8())
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

impl From<u8> for Command {
    fn from(command: u8) -> Self {
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
}

impl VSizedBuffer {
    pub fn push_command(&mut self, push: Command) -> &mut Self {
        self.push_u8(&(push as u8))
    }
    pub fn pull_command(&mut self) -> Command {
        Command::from(self.pull_u8())
    }
    pub fn xfer_command(&mut self, push: &mut VSizedBuffer) -> &mut Self {
        self.push_u8(&push.pull_u8())
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use strum::IntoEnumIterator;

        use crate::op::Command;
        use crate::op::Flavor;
        use crate::op::Route;
        use crate::VSizedBuffer;

        #[test]
        fn flavor() {
            for flavor in Flavor::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push_flavor(flavor);
                buf1.push_flavor(flavor);

                assert_eq!(flavor, buf1.pull_flavor());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer_flavor(&mut buf1);

                assert_eq!(flavor, buf2.pull_flavor());
            }
        }

        #[test]
        fn route() {
            for route in Route::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push_route(route);
                buf1.push_route(route);

                assert_eq!(route, buf1.pull_route());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer_route(&mut buf1);

                assert_eq!(route, buf2.pull_route());
            }
        }

        #[test]
        fn command() {
            for command in Command::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push_command(command);
                buf1.push_command(command);

                assert_eq!(command, buf1.pull_command());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer_command(&mut buf1);

                assert_eq!(command, buf2.pull_command());
            }
        }
    }
}
