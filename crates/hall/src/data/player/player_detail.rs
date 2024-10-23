use std::mem::size_of;

use serde::{Deserialize, Serialize};

use shared_data::player::detail;
use shared_data::player::detail::{Academic, Bureaucratic, Detail, Consumer, Corporate, Decentralized, Developer, Distro, Fringe, Hardened, Institution, IT, Location, Office, People, Physical, Public, Residence, Restricted, Role, Unauthorized};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedDetailType = u32;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerDetail {
    pub number: detail::NumberType,
    pub detail: Detail,
    pub value: detail::ValueType,
}


const BITS_FOR_DETAIL: PackedDetailType = 16;
const BITS_FOR_NUMBER: PackedDetailType = 8;
const BITS_FOR_VALUE: PackedDetailType = 8;

const SHIFT_FOR_DETAIL: PackedDetailType = 0;
const SHIFT_FOR_NUMBER: PackedDetailType = SHIFT_FOR_DETAIL + BITS_FOR_DETAIL;
const SHIFT_FOR_VALUE: PackedDetailType = SHIFT_FOR_NUMBER + BITS_FOR_NUMBER;

const MASK_FOR_DETAIL: PackedDetailType = (1 << BITS_FOR_DETAIL) - 1;
const MASK_FOR_NUMBER: PackedDetailType = (1 << BITS_FOR_NUMBER) - 1;
const MASK_FOR_VALUE: PackedDetailType = (1 << BITS_FOR_VALUE) - 1;

const BITS_FOR_DETAIL_PART: PackedDetailType = 5;
const SHIFT_FOR_DETAIL_SPECIFIC: PackedDetailType = 0;
const SHIFT_FOR_DETAIL_GENERAL: PackedDetailType = SHIFT_FOR_DETAIL_SPECIFIC + BITS_FOR_DETAIL_PART;
const SHIFT_FOR_DETAIL_KIND: PackedDetailType = SHIFT_FOR_DETAIL_GENERAL + BITS_FOR_DETAIL_PART;
const MASK_FOR_DETAIL_PART: PackedDetailType = (1 << BITS_FOR_DETAIL_PART) - 1;

impl PlayerDetail {
    fn pack_ins(ins: Institution) -> (PackedDetailType, PackedDetailType) {
        match ins {
            Institution::Any => (0, 0),
            Institution::Academic(academic) => (1, academic as PackedDetailType),
            Institution::Bureaucratic(bureaucratic) => (2, bureaucratic as PackedDetailType),
            Institution::Corporate(corporate) => (3, corporate as PackedDetailType),
            Institution::Decentralized(decentralized) => (4, decentralized as PackedDetailType),
        }
    }
    fn pack_rol(rol: Role) -> (PackedDetailType, PackedDetailType) {
        match rol {
            Role::Any => (0, 0),
            Role::Developer(developer) => (1, developer as PackedDetailType),
            Role::IT(it) => (2, it as PackedDetailType),
            Role::People(people) => (3, people as PackedDetailType),
            Role::Physical(physical) => (4, physical as PackedDetailType),
        }
    }
    fn pack_loc(loc: Location) -> (PackedDetailType, PackedDetailType) {
        match loc {
            Location::Any => (0, 0),
            Location::Office(office) => (1, office as PackedDetailType),
            Location::Public(public) => (2, public as PackedDetailType),
            Location::Residence(residence) => (3, residence as PackedDetailType),
            Location::Unauthorized(unauthorized) => (4, unauthorized as PackedDetailType),
        }
    }
    fn pack_dis(dis: Distro) -> (PackedDetailType, PackedDetailType) {
        match dis {
            Distro::Any => (0, 0),
            Distro::Consumer(consumer) => (1, consumer as PackedDetailType),
            Distro::Fringe(fringe) => (2, fringe as PackedDetailType),
            Distro::Hardened(hardened) => (3, hardened as PackedDetailType),
            Distro::Restricted(restricted) => (4, restricted as PackedDetailType),
        }
    }

