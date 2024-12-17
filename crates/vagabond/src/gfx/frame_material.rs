use bevy::asset::Asset;
use bevy::prelude::{LinearRgba, TypePath, Vec2};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

const SHADER_ASSET_PATH: &str = "shader/frame_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub(crate) struct FrameMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[uniform(1)]
    size: Vec2,
    #[uniform(2)]
    frame_width: f32,
}

impl FrameMaterial {
    pub(crate) fn new(color: LinearRgba, size: Vec2) -> Self {
        Self {
            color,
            size,
            frame_width: 4.0,
        }
    }
}

impl Material2d for FrameMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
