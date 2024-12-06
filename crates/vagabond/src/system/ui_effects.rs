use bevy::app::{App, Plugin, Update};
use bevy::math::ops;
use bevy::prelude::{Commands, Component, Entity, Mix, Query, Res, Sprite, Srgba, Time};
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
    source: Option<Srgba>,
    target: Srgba,
    speed: f32,
}

impl Glower {
    const DEFAULT_SPEED: f32 = 4.0;

    pub(crate) fn new(target: Srgba) -> Self {
        Self {
            source: None,
            target,
            speed: Self::DEFAULT_SPEED,
        }
    }
    pub(crate) fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }
    pub(crate) fn original(&self) -> Option<Srgba> {
        self.source
    }
}

fn glower_update(
    //
    mut glower_q: Query<(&mut Sprite, &mut Glower)>,
    time: Res<Time>,
) {
    for (mut sprite, mut glow) in glower_q.iter_mut() {
        let t = (ops::sin(time.elapsed_secs() * glow.speed) + 1.0) / 2.0;
        if glow.source.is_none() {
            glow.source = Some(sprite.color.into());
        }
        sprite.color = glow.source.unwrap().mix(&glow.target, t).into();
    }
}

#[derive(Component)]
pub(crate) struct Blinker {
    source: Option<Srgba>,
    target: Srgba,
    count: f32,
    delta_time: f32,
    speed: f32,
}

impl Blinker {
    const DEFAULT_COUNT: f32 = 1.0;
    const DEFAULT_SPEED: f32 = 1.0;

    pub(crate) fn new(target: Srgba) -> Self {
        Self {
            source: None,
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
    pub(crate) fn original(&self) -> Option<Srgba> {
        self.source
    }
}

fn blinker_update(
    //
    mut commands: Commands,
    mut blinker_q: Query<(Entity, &mut Sprite, &mut Blinker)>,
    time: Res<Time>,
) {
    for (e, mut sprite, mut blink) in blinker_q.iter_mut() {
        if blink.source.is_none() {
            blink.source = Some(sprite.color.into());
        }

        blink.delta_time += time.delta().as_secs_f32();
        let x = (blink.delta_time * blink.speed) - (PI / 2.0);
        let t = (ops::sin(x) + 1.0) / 2.0;

        sprite.color = blink.source.unwrap().mix(&blink.target, t).into();
        let target_time = (2.0 * PI * blink.count) / blink.speed;
        if blink.delta_time > target_time {
            sprite.color = blink.original().unwrap_or(sprite.color.into()).into();
            commands.entity(e).remove::<Blinker>();
        }
    }
}
