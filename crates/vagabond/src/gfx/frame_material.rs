use bevy::asset::Asset;
use bevy::prelude::{LinearRgba, TypePath, Vec2};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{AlphaMode2d, Material2d};

const SHADER_ASSET_PATH: &str = "shader/frame_material.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub(crate) struct FrameMaterial {
    #[uniform(0)]
    pub(crate) color: LinearRgba,
    #[uniform(1)]
    size: Vec2,
    #[uniform(2)]
    frame_width: f32,
    #[uniform(3)]
    dash_size: f32,
}

impl FrameMaterial {
    pub(crate) fn new(color: LinearRgba, size: Vec2, dash_size: f32) -> Self {
        Self {
            color,
            size,
            frame_width: 4.0,
            dash_size,
        }
    }
}

impl Material2d for FrameMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
