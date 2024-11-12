use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct Screen;

#[derive(Bundle)]
pub(crate) struct ScreenBundle<T: Bundle> {
    pub(crate) screen: Screen,
    pub(crate) base: T,
}

impl Default for ScreenBundle<NodeBundle> {
    fn default() -> Self {
        Self {
            screen: Screen,
            base: filled_rect(ZERO, ZERO, HUNDRED, HUNDRED, Srgba::BLACK),
        }
    }
}

pub(crate) const ZERO: Val = Val::Percent(0.0);
pub(crate) const HUNDRED: Val = Val::Percent(100.0);

pub(crate) fn screen_exit(mut commands: Commands, screen_q: Query<Entity, With<Screen>>) {
    commands.entity(screen_q.single()).despawn_recursive();
}

pub(crate) fn filled_rect(left: Val, top: Val, width: Val, height: Val, color: Srgba) -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left,
            top,
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            align_self: AlignSelf::Center,
            ..default()
        },
        background_color: color.into(),
        ..default()
    }
}

#[derive(Clone)]
pub struct FontInfo {
    pub(crate) handle: Handle<Font>,
    pub(crate) size: f32,
    pub(crate) color: Color,
}

pub(crate) fn text(text: impl Into<String>, font_info: &FontInfo) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font: font_info.handle.clone(),
            font_size: font_info.size,
            color: font_info.color,
        },
    ).with_style(Style {
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        ..default()
    })
}

pub(crate) fn font_size(asset_server: &Res<AssetServer>, size: f32) -> FontInfo {
    let font = asset_server.load("font/RobotoMono.ttf");
    FontInfo {
        handle: font.clone(),
        size,
        color: Color::BLACK,
    }
}
pub(crate) fn font_size_color(asset_server: &Res<AssetServer>, size: f32, color: impl Into<Color>) -> FontInfo {
    let mut info = font_size(asset_server, size);
    info.color = color.into();
    info
}
