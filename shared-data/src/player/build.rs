#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

pub type NumberType = u8;
pub type ValueType = u8;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Market {
    Any,
    Consumer,
    Enthusiast,
    Prosumer,
    Professional,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum ANT {
    Any,
    EXM(Market),
    NetTECH(Market),
    TransGlobal(Market),
    Uplink(Market),
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum BRD {
    Any,
    Axis(Market),
    PeriPeri(Market),
    SilPath(Market),
    Wasbleibt(Market),
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum CPU {
    Any,
    CCC(Market),
    Orbital(Market),
    RiscFree(Market),
    Visor(Market),
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum DSC {
    Any,
    Evoke(Market),
    Kollectiv(Market),
    Vault(Market),
    Warehaus(Market),
}

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Build {
    #[default] Any,
    ANT(ANT),
    BRD(BRD),
    CPU(CPU),
    DSC(DSC),
}
