use std::time::Duration;

use bevy::prelude::*;

use crate::system::app_state::AppState;
use crate::system::ui::HUNDRED;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Splash), splash_enter)
            .add_systems(Update, splash_update.run_if(in_state(AppState::Splash)))
            .add_systems(OnExit(AppState::Splash), splash_exit);
    }
}

#[derive(Component)]
struct Splash {
    timer: Timer,
}

fn splash_enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Splash { timer: Timer::new(Duration::from_secs(3), TimerMode::Once) }, NodeBundle { style: Style { width: HUNDRED, height: HUNDRED, position_type: PositionType::Absolute, justify_content: JustifyContent::Center, align_items: AlignItems::Center, align_content: AlignContent::Center, ..default() }, background_color: Color::WHITE.into(), ..default() })).with_children(|parent| {
        parent.spawn(ImageBundle { image: asset_server.load("image/impending.png").into(), ..default() });
    });
}

fn splash_update(
    mut splash_q: Query<&mut Splash>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    let mut splash = splash_q.single_mut();
    splash.timer.tick(time.delta());

    if splash.timer.just_finished() || keyboard_input.pressed(KeyCode::Space) || mouse_button.pressed(MouseButton::Left) {
        app_state.set(AppState::LoginDrawbridge);
    }
}

fn splash_exit(mut commands: Commands, splash_q: Query<Entity, With<Splash>>) {
    let splash = splash_q.single();
    commands.entity(splash).despawn_recursive();
}
