use rand::{Rng, RngExt};

use hall_lib::core::{ActorIdType, AuthLevel};
use hall_lib::view::GameActorPlayerView;

enum ActorKind {
    Guest,
}

pub(crate) struct GameActor {
    _kind: ActorKind,
    pub(crate) auth_level: AuthLevel,
}

impl GameActor {
    pub(crate) fn new(rng: &mut impl Rng) -> Self {
        let auth_level = Self::pick_auth(rng);
        Self {
            _kind: ActorKind::Guest,
            auth_level,
        }
    }

    fn pick_auth(rng: &mut impl Rng) -> AuthLevel {
        match rng.random::<u64>() & 0x0F {
            0..8 => AuthLevel::Guest,
            8..12 => AuthLevel::User,
            12..15 => AuthLevel::Admin,
            15 => AuthLevel::Root,
            _ => AuthLevel::Anonymous,
        }
    }
}

impl GameActor {
    pub(crate) fn to_player_view(&self, id: ActorIdType) -> GameActorPlayerView {
        GameActorPlayerView {
            id,
            auth_level: self.auth_level,
        }
    }
}
