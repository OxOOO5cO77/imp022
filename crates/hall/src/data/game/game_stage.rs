use num_enum::{FromPrimitive, IntoPrimitive};
use shared_net::{op, Bufferable, VSizedBuffer};

#[cfg(test)]
use strum_macros::EnumIter;

type GameStageType = u8;
type GamePhaseType = u8;

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum GameStage {
    #[default]
    Idle,
    Building,
    Running(GamePhase),
    End,
}

#[repr(u8)]
#[derive(Default, Clone, Copy, PartialEq, FromPrimitive, IntoPrimitive)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum GamePhase {
    #[default]
    ChooseIntent,
    ChooseAttr,
    CardPlay,
    TurnEnd,
}

impl GamePhase {
    pub(crate) fn expected_command(&self) -> op::Command {
        match self {
            GamePhase::ChooseIntent => op::Command::GameChooseIntent,
            GamePhase::ChooseAttr => op::Command::GameChooseAttr,
            GamePhase::CardPlay => op::Command::GamePlayCard,
            GamePhase::TurnEnd => op::Command::GameEndTurn,
        }
    }
}

impl GameStage {
    const REPR_IDLE: GameStageType = 0;
    const REPR_BUILDING: GameStageType = 1;
    const REPR_RUNNING: GameStageType = 2;
    const REPR_END: GameStageType = 3;
}

impl Bufferable for GameStage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            GameStage::Idle => Self::REPR_IDLE.push_into(buf),
            GameStage::Building => Self::REPR_BUILDING.push_into(buf),
            GameStage::Running(phase) => {
                Self::REPR_RUNNING.push_into(buf);
                phase.push_into(buf);
            }
            GameStage::End => Self::REPR_END.push_into(buf),
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        match u8::pull_from(buf) {
            Self::REPR_IDLE => GameStage::Idle,
            Self::REPR_BUILDING => GameStage::Building,
            Self::REPR_RUNNING => {
                let phase = GamePhase::pull_from(buf);
                GameStage::Running(phase)
            }
            Self::REPR_END => GameStage::End,
            _ => GameStage::Idle,
        }
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GameStageType>()
            + match self {
                GameStage::Running(phase) => phase.size_in_buffer(),
                _ => 0,
            }
    }
}

impl Bufferable for GamePhase {
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
        use crate::data::game::game_stage::{GamePhase, GameStage};
        use shared_net::VSizedBuffer;
        use strum::IntoEnumIterator;

        #[test]
        fn test_game_stage() {
            for game_stage in GameStage::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push(&game_stage);
                buf1.push(&game_stage);

                assert_eq!(game_stage, buf1.pull::<GameStage>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<GameStage>(&mut buf1);

                assert_eq!(game_stage, buf2.pull::<GameStage>());
            }
        }

        #[test]
        fn test_game_phase() {
            for game_phase in GamePhase::iter() {
                let mut buf1 = VSizedBuffer::new(32);
                buf1.push(&game_phase);
                buf1.push(&game_phase);

                assert_eq!(game_phase, buf1.pull::<GamePhase>());

                let mut buf2 = VSizedBuffer::new(32);
                buf2.xfer::<GamePhase>(&mut buf1);

                assert_eq!(game_phase, buf2.pull::<GamePhase>());
            }
        }
    }
}
