use bevy::prelude::{ops, Color, Commands, Component, Entity, Mix, Query, Res, Sprite, Time};
use std::f32::consts::PI;

#[derive(Component)]
pub(crate) struct Blinker {
    original: Color,
    source: Color,
    target: Color,
    delta_time: f32,
    target_time: f32,
    speed: f32,
}

impl Blinker {
    pub(crate) fn new(original: Color, target: Color, count: f32, speed: f32) -> Self {
        Self {
            original,
            source: (original.to_srgba() / 2.0).into(),
            target,
            delta_time: 0.0,
            target_time: (2.0 * PI * count) / speed,
            speed,
        }
    }

    pub(crate) fn remove(&self, commands: &mut Commands, sprite: &mut Sprite, entity: Entity) {
        sprite.color = self.original;
        commands.entity(entity).remove::<Blinker>();
    }
}

pub(crate) fn blinker_update(
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
