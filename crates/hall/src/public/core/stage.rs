use num_enum::{FromPrimitive, IntoPrimitive};
use shared_net::{op, Bufferable, VSizedBuffer};

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
    pub fn expected_command(&self) -> op::Command {
        match self {
            Phase::ChooseIntent => op::Command::GameChooseIntent,
            Phase::ChooseAttr => op::Command::GameChooseAttr,
            Phase::CardPlay => op::Command::GamePlayCard,
            Phase::TurnEnd => op::Command::GameEndTurn,
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
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            Stage::Idle => Self::REPR_IDLE.push_into(buf),
            Stage::Building => Self::REPR_BUILDING.push_into(buf),
            Stage::Running(phase) => {
                Self::REPR_RUNNING.push_into(buf);
                phase.push_into(buf);
            }
            Stage::End => Self::REPR_END.push_into(buf),
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        match u8::pull_from(buf) {
            Self::REPR_IDLE => Stage::Idle,
            Self::REPR_BUILDING => Stage::Building,
            Self::REPR_RUNNING => {
                let phase = Phase::pull_from(buf);
                Stage::Running(phase)
            }
            Self::REPR_END => Stage::End,
            _ => Stage::Idle,
        }
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
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let game_phase: GamePhaseType = (*self).into();
        game_phase.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_phase = GamePhaseType::pull_from(buf);
        game_phase.into()
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GamePhaseType>()
    }
}

#[cfg(test)]
mod test {
    mod test_vsizedbuffer {
        use crate::public::core::stage::{Phase, Stage};
        use shared_net::VSizedBuffer;
        use strum::IntoEnumIterator;

        #[test]
        fn test_game_stage() {
            for game_stage in Stage::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push(&game_stage);
                buf1.push(&game_stage);

                assert_eq!(game_stage, buf1.pull::<Stage>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<Stage>(&mut buf1);

                assert_eq!(game_stage, buf2.pull::<Stage>());
            }
        }

        #[test]
        fn test_game_phase() {
            for game_phase in Phase::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push(&game_phase);
                buf1.push(&game_phase);

                assert_eq!(game_phase, buf1.pull::<Phase>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<Phase>(&mut buf1);

                assert_eq!(game_phase, buf2.pull::<Phase>());
            }
        }
    }
}
