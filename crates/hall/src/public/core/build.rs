#![allow(clippy::upper_case_acronyms)]

use serde::{Deserialize, Serialize};

use crate::core::{CompanyType, MarketType};

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Build {
    #[default]
    Any,
    ANT(CompanyType, MarketType),
    BRD(CompanyType, MarketType),
    CPU(CompanyType, MarketType),
    DSK(CompanyType, MarketType),
}
