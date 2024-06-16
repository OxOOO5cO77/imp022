use std::mem::size_of;

use serde::{Deserialize, Serialize};

use shared_data::player::build;
use shared_data::player::build::{ANT, BRD, Build, CPU, DSC, Market};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedBuildType = u32;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerBuild {
    pub number: build::NumberType,
    pub build: Build,
    pub value: build::ValueType,
}

const BITS_FOR_BUILD: PackedBuildType = 16;
const BITS_FOR_NUMBER: PackedBuildType = 8;
const BITS_FOR_VALUE: PackedBuildType = 8;

const SHIFT_FOR_BUILD: PackedBuildType = 0;
const SHIFT_FOR_NUMBER: PackedBuildType = SHIFT_FOR_BUILD + BITS_FOR_BUILD;
const SHIFT_FOR_VALUE: PackedBuildType = SHIFT_FOR_NUMBER + BITS_FOR_NUMBER;

const MASK_FOR_BUILD: PackedBuildType = (1 << BITS_FOR_BUILD) - 1;
const MASK_FOR_NUMBER: PackedBuildType = (1 << BITS_FOR_NUMBER) - 1;
const MASK_FOR_VALUE: PackedBuildType = (1 << BITS_FOR_VALUE) - 1;

const BITS_FOR_BUILD_PART: PackedBuildType = 5;
const SHIFT_FOR_BUILD_MARKET: PackedBuildType = 0;
const SHIFT_FOR_BUILD_COMPANY: PackedBuildType = SHIFT_FOR_BUILD_MARKET + BITS_FOR_BUILD_PART;
const SHIFT_FOR_BUILD_KIND: PackedBuildType = SHIFT_FOR_BUILD_COMPANY + BITS_FOR_BUILD_PART;
const MASK_FOR_BUILD_PART: PackedBuildType = (1 << BITS_FOR_BUILD_PART) - 1;

impl PlayerBuild {
    fn pack_market(kind: PackedBuildType, company: PackedBuildType, market_enum: Market) -> PackedBuildType {
        let market = match market_enum {
            Market::Any => 0,
            Market::Consumer => 1,
            Market::Enthusiast => 2,
            Market::Prosumer => 3,
            Market::Professional => 4,
        };
        (kind << SHIFT_FOR_BUILD_KIND) | (company << SHIFT_FOR_BUILD_COMPANY) | (market << SHIFT_FOR_BUILD_MARKET)
    }

    fn pack_ant(kind: PackedBuildType, ant: ANT) -> PackedBuildType {
        match ant {
            ANT::Any => kind << SHIFT_FOR_BUILD_KIND,
            ANT::EXM(market) => Self::pack_market(kind, 1, market),
            ANT::NetTECH(market) => Self::pack_market(kind, 2, market),
            ANT::TransGlobal(market) => Self::pack_market(kind, 3, market),
            ANT::Uplink(market) => Self::pack_market(kind, 4, market),
        }
    }

    fn pack_brd(kind: PackedBuildType, brd: BRD) -> PackedBuildType {
        match brd {
            BRD::Any => kind << SHIFT_FOR_BUILD_KIND,
            BRD::Axis(market) => Self::pack_market(kind, 1, market),
            BRD::PeriPeri(market) => Self::pack_market(kind, 2, market),
            BRD::SilPath(market) => Self::pack_market(kind, 3, market),
            BRD::Wasbleibt(market) => Self::pack_market(kind, 4, market),
        }
    }

    fn pack_cpu(kind: PackedBuildType, cpu: CPU) -> PackedBuildType {
        match cpu {
            CPU::Any => kind << SHIFT_FOR_BUILD_KIND,
            CPU::CCC(market) => Self::pack_market(kind, 1, market),
            CPU::Orbital(market) => Self::pack_market(kind, 2, market),
            CPU::RiscFree(market) => Self::pack_market(kind, 3, market),
            CPU::Visor(market) => Self::pack_market(kind, 4, market),
        }
    }

    fn pack_dsc(kind: PackedBuildType, dsc: DSC) -> PackedBuildType {
        match dsc {
            DSC::Any => kind << SHIFT_FOR_BUILD_KIND,
            DSC::Evoke(market) => Self::pack_market(kind, 1, market),
            DSC::Kollectiv(market) => Self::pack_market(kind, 2, market),
            DSC::Vault(market) => Self::pack_market(kind, 3, market),
            DSC::Warehaus(market) => Self::pack_market(kind, 4, market),
        }
    }

