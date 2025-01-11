use hall::hall::HallMissionObjective;
use hall::view::GameMissionObjectivePlayerView;

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

impl From<&GameMissionObjective> for GameMissionObjectivePlayerView {
    fn from(value: &GameMissionObjective) -> Self {
        Self {
            complete: value.complete,
        }
    }
}
