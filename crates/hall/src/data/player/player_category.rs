use std::mem::size_of;

use serde::{Deserialize, Serialize};

use shared_data::player::category;
use shared_data::player::category::{Academic, Bureaucratic, Category, Consumer, Corporate, Decentralized, Developer, Distro, Fringe, Hardened, Institution, IT, Location, Office, People, Physical, Public, Residence, Restricted, Role, Unauthorized};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedCategoryType = u32;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerCategory {
    pub number: category::NumberType,
    pub category: Category,
    pub value: category::ValueType,
}


const BITS_FOR_CATEGORY: PackedCategoryType = 16;
const BITS_FOR_NUMBER: PackedCategoryType = 8;
const BITS_FOR_VALUE: PackedCategoryType = 8;

const SHIFT_FOR_CATEGORY: PackedCategoryType = 0;
const SHIFT_FOR_NUMBER: PackedCategoryType = SHIFT_FOR_CATEGORY + BITS_FOR_CATEGORY;
const SHIFT_FOR_VALUE: PackedCategoryType = SHIFT_FOR_NUMBER + BITS_FOR_NUMBER;

const MASK_FOR_CATEGORY: PackedCategoryType = (1 << BITS_FOR_CATEGORY) - 1;
const MASK_FOR_NUMBER: PackedCategoryType = (1 << BITS_FOR_NUMBER) - 1;
const MASK_FOR_VALUE: PackedCategoryType = (1 << BITS_FOR_VALUE) - 1;

const BITS_FOR_CATEGORY_PART: PackedCategoryType = 5;
const SHIFT_FOR_CATEGORY_SPECIFIC: PackedCategoryType = 0;
const SHIFT_FOR_CATEGORY_GENERAL: PackedCategoryType = SHIFT_FOR_CATEGORY_SPECIFIC + BITS_FOR_CATEGORY_PART;
const SHIFT_FOR_CATEGORY_KIND: PackedCategoryType = SHIFT_FOR_CATEGORY_GENERAL + BITS_FOR_CATEGORY_PART;
const MASK_FOR_CATEGORY_PART: PackedCategoryType = (1 << BITS_FOR_CATEGORY_PART) - 1;

impl PlayerCategory {
    fn pack_ins(ins: Institution) -> (PackedCategoryType, PackedCategoryType) {
        match ins {
            Institution::Any => (0, 0),
            Institution::Academic(academic) => (1, academic as PackedCategoryType),
            Institution::Bureaucratic(bureaucratic) => (2, bureaucratic as PackedCategoryType),
            Institution::Corporate(corporate) => (3, corporate as PackedCategoryType),
            Institution::Decentralized(decentralized) => (4, decentralized as PackedCategoryType),
        }
    }
    fn pack_rol(rol: Role) -> (PackedCategoryType, PackedCategoryType) {
        match rol {
            Role::Any => (0, 0),
            Role::Developer(developer) => (1, developer as PackedCategoryType),
            Role::IT(it) => (2, it as PackedCategoryType),
            Role::People(people) => (3, people as PackedCategoryType),
            Role::Physical(physical) => (4, physical as PackedCategoryType),
        }
    }
    fn pack_loc(loc: Location) -> (PackedCategoryType, PackedCategoryType) {
        match loc {
            Location::Any => (0, 0),
            Location::Office(office) => (1, office as PackedCategoryType),
            Location::Public(public) => (2, public as PackedCategoryType),
            Location::Residence(residence) => (3, residence as PackedCategoryType),
            Location::Unauthorized(unauthorized) => (4, unauthorized as PackedCategoryType),
        }
    }
    fn pack_dis(dis: Distro) -> (PackedCategoryType, PackedCategoryType) {
        match dis {
            Distro::Any => (0, 0),
            Distro::Consumer(consumer) => (1, consumer as PackedCategoryType),
            Distro::Fringe(fringe) => (2, fringe as PackedCategoryType),
            Distro::Hardened(hardened) => (3, hardened as PackedCategoryType),
            Distro::Restricted(restricted) => (4, restricted as PackedCategoryType),
        }
    }

    fn pack_category(&self) -> PackedCategoryType {
        let (kind, (general, specific)) = match self.category {
            Category::Any => (0, (0, 0)),
            Category::Institution(ins) => (1, Self::pack_ins(ins)),
            Category::Role(rol) => (2, Self::pack_rol(rol)),
            Category::Location(loc) => (3, Self::pack_loc(loc)),
            Category::Distro(dis) => (4, Self::pack_dis(dis)),
        };
        (kind << SHIFT_FOR_CATEGORY_KIND) | (general << SHIFT_FOR_CATEGORY_GENERAL) | (specific << SHIFT_FOR_CATEGORY_SPECIFIC)
    }

