use num_enum::{FromPrimitive, IntoPrimitive};

use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::GameSubCommand;

#[cfg(test)]
use strum_macros::EnumIter;

type GameStageType = u8;
type GamePhaseType = u8;

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum Stage {
    #[default]
    Idle,
    Building,
    Running(Phase),
    End,
}

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum Phase {
    #[default]
    ChooseIntent,
    ChooseAttr,
    CardPlay,
    TurnEnd,
}

impl Phase {
    pub fn expected_command(&self) -> GameSubCommand {
        match self {
            Phase::ChooseIntent => GameSubCommand::ChooseIntent,
            Phase::ChooseAttr => GameSubCommand::ChooseAttr,
            Phase::CardPlay => GameSubCommand::PlayCard,
            Phase::TurnEnd => GameSubCommand::EndTurn,
        }
    }
}

impl Stage {
    const REPR_IDLE: GameStageType = 0;
    const REPR_BUILDING: GameStageType = 1;
    const REPR_RUNNING: GameStageType = 2;
    const REPR_END: GameStageType = 3;
}

impl Bufferable for Stage {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        match self {
            Stage::Idle => Self::REPR_IDLE.push_into(buf),
            Stage::Building => Self::REPR_BUILDING.push_into(buf),
            Stage::Running(phase) => {
                Self::REPR_RUNNING.push_into(buf)?;
                phase.push_into(buf)
            }
            Stage::End => Self::REPR_END.push_into(buf),
        }
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let result = match u8::pull_from(buf)? {
            Self::REPR_IDLE => Stage::Idle,
            Self::REPR_BUILDING => Stage::Building,
            Self::REPR_RUNNING => {
                let phase = Phase::pull_from(buf)?;
                Stage::Running(phase)
            }
            Self::REPR_END => Stage::End,
            _ => Stage::Idle,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GameStageType>()
            + match self {
                Stage::Running(phase) => phase.size_in_buffer(),
                _ => 0,
            }
    }
}

impl Bufferable for Phase {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let game_phase: GamePhaseType = (*self).into();
        game_phase.push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let game_phase = GamePhaseType::pull_from(buf)?;
        Ok(game_phase.into())
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GamePhaseType>()
    }
}

#[cfg(test)]
mod test {
    use super::{Phase, Stage};
    use shared_net::{SizedBuffer, SizedBufferError};
    use strum::IntoEnumIterator;

    #[test]
    fn test_game_stage() -> Result<(), SizedBufferError> {
        for game_stage in Stage::iter() {
            let mut buf1 = SizedBuffer::new(32);
            buf1.push(&game_stage)?;
            buf1.push(&game_stage)?;

            assert_eq!(game_stage, buf1.pull::<Stage>()?);

            let mut buf2 = SizedBuffer::new(32);
            buf2.xfer::<Stage>(&mut buf1)?;

            assert_eq!(game_stage, buf2.pull::<Stage>()?);
        }
        Ok(())
    }

    #[test]
    fn test_game_phase() -> Result<(), SizedBufferError> {
        for game_phase in Phase::iter() {
            let mut buf1 = SizedBuffer::new(32);
            buf1.push(&game_phase)?;
            buf1.push(&game_phase)?;

            assert_eq!(game_phase, buf1.pull::<Phase>()?);

            let mut buf2 = SizedBuffer::new(32);
            buf2.xfer::<Phase>(&mut buf1)?;

            assert_eq!(game_phase, buf2.pull::<Phase>()?);
        }
        Ok(())
    }
}
