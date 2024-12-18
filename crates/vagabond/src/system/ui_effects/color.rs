use crate::gfx::FrameMaterial;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::Srgba;
use bevy::prelude::{Component, Entity, Event, MeshMaterial2d, Query, ResMut, Sprite, Trigger};

pub(crate) struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_observer(on_color_update_material)
            .add_observer(on_color_update_sprite);
    }
}

#[derive(Event)]
pub(crate) struct SetColorEvent {
    pub target: Entity,
    pub color: Srgba,
}

impl SetColorEvent {
    pub(crate) fn new(target: Entity, color: Srgba) -> Self {
        Self {
            target,
            color,
        }
    }
}

fn on_color_update_material(
    //
    event: Trigger<SetColorEvent>,
    material_q: Query<&MeshMaterial2d<FrameMaterial>>,
    mut materials: ResMut<Assets<FrameMaterial>>,
) {
    if let Ok(material) = material_q.get(event.target) {
        if let Some(instance) = materials.get_mut(&material.0) {
            instance.color = event.color.into();
        }
    }
}

fn on_color_update_sprite(
    //
    event: Trigger<SetColorEvent>,
    mut sprite_q: Query<&mut Sprite>,
) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.entity()) {
        sprite.color = event.color.into();
    }
}

#[derive(Component)]
pub(crate) struct UiFxTrackedColor {
    pub color: Srgba,
}

impl From<Srgba> for UiFxTrackedColor {
    fn from(color: Srgba) -> Self {
        Self {
            color,
        }
    }
}
