use std::time::Duration;
use bevy::prelude::*;
use crate::app_state::AppState;
use crate::system::ui::HUNDRED;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::SplashScreen), splash_enter)
            .add_systems(Update, splash_update.run_if(in_state(AppState::SplashScreen)))
            .add_systems(OnExit(AppState::SplashScreen), splash_exit)
        ;
    }
}

#[derive(Component)]
struct Splash {
    timer: Timer,
}

fn splash_enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            Splash { timer: Timer::new(Duration::from_secs(3), TimerMode::Once) },
            NodeBundle {
                style: Style {
                    width: HUNDRED,
                    height: HUNDRED,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                ..default()
            }
        ))
        .with_children(|parent| {
            parent
                .spawn(
                    ImageBundle {
                        image: asset_server.load("image/impending.png").into(),
                        ..default()
                    }
                );
        })
    ;
}

fn splash_update(
    mut splash_q: Query<&mut Splash>,
    mut app_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    let mut splash = splash_q.single_mut();
    splash.timer.tick(time.delta());

    if splash.timer.just_finished() {
        app_state.set(AppState::ComposeScreen);
    }
}

fn splash_exit(
    mut commands: Commands,
    splash_q: Query<Entity, With<Splash>>,
) {
    let splash = splash_q.single();
    commands.entity(splash).despawn_recursive();
}
