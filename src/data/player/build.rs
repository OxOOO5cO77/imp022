#![allow(clippy::upper_case_acronyms)]

use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Market {
    Any,
    Consumer,
    Enthusiast,
    Prosumer,
    Professional,
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum ANT {
    Any,
    EXM(Market),
    NetTECH(Market),
    TransGlobal(Market),
    Uplink(Market),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum BRD {
    Any,
    Axis(Market),
    PeriPeri(Market),
    SilPath(Market),
    Wasbleibt(Market),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum CPU {
    Any,
    CCC(Market),
    Orbital(Market),
    RiscFree(Market),
    Visor(Market),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum DSC {
    Any,
    Evoke(Market),
    Kollectiv(Market),
    Vault(Market),
    Warehaus(Market),
}

#[derive(Clone, Copy, Deserialize)]
pub(crate) enum Build {
    Any,
    ANT(ANT),
    BRD(BRD),
    CPU(CPU),
    DSC(DSC),
}
