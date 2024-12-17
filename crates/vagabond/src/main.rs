use crate::gfx::FrameMaterial;
use bevy::app::App;
use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::WindowResolution;
use system::AppState;

mod gfx;
mod manager;
mod network;
mod screen;
mod system;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
                title: "Vagabond".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_simple_text_input::TextInputPlugin)
        .add_plugins(Material2dPlugin::<FrameMaterial>::default())
        .add_plugins((manager::ManagerPlugins, screen::ScreenPlugins, system::SystemPlugins))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_translation(Vec3::new(960.0, -540.0, 0.0));
    commands.spawn((Camera2d, transform));
}
