use hall::core::AuthLevel;

enum ActorKind {
    Guest,
}

pub struct GameActor {
    _kind: ActorKind,
    _auth_level: AuthLevel,
}

impl GameActor {
    pub(crate) fn new() -> Self {
        Self {
            _kind: ActorKind::Guest,
            _auth_level: AuthLevel::Guest,
        }
    }
}
