use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::{DelayType, RuntimeAmountType};
use crate::view::GameProcessPlayerView;

#[derive(Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMachinePlayerView {
    pub vitals: [RuntimeAmountType; 4],
    pub queue: Vec<(GameProcessPlayerView, DelayType)>,
    pub running: Vec<GameProcessPlayerView>,
}

#[cfg(test)]
impl GameMachinePlayerView {
    pub fn test_default() -> Self {
        Self {
            vitals: [1, 2, 3, 4],
            queue: vec![(GameProcessPlayerView::test_default(), 0), (GameProcessPlayerView::test_default(), 3), (GameProcessPlayerView::test_default(), 5), (GameProcessPlayerView::test_default(), 9)],
            running: vec![GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default(), GameProcessPlayerView::test_default()],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::view::GameMachinePlayerView;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_machine_player_view() -> Result<(), SizedBufferError> {
        let orig = GameMachinePlayerView::test_default();

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameMachinePlayerView>()?;

        assert_eq!(orig.vitals, result.vitals);
        assert_eq!(orig.queue, result.queue);
        assert_eq!(orig.running, result.running);

        Ok(())
    }
}
