use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Academic {
    Any,
    CompSci,
    Cybernetics,
    Engineering,
    Theoretical,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Bureaucratic {
    Any,
    Africa,
    Americas,
    Asia,
    Europe,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Corporate {
    Any,
    Consumer,
    Entertainment,
    Industrial,
    Military,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Decentralized {
    Any,
    Activist,
    Enthusiast,
    Freelance,
    OpenSource,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Institution {
    Any,
    Academic(Academic),
    Bureaucratic(Bureaucratic),
    Corporate(Corporate),
    Decentralized(Decentralized),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Developer {
    Any,
    Art,
    Production,
    Programming,
    QA,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum IT {
    Any,
    DevOps,
    Hardware,
    Security,
    Support,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum People {
    Any,
    Accounting,
    Admin,
    HR,
    Marketing,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Physical {
    Any,
    Maintenance,
    Security,
    Supply,
    Trades,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    Any,
    Developer(Developer),
    IT(IT),
    People(People),
    Physical(Physical),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Office {
    Any,
    Campus,
    Ephemeral,
    Satellite,
    Tower,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Public {
    Any,
    Commercial,
    Education,
    Gastronomy,
    Municipal,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Residence {
    Any,
    Apartment,
    Detached,
    Hotel,
    Shared,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Unauthorized {
    Any,
    Infrastructure,
    Office,
    Public,
    Residential,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Location {
    Any,
    Office(Office),
    Public(Public),
    Residence(Residence),
    Unauthorized(Unauthorized),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Consumer {
    Any,
    Casual,
    Content,
    Gaming,
    Productivity,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Fringe {
    Any,
    Exotic,
    Niche,
    Retro,
    Source,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Hardened {
    Any,
    Anonymous,
    Crypto,
    Government,
    Industry,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Restricted {
    Any,
    Access,
    Distribution,
    Install,
    Use,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Distro {
    Any,
    Consumer(Consumer),
    Fringe(Fringe),
    Hardened(Hardened),
    Restricted(Restricted),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Category {
    Any,
    Institution(Institution),
    Role(Role),
    Location(Location),
    Distro(Distro),
}
