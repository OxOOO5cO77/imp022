use bevy::app::App;
use bevy::prelude::*;
use crate::app_state::AppState;
use crate::data::data_manager::DataManager;
use crate::screen::ScreenPlugins;
use crate::system::SystemPlugin;

mod data;
mod screen;
mod app_state;
mod system;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(SystemPlugin)
        .add_plugins(ScreenPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            ..default()
        }.into(),
        transform: Transform::from_xyz(0.0, 10.0, 0.1).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    let dm = DataManager::new().expect("[DataManager] error]");
    commands.insert_resource(dm);
}