    fn unpack_ins_aca(specific: PackedCategoryType) -> Academic {
        match specific {
            1 => Academic::CompSci,
            2 => Academic::Cybernetics,
            3 => Academic::Engineering,
            4 => Academic::Theoretical,
            _ => Academic::Any,
        }
    }
    fn unpack_ins_bur(specific: PackedCategoryType) -> Bureaucratic {
        match specific {
            1 => Bureaucratic::Africa,
            2 => Bureaucratic::Americas,
            3 => Bureaucratic::Asia,
            4 => Bureaucratic::Europe,
            _ => Bureaucratic::Any,
        }
    }
    fn unpack_ins_cor(specific: PackedCategoryType) -> Corporate {
        match specific {
            1 => Corporate::Consumer,
            2 => Corporate::Entertainment,
            3 => Corporate::Industrial,
            4 => Corporate::Military,
            _ => Corporate::Any,
        }
    }
    fn unpack_ins_dec(specific: PackedCategoryType) -> Decentralized {
        match specific {
            1 => Decentralized::Activist,
            2 => Decentralized::Enthusiast,
            3 => Decentralized::Freelance,
            4 => Decentralized::OpenSource,
            _ => Decentralized::Any,
        }
    }

    fn unpack_ins(company: PackedCategoryType, specific: PackedCategoryType) -> Institution {
        match company {
            1 => Institution::Academic(Self::unpack_ins_aca(specific)),
            2 => Institution::Bureaucratic(Self::unpack_ins_bur(specific)),
            3 => Institution::Corporate(Self::unpack_ins_cor(specific)),
            4 => Institution::Decentralized(Self::unpack_ins_dec(specific)),
            _ => Institution::Any,
        }
    }

    fn unpack_rol_dev(specific: PackedCategoryType) -> Developer {
        match specific {
            1 => Developer::Art,
            2 => Developer::Production,
            3 => Developer::Programming,
            4 => Developer::QA,
            _ => Developer::Any,
        }
    }
    fn unpack_rol_it(specific: PackedCategoryType) -> IT {
        match specific {
            1 => IT::DevOps,
            2 => IT::Hardware,
            3 => IT::Security,
            4 => IT::Support,
            _ => IT::Any,
        }
    }
    fn unpack_rol_peo(specific: PackedCategoryType) -> People {
        match specific {
            1 => People::Accounting,
            2 => People::Admin,
            3 => People::HR,
            4 => People::Marketing,
            _ => People::Any,
        }
    }
    fn unpack_rol_phy(specific: PackedCategoryType) -> Physical {
        match specific {
            1 => Physical::Maintenance,
            2 => Physical::Security,
            3 => Physical::Supply,
            4 => Physical::Trades,
            _ => Physical::Any,
        }
    }

    fn unpack_rol(company: PackedCategoryType, specific: PackedCategoryType) -> Role {
        match company {
            1 => Role::Developer(Self::unpack_rol_dev(specific)),
            2 => Role::IT(Self::unpack_rol_it(specific)),
            3 => Role::People(Self::unpack_rol_peo(specific)),
            4 => Role::Physical(Self::unpack_rol_phy(specific)),
            _ => Role::Any,
        }
    }

    fn unpack_loc_off(specific: PackedCategoryType) -> Office {
        match specific {
            1 => Office::Campus,
            2 => Office::Ephemeral,
            3 => Office::Satellite,
            4 => Office::Tower,
            _ => Office::Any,
        }
    }
    fn unpack_loc_pub(specific: PackedCategoryType) -> Public {
        match specific {
            1 => Public::Commercial,
            2 => Public::Education,
            3 => Public::Hospitality,
            4 => Public::Municipal,
            _ => Public::Any,
        }
    }
    fn unpack_loc_res(specific: PackedCategoryType) -> Residence {
        match specific {
            1 => Residence::Apartment,
            2 => Residence::Detached,
            3 => Residence::Hotel,
            4 => Residence::Shared,
            _ => Residence::Any,
        }
    }
    fn unpack_loc_una(specific: PackedCategoryType) -> Unauthorized {
        match specific {
            1 => Unauthorized::Infrastructure,
            2 => Unauthorized::Office,
            3 => Unauthorized::Public,
            4 => Unauthorized::Residential,
            _ => Unauthorized::Any,
        }
    }


