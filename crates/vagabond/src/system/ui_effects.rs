use bevy::app::{App, Plugin, Update};
use bevy::ecs::system::QueryLens;
use bevy::math::ops;
use bevy::prelude::{Color, Commands, Component, Entity, Mix, Mut, Query, Res, Sprite, Time};
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
    const DEFAULT_SPEED: f32 = 4.0;

    pub(crate) fn new(source: Color, target: Color) -> Self {
        Self {
            source,
            target,
            speed: Self::DEFAULT_SPEED,
        }
    }
    pub(crate) fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    pub(crate) fn remove_all(commands: &mut Commands, mut glower_q: QueryLens<(Entity, &mut Sprite, &Glower)>) {
        for (row, mut sprite, glower) in glower_q.query().iter_mut() {
            sprite.color = glower.source;
            commands.entity(row).remove::<Glower>();
        }
    }

    pub(crate) fn remove_all_optional(commands: &mut Commands, mut glower_q: QueryLens<(Entity, &mut Sprite, Option<&Glower>)>) {
        for (row, mut sprite, glower) in glower_q.query().iter_mut() {
            if let Some(glower) = glower {
                sprite.color = glower.source;
            }
            commands.entity(row).remove::<Glower>();
        }
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
    count: f32,
    delta_time: f32,
    speed: f32,
}

impl Blinker {
    const DEFAULT_COUNT: f32 = 1.0;
    const DEFAULT_SPEED: f32 = 1.0;

    pub(crate) fn new(source: Color, target: Color) -> Self {
        Self {
            source,
            target,
            count: Self::DEFAULT_COUNT,
            delta_time: 0.0,
            speed: Self::DEFAULT_SPEED,
        }
    }
    pub(crate) fn with_count(mut self, count: f32) -> Self {
        self.count = count;
        self
    }
    pub(crate) fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub(crate) fn remove(&self, commands: &mut Commands, sprite: &mut Mut<Sprite>, entity: Entity) {
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
        let x = (blink.delta_time * blink.speed) - (PI / 2.0);
        let t = (ops::sin(x) + 1.0) / 2.0;

        sprite.color = blink.source.mix(&blink.target, t);
        let target_time = (2.0 * PI * blink.count) / blink.speed;
        if blink.delta_time > target_time {
            blink.remove(&mut commands, &mut sprite, e);
        }
    }
}
