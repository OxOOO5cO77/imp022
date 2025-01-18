use serde::{Deserialize, Serialize};

use shared_net::{Bufferable, SeedType, SizedBuffer, SizedBufferError};

use crate::core::AttributeValueType;
use crate::player::{PlayerBuild, PlayerDetail};

type AttributeArray = [AttributeValueType; 4];
type BuildArray = [PlayerBuild; 4];
type DetailArray = [PlayerDetail; 4];

#[derive(Default, Clone, Copy, Bufferable, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct PlayerPart {
    pub seed: SeedType,
    pub values: AttributeArray,
    pub build: BuildArray,
    pub detail: DetailArray,
}

#[cfg(test)]
mod test {
    use crate::core::{Build, Detail};
    use crate::player::{PlayerBuild, PlayerDetail, PlayerPart};
    use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

    #[test]
    fn test_player_part() -> Result<(), SizedBufferError> {
        let orig = PlayerPart {
            seed: 1234567890,
            values: [9, 1, 9, 1],
            build: [
                PlayerBuild {
                    number: 1,
                    build: Build::ANT(4, 1),
                    value: 9,
                },
                PlayerBuild {
                    number: 2,
                    build: Build::BRD(3, 2),
                    value: 8,
                },
                PlayerBuild {
                    number: 3,
                    build: Build::CPU(2, 3),
                    value: 7,
                },
                PlayerBuild {
                    number: 4,
                    build: Build::DSK(1, 4),
                    value: 6,
                },
            ],
            detail: [
                PlayerDetail {
                    number: 1,
                    detail: Detail::Institution(4, 1),
                    value: 5,
                },
                PlayerDetail {
                    number: 2,
                    detail: Detail::Role(3, 2),
                    value: 4,
                },
                PlayerDetail {
                    number: 3,
                    detail: Detail::Location(2, 3),
                    value: 3,
                },
                PlayerDetail {
                    number: 4,
                    detail: Detail::Distro(1, 4),
                    value: 2,
                },
            ],
        };
        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<PlayerPart>()?;

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);

        Ok(())
    }
}
