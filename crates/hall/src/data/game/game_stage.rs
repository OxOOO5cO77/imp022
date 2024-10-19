use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

#[cfg(test)]
use strum_macros::EnumIter;

#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum GameStage {
    #[default] Idle,
    Building,
    Running(GamePhase),
    End,
}

#[derive(Default, Clone, Copy, PartialEq)]
#[cfg_attr(test, derive(Debug, EnumIter))]
pub enum GamePhase {
    #[default] Waiting,
    Roll,
    ChooseAttr,
    ResourceAlloc,
    CardPlay,
    CardResolve,
    Tick,
}


impl Bufferable for GameStage {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            GameStage::Idle => 0u8.push_into(buf),
            GameStage::Building => 1u8.push_into(buf),
            GameStage::Running(phase) => {
                2u8.push_into(buf);
                phase.push_into(buf);
            }
            GameStage::End => 3u8.push_into(buf),
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        match u8::pull_from(buf) {
            0 => GameStage::Idle,
            1 => GameStage::Building,
            2 => {
                let phase = GamePhase::pull_from(buf);
                GameStage::Running(phase)
            }
            3 => GameStage::End,
            _ => GameStage::Idle,
        }
    }

    fn size_in_buffer(&self) -> usize {
        match self {
            GameStage::Running(_) => 0u8.size_in_buffer() + 0u8.size_in_buffer(),
            _ => 0u8.size_in_buffer(),
        }
    }
}

impl Bufferable for GamePhase {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        match self {
            GamePhase::Waiting => 0u8,
            GamePhase::Roll => 1,
            GamePhase::ChooseAttr => 2,
            GamePhase::ResourceAlloc => 3,
            GamePhase::CardPlay => 4,
            GamePhase::CardResolve => 5,
            GamePhase::Tick => 6,
        }.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        match u8::pull_from(buf) {
            0 => GamePhase::Waiting,
            1 => GamePhase::Roll,
            2 => GamePhase::ChooseAttr,
            3 => GamePhase::ResourceAlloc,
            4 => GamePhase::CardPlay,
            5 => GamePhase::CardResolve,
            6 => GamePhase::Tick,
            _ => GamePhase::Waiting,
        }
    }

    fn size_in_buffer(&self) -> usize {
        0u8.size_in_buffer()
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
