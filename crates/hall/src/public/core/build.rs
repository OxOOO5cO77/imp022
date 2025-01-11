#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

pub type BuildNumberType = u8;
pub type BuildValueType = u8;

pub type CompanyType = u8;
pub type MarketType = u8;

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Build {
    #[default]
    Any,
    ANT(CompanyType, MarketType),
    BRD(CompanyType, MarketType),
    CPU(CompanyType, MarketType),
    DSK(CompanyType, MarketType),
}
