use crate::system::ui_effects::SetColorEvent;
use bevy::app::{App, Plugin, Update};
use bevy::color::Srgba;
use bevy::prelude::{Commands, Component, Entity, Mix, Query, Res, Time, ops};
use std::f32::consts::PI;

pub(crate) struct BlinkerPlugin;

impl Plugin for BlinkerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, blinker_update);
    }
}

#[derive(Component)]
pub(crate) struct Blinker {
    original: Srgba,
    source: Srgba,
    target: Srgba,
    delta_time: f32,
    target_time: f32,
    speed: f32,
}

impl Blinker {
    pub(crate) fn new(original: Srgba, target: Srgba, count: f32, speed: f32) -> Self {
        Self {
            original,
            source: original / 2.0,
            target,
            delta_time: 0.0,
            target_time: (2.0 * PI * count) / speed,
            speed,
        }
    }

    pub(crate) fn remove(&self, commands: &mut Commands, entity: Entity) {
        commands.entity(entity).remove::<Blinker>().trigger(SetColorEvent::new(entity, self.original));
    }
}

fn blinker_update(
    //
    mut commands: Commands,
    mut blinker_q: Query<(Entity, &mut Blinker)>,
    time: Res<Time>,
) {
    for (entity, mut blink) in blinker_q.iter_mut() {
        blink.delta_time += time.delta().as_secs_f32();
        if blink.delta_time > blink.target_time {
            blink.remove(&mut commands, entity);
            return;
        }

        let x = (blink.delta_time * blink.speed) - (PI / 2.0);
        let t = (ops::sin(x) + 1.0) / 2.0;

        let color = blink.source.mix(&blink.target, t);
        commands.entity(entity).trigger(SetColorEvent::new(entity, color));
    }
}
