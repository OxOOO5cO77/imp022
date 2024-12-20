use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Commands, Component, Entity, Query, Res, Time, Visibility};

pub(crate) struct HiderPlugin;

impl Plugin for HiderPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, hider_update);
    }
}

#[derive(Component)]
pub(crate) struct Hider {
    delta_time: f32,
    duration: f32,
    state: Visibility,
}

impl Hider {
    pub(crate) fn new(duration: f32, state: Visibility) -> Self {
        Self {
            delta_time: 0.0,
            duration,
            state,
        }
    }
}

fn hider_update(
    //
    mut commands: Commands,
    mut hider_q: Query<(Entity, &mut Hider)>,
    time: Res<Time>,
) {
    for (e, mut hide) in hider_q.iter_mut() {
        hide.delta_time += time.delta().as_secs_f32();
        if hide.delta_time > hide.duration {
            commands.entity(e).remove::<Hider>().insert(hide.state);
        }
    }
}
