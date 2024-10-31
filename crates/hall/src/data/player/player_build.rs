use std::mem::size_of;

use serde::{Deserialize, Serialize};

use shared_data::player::build::{Build, NumberType, MarketType, CompanyType, ValueType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedBuildType = u32;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerBuild {
    pub number: NumberType,
    pub build: Build,
    pub value: ValueType,
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
    fn pack_build(&self) -> PackedBuildType {
        let (kind, company, market) = match self.build {
            Build::Any => (0, 0, 0),
            Build::ANT(company, market) => (1, company, market),
            Build::BRD(company, market) => (2, company, market),
            Build::CPU(company, market) => (3, company, market),
            Build::DSK(company, market) => (4, company, market),
        };
        let (kind, company, market) = (kind, company as PackedBuildType, market as PackedBuildType);
        (kind << SHIFT_FOR_BUILD_KIND) | (company << SHIFT_FOR_BUILD_COMPANY) | (market << SHIFT_FOR_BUILD_MARKET)
    }

    fn unpack_build(packed: PackedBuildType) -> Build {
        let kind = (packed >> SHIFT_FOR_BUILD_KIND) & MASK_FOR_BUILD_PART;
        let company = ((packed >> SHIFT_FOR_BUILD_COMPANY) & MASK_FOR_BUILD_PART) as CompanyType;
        let market = ((packed >> SHIFT_FOR_BUILD_MARKET) & MASK_FOR_BUILD_PART) as MarketType;

        match kind {
            1 => Build::ANT(company, market),
            2 => Build::BRD(company, market),
            3 => Build::CPU(company, market),
            4 => Build::DSK(company, market),
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
            number: packed_number as NumberType,
            build: Self::unpack_build(packed_build),
            value: packed_value as ValueType,
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
    use crate::data::player::player_build::PlayerBuild;
    use shared_data::player::build::Build;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_player_build() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerBuild {
            number: 123,
            build: Build::CPU(3,12),
            value: 9,
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerBuild>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
