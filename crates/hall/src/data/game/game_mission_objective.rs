use crate::data::hall::HallMissionObjective;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

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

#[derive(Default, Clone)]
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

impl Bufferable for GameMissionObjectivePlayerView {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.complete.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let complete = bool::pull_from(buf);
        Self {
            complete,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.complete.size_in_buffer()
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
    use crate::data::game::GameMissionObjectivePlayerView;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_game_mission_objective_player_view() {
        let orig_view = GameMissionObjectivePlayerView::test_default();

        let mut buf = VSizedBuffer::new(orig_view.size_in_buffer());
        buf.push(&orig_view);
        let new_view = buf.pull::<GameMissionObjectivePlayerView>();

        assert_eq!(orig_view, new_view);
    }
}
