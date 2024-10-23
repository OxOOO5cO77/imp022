use serde::{Deserialize, Serialize};

use shared_data::player::attribute;
use shared_data::types::SeedType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

use crate::data::player::player_build::PlayerBuild;
use crate::data::player::player_detail::PlayerDetail;

type AttributeArray = [attribute::ValueType; 4];
type BuildArray = [PlayerBuild; 4];
type DetailArray = [PlayerDetail; 4];

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerPart {
    pub seed: SeedType,
    pub values: AttributeArray,
    pub build: BuildArray,
    pub detail: DetailArray,
}

impl Bufferable for PlayerPart {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.seed.push_into(buf);
        self.values.push_into(buf);
        self.build.push_into(buf);
        self.detail.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let seed = SeedType::pull_from(buf);
        let values = AttributeArray::pull_from(buf);
        let build = BuildArray::pull_from(buf);
        let detail = DetailArray::pull_from(buf);
        Self {
            seed,
            values,
            build,
            detail,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.seed.size_in_buffer() + self.values.size_in_buffer() + self.build.size_in_buffer() + self.detail.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::build::{Build, Market, ANT, BRD, CPU, DSC};
    use shared_data::player::detail::{Academic, Detail, Distro, Institution, Location, Physical, Restricted, Role, Unauthorized};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player::player_build::PlayerBuild;
    use crate::data::player::player_detail::PlayerDetail;
    use crate::data::player::player_part::PlayerPart;

    #[test]
    fn test_player_part() {
        let mut buf = VSizedBuffer::new(64);
        let orig = PlayerPart {
            seed: 1234567890,
            values: [9, 1, 9, 1],
            build: [
                PlayerBuild { number: 1, build: Build::ANT(ANT::EXM(Market::Enthusiast)), value: 9 },
                PlayerBuild { number: 2, build: Build::BRD(BRD::Wasbleibt(Market::Professional)), value: 8 },
                PlayerBuild { number: 3, build: Build::CPU(CPU::RiscFree(Market::Consumer)), value: 7 },
                PlayerBuild { number: 4, build: Build::DSC(DSC::Warehaus(Market::Prosumer)), value: 6 },
            ],
            detail: [
                PlayerDetail { number: 1, detail: Detail::Institution(Institution::Academic(Academic::CompSci)), value: 5 },
                PlayerDetail { number: 2, detail: Detail::Role(Role::Physical(Physical::Trades)), value: 4 },
                PlayerDetail { number: 3, detail: Detail::Location(Location::Unauthorized(Unauthorized::Infrastructure)), value: 3 },
                PlayerDetail { number: 4, detail: Detail::Distro(Distro::Restricted(Restricted::Distribution)), value: 2 },
            ],
        };
        buf.push(&orig);
        let result = buf.pull::<PlayerPart>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
