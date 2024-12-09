use bevy::app::{App, Plugin, Update};
use bevy::math::ops;
use bevy::prelude::{Color, Commands, Component, Entity, Mix, Query, Res, Sprite, Time};
use std::f32::consts::PI;

pub(crate) struct UiEffectsPlugin;

impl Plugin for UiEffectsPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, glower_update)
            .add_systems(Update, blinker_update);
    }
}

#[derive(Component)]
pub(crate) struct Glower {
    source: Color,
    target: Color,
    speed: f32,
}

impl Glower {
    pub(crate) fn new(source: Color, target: Color, speed: f32) -> Self {
        Self {
            source,
            target,
            speed,
        }
    }
    pub(crate) fn remove(&self, commands: &mut Commands, sprite: &mut Sprite, entity: Entity) {
        sprite.color = self.source;
        commands.entity(entity).remove::<Glower>();
    }
}

fn glower_update(
    //
    mut glower_q: Query<(&mut Sprite, &mut Glower)>,
    time: Res<Time>,
) {
    for (mut sprite, glow) in glower_q.iter_mut() {
        let t = (ops::sin(time.elapsed_secs() * glow.speed) + 1.0) / 2.0;
        sprite.color = glow.source.mix(&glow.target, t);
    }
}

#[derive(Component)]
pub(crate) struct Blinker {
    source: Color,
    target: Color,
    delta_time: f32,
    target_time: f32,
    speed: f32,
}

impl Blinker {
    pub(crate) fn new(source: Color, target: Color, count: f32, speed: f32) -> Self {
        Self {
            source,
            target,
            delta_time: 0.0,
            target_time: (2.0 * PI * count) / speed,
            speed,
        }
    }

    pub(crate) fn remove(&self, commands: &mut Commands, sprite: &mut Sprite, entity: Entity) {
        sprite.color = self.source;
        commands.entity(entity).remove::<Blinker>();
    }
}

fn blinker_update(
    //
    mut commands: Commands,
    mut blinker_q: Query<(Entity, &mut Sprite, &mut Blinker)>,
    time: Res<Time>,
) {
    for (e, mut sprite, mut blink) in blinker_q.iter_mut() {
        blink.delta_time += time.delta().as_secs_f32();
        if blink.delta_time > blink.target_time {
            blink.remove(&mut commands, &mut sprite, e);
            return;
        }

        let x = (blink.delta_time * blink.speed) - (PI / 2.0);
        let t = (ops::sin(x) + 1.0) / 2.0;

        sprite.color = blink.source.mix(&blink.target, t);
    }
}