    fn unpack_loc(company: PackedCategoryType, specific: PackedCategoryType) -> Location {
        match company {
            1 => Location::Office(Self::unpack_loc_off(specific)),
            2 => Location::Public(Self::unpack_loc_pub(specific)),
            3 => Location::Residence(Self::unpack_loc_res(specific)),
            4 => Location::Unauthorized(Self::unpack_loc_una(specific)),
            _ => Location::Any,
        }
    }

    fn unpack_dis_con(specific: PackedCategoryType) -> Consumer {
        match specific {
            1 => Consumer::Casual,
            2 => Consumer::Content,
            3 => Consumer::Gaming,
            4 => Consumer::Productivity,
            _ => Consumer::Any,
        }
    }
    fn unpack_dis_fri(specific: PackedCategoryType) -> Fringe {
        match specific {
            1 => Fringe::Exotic,
            2 => Fringe::Niche,
            3 => Fringe::Retro,
            4 => Fringe::Source,
            _ => Fringe::Any,
        }
    }
    fn unpack_dis_har(specific: PackedCategoryType) -> Hardened {
        match specific {
            1 => Hardened::Anonymous,
            2 => Hardened::Crypto,
            3 => Hardened::Government,
            4 => Hardened::Industry,
            _ => Hardened::Any,
        }
    }
    fn unpack_dis_res(specific: PackedCategoryType) -> Restricted {
        match specific {
            1 => Restricted::Access,
            2 => Restricted::Distribution,
            3 => Restricted::Install,
            4 => Restricted::Use,
            _ => Restricted::Any,
        }
    }

    fn unpack_dis(company: PackedCategoryType, specific: PackedCategoryType) -> Distro {
        match company {
            1 => Distro::Consumer(Self::unpack_dis_con(specific)),
            2 => Distro::Fringe(Self::unpack_dis_fri(specific)),
            3 => Distro::Hardened(Self::unpack_dis_har(specific)),
            4 => Distro::Restricted(Self::unpack_dis_res(specific)),
            _ => Distro::Any,
        }
    }

    fn unpack_category(packed: PackedCategoryType) -> Category {
        let kind = (packed >> SHIFT_FOR_CATEGORY_KIND) & MASK_FOR_CATEGORY_PART;
        let general = (packed >> SHIFT_FOR_CATEGORY_GENERAL) & MASK_FOR_CATEGORY_PART;
        let specific = (packed >> SHIFT_FOR_CATEGORY_SPECIFIC) & MASK_FOR_CATEGORY_PART;

        match kind {
            1 => Category::Institution(Self::unpack_ins(general, specific)),
            2 => Category::Role(Self::unpack_rol(general, specific)),
            3 => Category::Location(Self::unpack_loc(general, specific)),
            4 => Category::Distro(Self::unpack_dis(general, specific)),
            _ => Category::Any,
        }
    }

    fn pack(&self) -> PackedCategoryType {
        let packed_build = self.pack_category();
        let packed_number = self.number as PackedCategoryType;
        let packed_value = self.value as PackedCategoryType;
        (packed_build << SHIFT_FOR_CATEGORY) | (packed_number << SHIFT_FOR_NUMBER) | (packed_value << SHIFT_FOR_VALUE)
    }

    fn unpack(packed: PackedCategoryType) -> Self {
        let packed_build = (packed >> SHIFT_FOR_CATEGORY) & MASK_FOR_CATEGORY;
        let packed_number = (packed >> SHIFT_FOR_NUMBER) & MASK_FOR_NUMBER;
        let packed_value = (packed >> SHIFT_FOR_VALUE) & MASK_FOR_VALUE;
        Self {
            number: packed_number as category::NumberType,
            category: Self::unpack_category(packed_build),
            value: packed_value as category::ValueType,
        }
    }
}

impl Bufferable for PlayerCategory {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.pack().push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        Self::unpack(PackedCategoryType::pull_from(buf))
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedCategoryType>()
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::category::{Category, Developer, Role};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player::player_category::PlayerCategory;

    #[test]
    fn test_player_category() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerCategory {
            number: 123,
            category: Category::Role(Role::Developer(Developer::QA)),
            value: 9,
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerCategory>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(result, orig);
    }
}
