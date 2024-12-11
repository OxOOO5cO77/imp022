use bevy::prelude::{ops, Color, Commands, Component, Entity, Mix, Query, Res, Sprite, Time};

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

pub(crate) fn glower_update(
    //
    mut glower_q: Query<(&mut Sprite, &mut Glower)>,
    time: Res<Time>,
) {
    for (mut sprite, glow) in glower_q.iter_mut() {
        let t = (ops::sin(time.elapsed_secs() * glow.speed) + 1.0) / 2.0;
        sprite.color = glow.source.mix(&glow.target, t);
    }
}
