use std::mem::discriminant;
use serde::Deserialize;
use crate::data::card::CardSlot;
use crate::data::player_builder::PlayerPart;

#[derive(Clone, Deserialize)]
pub(crate) enum Academic { Any, CompSci, Cybernetics, Engineering, Theoretical, }

#[derive(Clone, Deserialize)]
pub(crate) enum Bureaucratic { Any, Africa, Americas, Asia, Europe, }

#[derive(Clone, Deserialize)]
pub(crate) enum Corporate { Any, Consumer, Entertainment, Industrial, Military, }

#[derive(Clone, Deserialize)]
pub(crate) enum Decentralized { Any, Activist, Enthusiast, Freelance, OpenSource, }

#[derive(Clone, Deserialize)]
pub(crate) enum Institution {
    Any,
    Academic(Academic),
    Bureaucratic(Bureaucratic),
    Corporate(Corporate),
    Decentralized(Decentralized),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Developer { Any, Art, Production, Programming, QA, }

#[derive(Clone, Deserialize)]
pub(crate) enum IT { Any, DevOps, Hardware, Security, Support, }

#[derive(Clone, Deserialize)]
pub(crate) enum People { Any, Accounting, Admin, HR, Marketing, }

#[derive(Clone, Deserialize)]
pub(crate) enum Physical { Any, Maintenance, Security, Supply, Trades, }

#[derive(Clone, Deserialize)]
pub(crate) enum Role {
    Any,
    Developer(Developer),
    IT(IT),
    People(People),
    Physical(Physical),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Office { Any, Campus, Ephemeral, Satellite, Tower, }

#[derive(Clone, Deserialize)]
pub(crate) enum Public { Any, Commercial, Education, Gastronomy, Municipal, }

#[derive(Clone, Deserialize)]
pub(crate) enum Residence { Any, Apartment, Detached, Hotel, Shared, }

#[derive(Clone, Deserialize)]
pub(crate) enum Unauthorized { Any, Infrastructure, Office, Public, Residential, }

#[derive(Clone, Deserialize)]
pub(crate) enum Location {
    Any,
    Office(Office),
    Public(Public),
    Residence(Residence),
    Unauthorized(Unauthorized),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Consumer { Any, Casual, Content, Gaming, Productivity, }

#[derive(Clone, Deserialize)]
pub(crate) enum Fringe {Any, Exotic, Niche, Retro, Source, }

#[derive(Clone, Deserialize)]
pub(crate) enum Hardened { Any, Anonymous, Crypto, Government, Industry, }

#[derive(Clone, Deserialize)]
pub(crate) enum Restricted { Any, Access, Distribution, Install, Use, }

#[derive(Clone, Deserialize)]
pub(crate) enum Distro {
    Any,
    Consumer(Consumer),
    Fringe(Fringe),
    Hardened(Hardened),
    Restricted(Restricted),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Category {
    Any,
    Institution(Institution),
    Role(Role),
    Location(Location),
    Distro(Distro),
}

#[derive(Clone, Deserialize)]
pub(crate) struct CategoryInstance {
    category: Category,
    pub(crate) title: String,
    pub(crate) cards: Vec<CardSlot>,
}

impl CategoryInstance {
    pub(crate) fn is(&self, other: &Category) -> bool {
        discriminant(&self.category) == discriminant(other)
    }

    pub(crate) fn from_parts(build: &PlayerPart, values: &PlayerPart) -> [(CategoryInstance, u8); 4] {
        [
            (build.category[0].clone(), values.values[0]),
            (build.category[1].clone(), values.values[1]),
            (build.category[2].clone(), values.values[2]),
            (build.category[3].clone(), values.values[3]),
        ]
    }
}