    fn pack_build(&self) -> PackedBuildType {
        match self.build {
            Build::Any => 0,
            Build::ANT(ant) => Self::pack_ant(1, ant),
            Build::BRD(brd) => Self::pack_brd(2, brd),
            Build::CPU(cpu) => Self::pack_cpu(3, cpu),
            Build::DSC(dsc) => Self::pack_dsc(4, dsc),
        }
    }

    fn unpack_market(market: PackedBuildType) -> Market {
        match market {
            1 => Market::Consumer,
            2 => Market::Enthusiast,
            3 => Market::Prosumer,
            4 => Market::Professional,
            _ => Market::Any,
        }
    }

    fn unpack_ant(company: PackedBuildType, market: Market) -> Build {
        match company {
            1 => Build::ANT(ANT::EXM(market)),
            2 => Build::ANT(ANT::NetTECH(market)),
            3 => Build::ANT(ANT::TransGlobal(market)),
            4 => Build::ANT(ANT::Uplink(market)),
            _ => Build::ANT(ANT::Any),
        }
    }

    fn unpack_brd(company: PackedBuildType, market: Market) -> Build {
        match company {
            1 => Build::BRD(BRD::Axis(market)),
            2 => Build::BRD(BRD::PeriPeri(market)),
            3 => Build::BRD(BRD::SilPath(market)),
            4 => Build::BRD(BRD::Wasbleibt(market)),
            _ => Build::BRD(BRD::Any),
        }
    }
    fn unpack_cpu(company: PackedBuildType, market: Market) -> Build {
        match company {
            1 => Build::CPU(CPU::CCC(market)),
            2 => Build::CPU(CPU::Orbital(market)),
            3 => Build::CPU(CPU::RiscFree(market)),
            4 => Build::CPU(CPU::Visor(market)),
            _ => Build::CPU(CPU::Any),
        }
    }
    fn unpack_dsc(company: PackedBuildType, market: Market) -> Build {
        match company {
            1 => Build::DSC(DSC::Evoke(market)),
            2 => Build::DSC(DSC::Kollectiv(market)),
            3 => Build::DSC(DSC::Vault(market)),
            4 => Build::DSC(DSC::Warehaus(market)),
            _ => Build::DSC(DSC::Any),
        }
    }

    fn unpack_build(packed: PackedBuildType) -> Build {
        let kind = (packed >> SHIFT_FOR_BUILD_KIND) & MASK_FOR_BUILD_PART;
        let company = (packed >> SHIFT_FOR_BUILD_COMPANY) & MASK_FOR_BUILD_PART;
        let market = (packed >> SHIFT_FOR_BUILD_MARKET) & MASK_FOR_BUILD_PART;

        let market_enum = Self::unpack_market(market);
        match kind {
            1 => Self::unpack_ant(company, market_enum),
            2 => Self::unpack_brd(company, market_enum),
            3 => Self::unpack_cpu(company, market_enum),
            4 => Self::unpack_dsc(company, market_enum),
            _ => Build::Any,
        }
    }

    fn pack(&self) -> PackedBuildType {
        let packed_build = self.pack_build();
        let packed_number = self.number as PackedBuildType;
        let packed_value = self.value as PackedBuildType;
        (packed_build << SHIFT_FOR_BUILD) | (packed_number << SHIFT_FOR_NUMBER) | (packed_value << SHIFT_FOR_VALUE)
    }

    fn unpack(packed: PackedBuildType) -> Self {
        let packed_build = (packed >> SHIFT_FOR_BUILD) & MASK_FOR_BUILD;
        let packed_number = (packed >> SHIFT_FOR_NUMBER) & MASK_FOR_NUMBER;
        let packed_value = (packed >> SHIFT_FOR_VALUE) & MASK_FOR_VALUE;
        Self {
            number: packed_number as build::NumberType,
            build: Self::unpack_build(packed_build),
            value: packed_value as build::ValueType,
        }
    }
}

impl Bufferable for PlayerBuild {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.pack().push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        Self::unpack(PackedBuildType::pull_from(buf))
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedBuildType>()
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::build::{Build, CPU, Market};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;
    use crate::data::player_build::PlayerBuild;

    #[test]
    fn test_player_build() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerBuild {
            number: 123,
            build: Build::CPU(CPU::RiscFree(Market::Enthusiast)),
            value: 9,
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerBuild>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
