use std::mem::size_of;

use shared_data::types::GameIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

use crate::data::player_part::PlayerPart;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartRequest {
    pub game_id: GameIdType,
}

impl Bufferable for GameStartRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        Self {
            game_id,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GameIdType>()
    }
}

const PART_COUNT: usize = 8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameStartResponse {
    pub game_id: GameIdType,
    pub parts: [PlayerPart; PART_COUNT],
}

impl Bufferable for GameStartResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        for part in &self.parts {
            part.push_into(buf);
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let mut result = Self {
            game_id,
            parts: [PlayerPart::default(); PART_COUNT],
        };
        for i in 0..result.parts.len() {
            result.parts[i] = PlayerPart::pull_from(buf);
        }
        result
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GameIdType>() + (self.parts[0].size_in_buffer() * self.parts.len())
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::build::{ANT, BRD, Build, CPU, DSC, Market};
    use shared_data::player::category::{Academic, Category, Distro, Institution, Location, Physical, Restricted, Role, Unauthorized};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player_build::PlayerBuild;
    use crate::data::player_category::PlayerCategory;
    use crate::data::player_part::PlayerPart;
    use crate::message::gamestart::{GameStartRequest, GameStartResponse};

    #[test]
    fn test_request() {
        let orig = GameStartRequest {
            game_id: 1234567890,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameStartRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let part = PlayerPart {
            seed: 1234567890,
            values: [4, 6, 9, 1],
            build: [
                PlayerBuild { number: 4, build: Build::ANT(ANT::EXM(Market::Enthusiast)), value: 9 },
                PlayerBuild { number: 3, build: Build::BRD(BRD::Wasbleibt(Market::Professional)), value: 8 },
                PlayerBuild { number: 2, build: Build::CPU(CPU::RiscFree(Market::Consumer)), value: 7 },
                PlayerBuild { number: 1, build: Build::DSC(DSC::Warehaus(Market::Prosumer)), value: 6 },
            ],
            category: [
                PlayerCategory { number: 4, category: Category::Institution(Institution::Academic(Academic::CompSci)), value: 5 },
                PlayerCategory { number: 3, category: Category::Role(Role::Physical(Physical::Trades)), value: 4 },
                PlayerCategory { number: 2, category: Category::Location(Location::Unauthorized(Unauthorized::Infrastructure)), value: 3 },
                PlayerCategory { number: 1, category: Category::Distro(Distro::Restricted(Restricted::Distribution)), value: 2 },
            ],
        };

        let orig = GameStartResponse {
            game_id: 1234567890,
            parts: [part; 8],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameStartResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}