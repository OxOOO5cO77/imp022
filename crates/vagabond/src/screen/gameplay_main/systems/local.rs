use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Srgba, Text2d, TextColor, Trigger, With};

use crate::manager::DataManager;
use crate::screen::gameplay_main::components::{AttributeRow, PlayerStateText};
use crate::screen::gameplay_main::events::{ChooseAttrTrigger, PlayerStateTrigger};
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::gameplay_main::{map_kind_to_index, VagabondGamePhase, GLOWER_SPEED};
use crate::system::ui_effects::{Glower, UiFxTrackedColor};

pub(super) fn on_local_state_update_player(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut context: ResMut<GameplayContext>,
    dm: Res<DataManager>,
) {
    context.cached_state = event.state.clone();
    context.hand = event.state.hand.iter().filter_map(|card| dm.convert_card(card)).collect();
}

pub(super) fn on_local_ui_update_player(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut text_q: Query<(&mut Text2d, &PlayerStateText)>,
) {
    for (mut text, state_text) in text_q.iter_mut() {
        match state_text {
            PlayerStateText::Attribute(row, col) => *text = format!("{}", event.state.attr[*row][*col]).into(),
            PlayerStateText::Deck => *text = event.state.deck.to_string().into(),
            PlayerStateText::Heap => *text = event.state.heap.len().to_string().into(),
            _ => {}
        }
    }
}

pub(super) fn on_local_ui_update_attr(
    // bevy system
    event: Trigger<ChooseAttrTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut TextColor, &PlayerStateText)>,
    mut row_q: Query<(Entity, &UiFxTrackedColor, Option<&Glower>), With<AttributeRow>>,
    mut context: ResMut<GameplayContext>,
) {
    if context.phase != VagabondGamePhase::Pick {
        return;
    }

    if event.kind.is_none() {
        for (entity, source_color, _) in row_q.iter() {
            let color = source_color.color;
            commands.entity(entity).insert(Glower::new(color, Srgba::new(0.0, 1.0, 0.0, 1.0), GLOWER_SPEED));
        }
    } else {
        for (entity, _, glower) in row_q.iter_mut() {
            if let Some(glower) = glower {
                glower.remove(&mut commands, entity);
            }
        }
    }

    for (mut color, state_text) in text_q.iter_mut() {
        if let PlayerStateText::Attribute(row, _) = state_text {
            *color = if let Some(kind) = event.kind {
                if *row == map_kind_to_index(kind) {
                    context.attr_pick = Some(kind);
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::GRAY
                }
            } else {
                bevy::color::palettes::basic::GRAY
            }
            .into()
        }
    }
}
