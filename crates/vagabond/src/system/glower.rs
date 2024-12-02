use bevy::app::{App, Plugin, Update};
use bevy::math::ops;
use bevy::prelude::{Component, Mix, Query, Res, Sprite, Srgba, Time};

pub struct GlowerPlugin;

impl Plugin for GlowerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Update, glower_update);
    }
}

#[derive(Component)]
pub struct Glower {
    source: Option<Srgba>,
    target: Srgba,
}

impl Glower {
    pub fn new(target: Srgba) -> Self {
        Self {
            source: None,
            target,
        }
    }
    pub fn original(&self) -> Option<Srgba> {
        self.source
    }
}

fn glower_update(
    //
    mut glower_q: Query<(&mut Sprite, &mut Glower)>,
    time: Res<Time>,
) {
    let t = (ops::sin(time.elapsed_secs() * 4.0) + 1.0) / 2.0;

    for (mut sprite, mut glow) in glower_q.iter_mut() {
        if glow.source.is_none() {
            glow.source = Some(sprite.color.into());
        }
        sprite.color = glow.source.unwrap().mix(&glow.target, t).into();
    }
}
