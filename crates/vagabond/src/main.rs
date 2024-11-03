use bevy::app::App;
use bevy::prelude::*;

use system::app_state::AppState;

mod manager;
mod screen;
mod system;
mod network;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin)
        .add_plugins(pyri_tooltip::TooltipPlugin::default())
        .add_plugins(system::SystemPlugin)
        .add_plugins(manager::ManagerPlugin)
        .add_plugins(screen::ScreenPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .run()
    ;
}

fn setup(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            projection: OrthographicProjection { ..default() }.into(),
            transform: Transform::from_xyz(0.0, 10.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
    commands.spawn(
        PointLightBundle {
            point_light: PointLight { intensity: 9000.0, range: 100., shadows_enabled: true, ..default() },
            transform: Transform::from_xyz(8.0, 16.0, 8.0),
            ..default()
        }
    );
}
