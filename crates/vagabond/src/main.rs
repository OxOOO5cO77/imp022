use std::num::NonZero;

use bevy::prelude::*;
use bevy::sprite::Material2dPlugin;
use bevy::window::WindowResolution;
use mimalloc::MiMalloc;

use system::AppState;

use crate::gfx::FrameMaterial;

mod gfx;
mod manager;
mod network;
mod screen;
mod system;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

#[derive(Debug)]
#[allow(dead_code)] //Note: Derived Debug is intentionally ignored during dead code analysis
enum VagabondAppError {
    DotEnv(dotenvy::Error),
    Bevy(NonZero<u8>),
}

fn main() -> Result<(), VagabondAppError> {
    dotenvy::dotenv().map_err(VagabondAppError::DotEnv)?;
    bevy_main().map_err(VagabondAppError::Bevy)?;
    Ok(())
}

fn bevy_main() -> Result<(), NonZero<u8>> {
    let result = App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT).with_scale_factor_override(1.0),
                title: "Vagabond".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MeshPickingPlugin)
        .add_plugins(bevy_simple_text_input::TextInputPlugin)
        .add_plugins(Material2dPlugin::<FrameMaterial>::default())
        .add_plugins((manager::ManagerPlugins, screen::ScreenPlugins, system::SystemPlugins))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
    match result {
        AppExit::Success => Ok(()),
        AppExit::Error(code) => Err(code),
    }
}

fn setup(mut commands: Commands) {
    let transform = Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0, -WINDOW_HEIGHT / 2.0, 0.0));
    commands.spawn((Camera2d, transform));
}
