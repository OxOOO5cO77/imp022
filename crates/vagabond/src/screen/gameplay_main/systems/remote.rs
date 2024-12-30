use bevy::prelude::{Commands, Entity, Query, Res, Sprite, Text2d, TextColor, Trigger, Visibility, With};

use crate::manager::AtlasManager;
use crate::screen::gameplay_main::components::{RemoteAttrIcon, RemoteAttrText};
use crate::screen::gameplay_main::events::{ResourcesTrigger, RollTrigger};
use crate::screen::shared::{replace_kind_icon, KindIconSize};

pub(super) fn on_remote_ui_update_roll(
    // bevy system
    _event: Trigger<RollTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut Text2d, &mut TextColor), With<RemoteAttrText>>,
    icon_q: Query<Entity, With<RemoteAttrIcon>>,
) {
    for (mut attr_text, mut color) in text_q.iter_mut() {
        *attr_text = "-".into();
        *color = bevy::color::palettes::basic::GRAY.into();
    }
    if let Ok(entity) = icon_q.get_single() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

pub(super) fn on_remote_ui_update_resources(
    // bevy system
    event: Trigger<ResourcesTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut Text2d, &mut TextColor, &RemoteAttrText)>,
    mut icon_q: Query<(Entity, &mut Sprite), With<RemoteAttrIcon>>,
    am: Res<AtlasManager>,
) {
    for (mut attr_text, mut color, remote_attr) in text_q.iter_mut() {
        *attr_text = event.remote_attr[remote_attr.index].to_string().into();
        *color = bevy::color::palettes::basic::RED.into();
    }
    if let Ok((entity, mut sprite)) = icon_q.get_single_mut() {
        replace_kind_icon(&mut sprite, event.remote_kind, KindIconSize::Large, &am);
        commands.entity(entity).insert(Visibility::Visible);
    }
}
