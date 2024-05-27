use bevy::app::App;
use bevy::prelude::*;

use system::app_state::AppState;
use crate::data::backend_manager::BackendManager;
use crate::screen::ScreenPlugins;
use crate::system::SystemPlugin;

mod data;
mod screen;
mod system;
mod game;

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(SystemPlugin)
        .add_plugins(ScreenPlugins)
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

    let bm = BackendManager::new().expect("[BackendManager] error]");
    commands.insert_resource(bm);
}
