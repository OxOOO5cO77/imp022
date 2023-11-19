#![allow(clippy::upper_case_acronyms)]

use std::mem::discriminant;
use serde::Deserialize;
use crate::data::card::CardSlot;
use crate::data::player_builder::PlayerPart;

#[derive(Clone, Deserialize)]
pub(crate) enum Market { Any, Consumer, Enthusiast, Prosumer, Professional }

#[derive(Clone, Deserialize)]
pub(crate) enum ANT {
    Any,
    EXM(Market),
    NetTECH(Market),
    TransGlobal(Market),
    Uplink(Market),
}

#[derive(Clone, Deserialize)]
pub(crate) enum BRD {
    Any,
    Axis(Market),
    PeriPeri(Market),
    SilPath(Market),
    Wasbleibt(Market),
}

#[derive(Clone, Deserialize)]
pub(crate) enum CPU {
    Any,
    CCC(Market),
    Orbital(Market),
    RiscFree(Market),
    Visor(Market),
}

#[derive(Clone, Deserialize)]
pub(crate) enum DSC {
    Any,
    Evoke(Market),
    Kollectiv(Market),
    Vault(Market),
    Warehaus(Market),
}

#[derive(Clone, Deserialize)]
pub(crate) enum Build {
    Any,
    ANT(ANT),
    BRD(BRD),
    CPU(CPU),
    DSC(DSC),
}

#[derive(Clone, Deserialize)]
pub(crate) struct BuildInstance {
    build: Build,
    pub(crate) title: String,
    pub(crate) cards: Vec<CardSlot>,
}

impl BuildInstance {
    pub(crate) fn is(&self, other: &Build) -> bool {
        discriminant(&self.build) == discriminant(other)
    }

    pub(crate) fn from_parts(build: &PlayerPart, values: &PlayerPart) -> [(BuildInstance, u8); 4] {
        [
            (build.build[0].clone(), values.values[0]),
            (build.build[1].clone(), values.values[1]),
            (build.build[2].clone(), values.values[2]),
            (build.build[3].clone(), values.values[3]),
        ]
    }
}