    fn pack_detail(&self) -> PackedDetailType {
        let (kind, (general, specific)) = match self.detail {
            Detail::Any => (0, (0, 0)),
            Detail::Institution(ins) => (1, Self::pack_ins(ins)),
            Detail::Role(rol) => (2, Self::pack_rol(rol)),
            Detail::Location(loc) => (3, Self::pack_loc(loc)),
            Detail::Distro(dis) => (4, Self::pack_dis(dis)),
        };
        (kind << SHIFT_FOR_DETAIL_KIND) | (general << SHIFT_FOR_DETAIL_GENERAL) | (specific << SHIFT_FOR_DETAIL_SPECIFIC)
    }

    fn unpack_ins_aca(specific: PackedDetailType) -> Academic {
        match specific {
            1 => Academic::CompSci,
            2 => Academic::Cybernetics,
            3 => Academic::Engineering,
            4 => Academic::Theoretical,
            _ => Academic::Any,
        }
    }
    fn unpack_ins_bur(specific: PackedDetailType) -> Bureaucratic {
        match specific {
            1 => Bureaucratic::Africa,
            2 => Bureaucratic::Americas,
            3 => Bureaucratic::Asia,
            4 => Bureaucratic::Europe,
            _ => Bureaucratic::Any,
        }
    }
    fn unpack_ins_cor(specific: PackedDetailType) -> Corporate {
        match specific {
            1 => Corporate::Consumer,
            2 => Corporate::Entertainment,
            3 => Corporate::Industrial,
            4 => Corporate::Military,
            _ => Corporate::Any,
        }
    }
    fn unpack_ins_dec(specific: PackedDetailType) -> Decentralized {
        match specific {
            1 => Decentralized::Activist,
            2 => Decentralized::Enthusiast,
            3 => Decentralized::Freelance,
            4 => Decentralized::OpenSource,
            _ => Decentralized::Any,
        }
    }

    fn unpack_ins(company: PackedDetailType, specific: PackedDetailType) -> Institution {
        match company {
            1 => Institution::Academic(Self::unpack_ins_aca(specific)),
            2 => Institution::Bureaucratic(Self::unpack_ins_bur(specific)),
            3 => Institution::Corporate(Self::unpack_ins_cor(specific)),
            4 => Institution::Decentralized(Self::unpack_ins_dec(specific)),
            _ => Institution::Any,
        }
    }

    fn unpack_rol_dev(specific: PackedDetailType) -> Developer {
        match specific {
            1 => Developer::Art,
            2 => Developer::Production,
            3 => Developer::Programming,
            4 => Developer::QA,
            _ => Developer::Any,
        }
    }
    fn unpack_rol_it(specific: PackedDetailType) -> IT {
        match specific {
            1 => IT::DevOps,
            2 => IT::Hardware,
            3 => IT::Security,
            4 => IT::Support,
            _ => IT::Any,
        }
    }
    fn unpack_rol_peo(specific: PackedDetailType) -> People {
        match specific {
            1 => People::Accounting,
            2 => People::Admin,
            3 => People::HR,
            4 => People::Marketing,
            _ => People::Any,
        }
    }
    fn unpack_rol_phy(specific: PackedDetailType) -> Physical {
        match specific {
            1 => Physical::Maintenance,
            2 => Physical::Security,
            3 => Physical::Supply,
            4 => Physical::Trades,
            _ => Physical::Any,
        }
    }

    fn unpack_rol(company: PackedDetailType, specific: PackedDetailType) -> Role {
        match company {
            1 => Role::Developer(Self::unpack_rol_dev(specific)),
            2 => Role::IT(Self::unpack_rol_it(specific)),
            3 => Role::People(Self::unpack_rol_peo(specific)),
            4 => Role::Physical(Self::unpack_rol_phy(specific)),
            _ => Role::Any,
        }
    }

