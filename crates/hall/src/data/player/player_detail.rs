use std::mem::size_of;

use serde::{Deserialize, Serialize};
use shared_data::player::detail::{Detail, GeneralType, NumberType, SpecificType, ValueType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedDetailType = u64;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerDetail {
    pub number: NumberType,
    pub detail: Detail,
    pub value: ValueType,
}


const BITS_FOR_DETAIL: PackedDetailType = 24;
const BITS_FOR_NUMBER: PackedDetailType = 8;
const BITS_FOR_VALUE: PackedDetailType = 8;

const SHIFT_FOR_DETAIL: PackedDetailType = 0;
const SHIFT_FOR_NUMBER: PackedDetailType = SHIFT_FOR_DETAIL + BITS_FOR_DETAIL;
const SHIFT_FOR_VALUE: PackedDetailType = SHIFT_FOR_NUMBER + BITS_FOR_NUMBER;

const MASK_FOR_DETAIL: PackedDetailType = (1 << BITS_FOR_DETAIL) - 1;
const MASK_FOR_NUMBER: PackedDetailType = (1 << BITS_FOR_NUMBER) - 1;
const MASK_FOR_VALUE: PackedDetailType = (1 << BITS_FOR_VALUE) - 1;

const BITS_FOR_DETAIL_PART: PackedDetailType = 8;
const SHIFT_FOR_DETAIL_SPECIFIC: PackedDetailType = 0;
const SHIFT_FOR_DETAIL_GENERAL: PackedDetailType = SHIFT_FOR_DETAIL_SPECIFIC + BITS_FOR_DETAIL_PART;
const SHIFT_FOR_DETAIL_KIND: PackedDetailType = SHIFT_FOR_DETAIL_GENERAL + BITS_FOR_DETAIL_PART;
const MASK_FOR_DETAIL_PART: PackedDetailType = (1 << BITS_FOR_DETAIL_PART) - 1;

impl PlayerDetail {
    fn pack_detail(&self) -> PackedDetailType {
        let (kind, general, specific) = match self.detail {
            Detail::Any => (0, 0, 0),
            Detail::Institution(general, specific) => (1, general, specific),
            Detail::Role(general, specific) => (2, general, specific),
            Detail::Location(general, specific) => (3, general, specific),
            Detail::Distro(general, specific) => (4, general, specific),
        };
        let (kind, general, specific) = (kind, general as PackedDetailType, specific as PackedDetailType);
        (kind << SHIFT_FOR_DETAIL_KIND) | (general << SHIFT_FOR_DETAIL_GENERAL) | (specific << SHIFT_FOR_DETAIL_SPECIFIC)
    }

    fn unpack_detail(packed: PackedDetailType) -> Detail {
        let kind = (packed >> SHIFT_FOR_DETAIL_KIND) & MASK_FOR_DETAIL_PART;
        let general = ((packed >> SHIFT_FOR_DETAIL_GENERAL) & MASK_FOR_DETAIL_PART) as GeneralType;
        let specific = ((packed >> SHIFT_FOR_DETAIL_SPECIFIC) & MASK_FOR_DETAIL_PART) as SpecificType;

        match kind {
            1 => Detail::Institution(general, specific),
            2 => Detail::Role(general, specific),
            3 => Detail::Location(general, specific),
            4 => Detail::Distro(general, specific),
            _ => Detail::Any,
        }
    }

    fn pack(&self) -> PackedDetailType {
        let packed_build = self.pack_detail();
        let packed_number = self.number as PackedDetailType;
        let packed_value = self.value as PackedDetailType;
        (packed_build << SHIFT_FOR_DETAIL) | (packed_number << SHIFT_FOR_NUMBER) | (packed_value << SHIFT_FOR_VALUE)
    }

    fn unpack(packed: PackedDetailType) -> Self {
        let packed_build = (packed >> SHIFT_FOR_DETAIL) & MASK_FOR_DETAIL;
        let packed_number = (packed >> SHIFT_FOR_NUMBER) & MASK_FOR_NUMBER;
        let packed_value = (packed >> SHIFT_FOR_VALUE) & MASK_FOR_VALUE;
        Self {
            number: packed_number as NumberType,
            detail: Self::unpack_detail(packed_build),
            value: packed_value as ValueType,
        }
    }
}

impl Bufferable for PlayerDetail {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.pack().push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        Self::unpack(PackedDetailType::pull_from(buf))
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedDetailType>()
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::detail::Detail;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player::player_detail::PlayerDetail;

    #[test]
    fn test_player_detail() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerDetail {
            number: 123,
            detail: Detail::Role(4,16),
            value: 9,
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerDetail>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(result, orig);
    }
}
