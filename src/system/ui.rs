use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Screen;

#[derive(Bundle)]
pub(crate) struct ScreenBundle {
    screen: Screen,
    node: NodeBundle,
}

impl Default for ScreenBundle {
    fn default() -> Self {
        Self { screen: Screen, node: filled_rect(ZERO, ZERO, HUNDRED, HUNDRED, Color::BLACK) }
    }
}

pub(crate) const ZERO: Val = Val::Percent(0.0);
pub(crate) const HUNDRED: Val = Val::Percent(100.0);

pub(crate) fn screen_exit(mut commands: Commands, screen_q: Query<Entity, With<Screen>>) {
    commands.entity(screen_q.single()).despawn_recursive();
}

pub(crate) fn filled_rect(left: Val, top: Val, width: Val, height: Val, color: Color) -> NodeBundle {
    NodeBundle { style: Style { position_type: PositionType::Absolute, left, top, width, height, justify_content: JustifyContent::Center, align_items: AlignItems::Center, align_content: AlignContent::Center, align_self: AlignSelf::Center, ..default() }, background_color: color.into(), ..default() }
}
