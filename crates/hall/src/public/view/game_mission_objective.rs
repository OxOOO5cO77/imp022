use crate::hall::HallMissionObjective;
use shared_net::{Bufferable, VSizedBuffer};

pub struct GameMissionObjective {
    complete: bool,
}

impl From<&HallMissionObjective> for GameMissionObjective {
    fn from(_value: &HallMissionObjective) -> Self {
        Self {
            complete: false,
        }
    }
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
    use crate::view::GameMissionObjectivePlayerView;
    use shared_net::{Bufferable, VSizedBuffer};

    #[test]
    fn test_game_mission_objective_player_view() {
        let orig_view = GameMissionObjectivePlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionObjectivePlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
