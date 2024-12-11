use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Visibility};

#[derive(Component)]
pub(crate) struct Hider {
    delta_time: f32,
    duration: f32,
}

impl Hider {
    pub(crate) fn new(duration: f32) -> Self {
        Self {
            delta_time: 0.0,
            duration,
        }
    }
}

pub(crate) fn hider_update(
    //
    mut commands: Commands,
    mut hider_q: Query<(Entity, &mut Hider)>,
    time: Res<Time>,
) {
    for (e, mut hide) in hider_q.iter_mut() {
        hide.delta_time += time.delta().as_secs_f32();
        if hide.delta_time > hide.duration {
            commands.entity(e).remove::<Hider>().insert(Visibility::Hidden);
        }
    }
}
