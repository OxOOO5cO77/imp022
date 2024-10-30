use shared_data::types::GameIdType;
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

use crate::data::player::player_part::PlayerPart;
use crate::message::Request;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActivateRequest {
    pub game_id: GameIdType,
}

impl Request for GameActivateRequest {
    const COMMAND: op::Command = op::Command::GameActivate;
}

impl Bufferable for GameActivateRequest {
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
        self.game_id.size_in_buffer()
    }
}

type PartsArray = [PlayerPart; 8];

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActivateResponse {
    pub game_id: GameIdType,
    pub parts: PartsArray,
}

impl Bufferable for GameActivateResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        self.parts.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let parts = PartsArray::pull_from(buf);
        Self {
            game_id,
            parts,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer() + self.parts.size_in_buffer()
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
    use crate::message::game_activate::{GameActivateRequest, GameActivateResponse};

    #[test]
    fn test_request() {
        let orig = GameActivateRequest {
            game_id: 1234567890,
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameActivateRequest>();

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
            detail: [
                PlayerDetail { number: 4, detail: Detail::Institution(Institution::Academic(Academic::CompSci)), value: 5 },
                PlayerDetail { number: 3, detail: Detail::Role(Role::Physical(Physical::Trades)), value: 4 },
                PlayerDetail { number: 2, detail: Detail::Location(Location::Unauthorized(Unauthorized::Infrastructure)), value: 3 },
                PlayerDetail { number: 1, detail: Detail::Distro(Distro::Restricted(Restricted::Distribution)), value: 2 },
            ],
        };

        let orig = GameActivateResponse {
            game_id: 1234567890,
            parts: [part; 8],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameActivateResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }
}
