use std::time::Duration;

use bevy::prelude::*;
use bevy::ui::Val::Percent;

use crate::screen::shared::AppScreenExt;
use crate::system::AppState;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.build_screen(AppState::Splash, splash_enter, splash_update, splash_exit);
    }
}

#[derive(Component)]
struct SplashScreen {
    timer: Timer,
}

fn splash_enter(
    // bevy system
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            SplashScreen {
                timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
            },
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            BackgroundColor(Color::WHITE),
        ))
        .with_children(|parent| {
            parent.spawn(ImageNode {
                image: asset_server.load("image/impending.png"),
                ..default()
            });
        });
}

fn splash_update(
    // bevy system
    mut splash_q: Query<&mut SplashScreen>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let mut splash = splash_q.single_mut();
    splash.timer.tick(time.delta());

    if splash.timer.just_finished() || keyboard_input.any_pressed([KeyCode::Space, KeyCode::Enter]) || mouse_button.pressed(MouseButton::Left) {
        app_state.set(AppState::LoginDrawbridge);
    }
}

fn splash_exit(
    // bevy system
    mut commands: Commands,
    splash_q: Query<Entity, With<SplashScreen>>,
) {
    for e in &splash_q {
        commands.entity(e).despawn_recursive();
    }
}
