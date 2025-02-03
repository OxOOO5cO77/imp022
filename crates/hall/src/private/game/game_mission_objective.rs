use hall::view::GameMissionObjectivePlayerView;

pub struct GameMissionObjective {
    complete: bool,
}

impl From<&GameMissionObjective> for GameMissionObjectivePlayerView {
    fn from(value: &GameMissionObjective) -> Self {
        Self {
            complete: value.complete,
        }
    }
}
