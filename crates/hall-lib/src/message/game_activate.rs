use crate::core::GameSubCommand;
use crate::message::{CommandMessage, GameRequestMessage, GameResponseMessage};
use crate::player::PlayerPart;
use shared_net::op::SubCommandType;
use shared_net::{Bufferable, GameIdType, SizedBuffer, SizedBufferError, op};

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameActivateRequest {
    pub game_id: GameIdType,
}

impl CommandMessage for GameActivateRequest {
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Activate as SubCommandType);
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
    const COMMAND: op::Command = op::Command::Game(GameSubCommand::Activate as SubCommandType);
}

impl GameResponseMessage for GameActivateResponse {}

#[cfg(test)]
mod test {
    use crate::core::{Build, Detail};
    use crate::message::game_activate::{GameActivateRequest, GameActivateResponse};
    use crate::player::{PlayerBuild, PlayerDetail, PlayerPart};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_request() -> Result<(), SizedBufferError> {
        let orig = GameActivateRequest {
            game_id: 1234567890,
        };

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameActivateRequest>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }

    #[test]
    fn test_response() -> Result<(), SizedBufferError> {
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

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameActivateResponse>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
        Ok(())
    }
}
