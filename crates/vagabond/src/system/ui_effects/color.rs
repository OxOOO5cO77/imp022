use crate::gfx::FrameMaterial;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::Srgba;
use bevy::prelude::{ColorMaterial, Component, Entity, Event, MeshMaterial2d, Query, ResMut, Sprite, Trigger};
use bevy::sprite::Material2d;

pub(crate) struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_observer(on_color_update_frame_material)
            .add_observer(on_color_update_color_material)
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

trait SetColorMaterial: Material2d {
    fn set_color(&mut self, color: Srgba);
}

impl SetColorMaterial for FrameMaterial {
    fn set_color(&mut self, color: Srgba) {
        self.color = color.into();
    }
}

impl SetColorMaterial for ColorMaterial {
    fn set_color(&mut self, color: Srgba) {
        self.color = color.into();
    }
}

fn set_color_common<T>(event: Trigger<SetColorEvent>, material_q: Query<&MeshMaterial2d<T>>, mut materials: ResMut<Assets<T>>)
where
    T: SetColorMaterial,
{
    if let Ok(material) = material_q.get(event.target)
        && let Some(instance) = materials.get_mut(&material.0)
    {
        instance.set_color(event.color);
    }
}

fn on_color_update_frame_material(
    //
    event: Trigger<SetColorEvent>,
    material_q: Query<&MeshMaterial2d<FrameMaterial>>,
    materials: ResMut<Assets<FrameMaterial>>,
) {
    set_color_common(event, material_q, materials);
}

fn on_color_update_color_material(
    //
    event: Trigger<SetColorEvent>,
    material_q: Query<&MeshMaterial2d<ColorMaterial>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    set_color_common(event, material_q, materials);
}

fn on_color_update_sprite(
    //
    event: Trigger<SetColorEvent>,
    mut sprite_q: Query<&mut Sprite>,
) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target()) {
        sprite.color = event.color.into();
    }
}

#[derive(Component)]
pub(crate) struct UiFxTrackedColor {
    pub(crate) color: Srgba,
}

impl From<Srgba> for UiFxTrackedColor {
    fn from(color: Srgba) -> Self {
        Self {
            color,
        }
    }
}
