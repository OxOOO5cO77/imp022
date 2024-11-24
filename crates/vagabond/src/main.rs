use bevy::app::App;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_mod_picking::DefaultPickingPlugins;
use system::app_state::AppState;

mod manager;
mod network;
mod screen;
mod system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920.0, 1080.0).with_scale_factor_override(1.0),
                title: "Vagabond".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(pyri_tooltip::TooltipPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins((system::SystemPlugin, manager::ManagerPlugin, screen::ScreenPlugin))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(960.0, -540.0, 0.0);
    commands.spawn(camera);
}
