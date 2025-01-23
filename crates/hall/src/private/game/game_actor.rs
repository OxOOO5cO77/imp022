use hall::core::AuthLevel;

enum ActorKind {
    Guest,
}

pub(crate) struct GameActor {
    _kind: ActorKind,
    pub(crate) auth_level: AuthLevel,
}

impl GameActor {
    pub(crate) fn new() -> Self {
        Self {
            _kind: ActorKind::Guest,
            auth_level: AuthLevel::Guest,
        }
    }
}
