use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Academic {
    Any,
    CompSci,
    Cybernetics,
    Engineering,
    Theoretical,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Bureaucratic {
    Any,
    Africa,
    Americas,
    Asia,
    Europe,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Corporate {
    Any,
    Consumer,
    Entertainment,
    Industrial,
    Military,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Decentralized {
    Any,
    Activist,
    Enthusiast,
    Freelance,
    OpenSource,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Institution {
    Any,
    Academic(Academic),
    Bureaucratic(Bureaucratic),
    Corporate(Corporate),
    Decentralized(Decentralized),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Developer {
    Any,
    Art,
    Production,
    Programming,
    QA,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum IT {
    Any,
    DevOps,
    Hardware,
    Security,
    Support,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum People {
    Any,
    Accounting,
    Admin,
    HR,
    Marketing,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Physical {
    Any,
    Maintenance,
    Security,
    Supply,
    Trades,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Role {
    Any,
    Developer(Developer),
    IT(IT),
    People(People),
    Physical(Physical),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Office {
    Any,
    Campus,
    Ephemeral,
    Satellite,
    Tower,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Public {
    Any,
    Commercial,
    Education,
    Gastronomy,
    Municipal,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Residence {
    Any,
    Apartment,
    Detached,
    Hotel,
    Shared,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Unauthorized {
    Any,
    Infrastructure,
    Office,
    Public,
    Residential,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Location {
    Any,
    Office(Office),
    Public(Public),
    Residence(Residence),
    Unauthorized(Unauthorized),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Consumer {
    Any,
    Casual,
    Content,
    Gaming,
    Productivity,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Fringe {
    Any,
    Exotic,
    Niche,
    Retro,
    Source,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Hardened {
    Any,
    Anonymous,
    Crypto,
    Government,
    Industry,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Restricted {
    Any,
    Access,
    Distribution,
    Install,
    Use,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Distro {
    Any,
    Consumer(Consumer),
    Fringe(Fringe),
    Hardened(Hardened),
    Restricted(Restricted),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Category {
    Any,
    Institution(Institution),
    Role(Role),
    Location(Location),
    Distro(Distro),
}
