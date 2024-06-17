use serde::{Deserialize, Serialize};

pub type NumberType = u8;
pub type ValueType = u8;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Academic {
    Any,
    CompSci,
    Cybernetics,
    Engineering,
    Theoretical,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Bureaucratic {
    Any,
    Africa,
    Americas,
    Asia,
    Europe,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Corporate {
    Any,
    Consumer,
    Entertainment,
    Industrial,
    Military,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Decentralized {
    Any,
    Activist,
    Enthusiast,
    Freelance,
    OpenSource,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Institution {
    Any,
    Academic(Academic),
    Bureaucratic(Bureaucratic),
    Corporate(Corporate),
    Decentralized(Decentralized),
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Developer {
    Any,
    Art,
    Production,
    Programming,
    QA,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum IT {
    Any,
    DevOps,
    Hardware,
    Security,
    Support,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum People {
    Any,
    Accounting,
    Admin,
    HR,
    Marketing,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Physical {
    Any,
    Maintenance,
    Security,
    Supply,
    Trades,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Role {
    Any,
    Developer(Developer),
    IT(IT),
    People(People),
    Physical(Physical),
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Office {
    Any,
    Campus,
    Ephemeral,
    Satellite,
    Tower,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Public {
    Any,
    Commercial,
    Education,
    Hospitality,
    Municipal,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Residence {
    Any,
    Apartment,
    Detached,
    Hotel,
    Shared,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Unauthorized {
    Any,
    Infrastructure,
    Office,
    Public,
    Residential,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Location {
    Any,
    Office(Office),
    Public(Public),
    Residence(Residence),
    Unauthorized(Unauthorized),
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Consumer {
    Any,
    Casual,
    Content,
    Gaming,
    Productivity,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Fringe {
    Any,
    Exotic,
    Niche,
    Retro,
    Source,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Hardened {
    Any,
    Anonymous,
    Crypto,
    Government,
    Industry,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Restricted {
    Any,
    Access,
    Distribution,
    Install,
    Use,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Distro {
    Any,
    Consumer(Consumer),
    Fringe(Fringe),
    Hardened(Hardened),
    Restricted(Restricted),
}

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Category {
    #[default] Any,
    Institution(Institution),
    Role(Role),
    Location(Location),
    Distro(Distro),
}
