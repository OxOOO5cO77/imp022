use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

pub struct GameMissionObjective {
    complete: bool,
}

#[derive(Default, Clone, Bufferable)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameMissionObjectivePlayerView {
    pub complete: bool,
}

impl From<&GameMissionObjective> for GameMissionObjectivePlayerView {
    fn from(value: &GameMissionObjective) -> Self {
        Self {
            complete: value.complete,
        }
    }
}

#[cfg(test)]
impl GameMissionObjectivePlayerView {
    pub fn test_default() -> Self {
        Self {
            complete: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::GameMissionObjectivePlayerView;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_game_mission_objective_player_view() -> Result<(), SizedBufferError> {
        let orig_view = GameMissionObjectivePlayerView::test_default();

        let mut buf = SizedBuffer::from(&orig_view)?;
        let new_view = buf.pull::<GameMissionObjectivePlayerView>()?;

        assert_eq!(orig_view, new_view);
        Ok(())
    }
}
