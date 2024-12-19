use crate::data::player::PlayerPart;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use shared_net::bufferable_derive::Bufferable;
use shared_net::sizedbuffers::Bufferable;
use shared_net::types::GameIdType;
use shared_net::{op, VSizedBuffer};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActivateRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameActivateRequest {
    const COMMAND: op::Command = op::Command::GameActivate;
}

impl GameRequestMessage for GameActivateRequest {
    fn game_id(&self) -> GameIdType {
        self.game_id
    }
}

type PartsArray = [PlayerPart; 8];

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActivateResponse {
    pub game_id: GameIdType,
    pub parts: PartsArray,
}

impl CommandMessage for GameActivateResponse {
    const COMMAND: op::Command = op::Command::GameActivate;
}

impl GameResponseMessage for GameActivateResponse {}

#[cfg(test)]
mod test {
    use crate::data::core::{Build, Detail};
    use crate::data::player::PlayerBuild;
    use crate::data::player::PlayerDetail;
    use crate::data::player::PlayerPart;
    use crate::message::game_activate::{GameActivateRequest, GameActivateResponse};
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

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
                PlayerBuild {
                    number: 4,
                    build: Build::ANT(1, 2),
                    value: 9,
                },
                PlayerBuild {
                    number: 3,
                    build: Build::BRD(8, 3),
                    value: 8,
                },
                PlayerBuild {
                    number: 2,
                    build: Build::CPU(5, 1),
                    value: 7,
                },
                PlayerBuild {
                    number: 1,
                    build: Build::DSK(3, 4),
                    value: 6,
                },
            ],
            detail: [
                PlayerDetail {
                    number: 4,
                    detail: Detail::Institution(1, 2),
                    value: 5,
                },
                PlayerDetail {
                    number: 3,
                    detail: Detail::Role(2, 5),
                    value: 4,
                },
                PlayerDetail {
                    number: 2,
                    detail: Detail::Location(3, 10),
                    value: 3,
                },
                PlayerDetail {
                    number: 1,
                    detail: Detail::Distro(4, 15),
                    value: 2,
                },
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
