use crate::system::ui_effects::SetColorEvent;
use bevy::app::{App, Plugin, Update};
use bevy::color::Srgba;
use bevy::prelude::{ops, Commands, Component, Entity, Mix, Query, Res, Time};

pub(crate) struct GlowerPlugin;

impl Plugin for GlowerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, glower_update);
    }
}

#[derive(Component)]
pub(crate) struct Glower {
    source: Srgba,
    target: Srgba,
    speed: f32,
}

impl Glower {
    pub(crate) fn new(source: Srgba, target: Srgba, speed: f32) -> Self {
        Self {
            source,
            target,
            speed,
        }
    }

    pub(crate) fn remove(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Glower>().trigger(SetColorEvent::from(self.source));
    }
}

fn glower_update(
    //
    mut commands: Commands,
    time: Res<Time>,
    mut mesh_q: Query<(Entity, &Glower)>,
) {
    for (entity, glow) in mesh_q.iter_mut() {
        let t = (ops::sin(time.elapsed_secs() * glow.speed) + 1.0) / 2.0;
        let color = glow.source.mix(&glow.target, t);
        commands.entity(entity).trigger(SetColorEvent::from(color));
    }
}