    fn unpack_loc_off(specific: PackedDetailType) -> Office {
        match specific {
            1 => Office::Campus,
            2 => Office::Ephemeral,
            3 => Office::Satellite,
            4 => Office::Tower,
            _ => Office::Any,
        }
    }
    fn unpack_loc_pub(specific: PackedDetailType) -> Public {
        match specific {
            1 => Public::Commercial,
            2 => Public::Education,
            3 => Public::Hospitality,
            4 => Public::Municipal,
            _ => Public::Any,
        }
    }
    fn unpack_loc_res(specific: PackedDetailType) -> Residence {
        match specific {
            1 => Residence::Apartment,
            2 => Residence::Detached,
            3 => Residence::Hotel,
            4 => Residence::Shared,
            _ => Residence::Any,
        }
    }
    fn unpack_loc_una(specific: PackedDetailType) -> Unauthorized {
        match specific {
            1 => Unauthorized::Infrastructure,
            2 => Unauthorized::Office,
            3 => Unauthorized::Public,
            4 => Unauthorized::Residential,
            _ => Unauthorized::Any,
        }
    }


    fn unpack_loc(company: PackedDetailType, specific: PackedDetailType) -> Location {
        match company {
            1 => Location::Office(Self::unpack_loc_off(specific)),
            2 => Location::Public(Self::unpack_loc_pub(specific)),
            3 => Location::Residence(Self::unpack_loc_res(specific)),
            4 => Location::Unauthorized(Self::unpack_loc_una(specific)),
            _ => Location::Any,
        }
    }

    fn unpack_dis_con(specific: PackedDetailType) -> Consumer {
        match specific {
            1 => Consumer::Casual,
            2 => Consumer::Content,
            3 => Consumer::Gaming,
            4 => Consumer::Productivity,
            _ => Consumer::Any,
        }
    }
    fn unpack_dis_fri(specific: PackedDetailType) -> Fringe {
        match specific {
            1 => Fringe::Exotic,
            2 => Fringe::Niche,
            3 => Fringe::Retro,
            4 => Fringe::Source,
            _ => Fringe::Any,
        }
    }
    fn unpack_dis_har(specific: PackedDetailType) -> Hardened {
        match specific {
            1 => Hardened::Anonymous,
            2 => Hardened::Crypto,
            3 => Hardened::Government,
            4 => Hardened::Industry,
            _ => Hardened::Any,
        }
    }
    fn unpack_dis_res(specific: PackedDetailType) -> Restricted {
        match specific {
            1 => Restricted::Access,
            2 => Restricted::Distribution,
            3 => Restricted::Install,
            4 => Restricted::Use,
            _ => Restricted::Any,
        }
    }

    fn unpack_dis(company: PackedDetailType, specific: PackedDetailType) -> Distro {
        match company {
            1 => Distro::Consumer(Self::unpack_dis_con(specific)),
            2 => Distro::Fringe(Self::unpack_dis_fri(specific)),
            3 => Distro::Hardened(Self::unpack_dis_har(specific)),
            4 => Distro::Restricted(Self::unpack_dis_res(specific)),
            _ => Distro::Any,
        }
    }

    fn unpack_detail(packed: PackedDetailType) -> Detail {
        let kind = (packed >> SHIFT_FOR_DETAIL_KIND) & MASK_FOR_DETAIL_PART;
        let general = (packed >> SHIFT_FOR_DETAIL_GENERAL) & MASK_FOR_DETAIL_PART;
        let specific = (packed >> SHIFT_FOR_DETAIL_SPECIFIC) & MASK_FOR_DETAIL_PART;

        match kind {
            1 => Detail::Institution(Self::unpack_ins(general, specific)),
            2 => Detail::Role(Self::unpack_rol(general, specific)),
            3 => Detail::Location(Self::unpack_loc(general, specific)),
            4 => Detail::Distro(Self::unpack_dis(general, specific)),
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
            number: packed_number as detail::NumberType,
            detail: Self::unpack_detail(packed_build),
            value: packed_value as detail::ValueType,
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
    use shared_data::player::detail::{Detail, Developer, Role};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player::player_detail::PlayerDetail;

    #[test]
    fn test_player_detail() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerDetail {
            number: 123,
            detail: Detail::Role(Role::Developer(Developer::QA)),
            value: 9,
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerDetail>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(result, orig);
    }
}
