use std::cmp::Ordering;

use bevy::prelude::{Query, Text2d, TextColor, Trigger};

use crate::screen::gameplay_main::components::RollText;
use crate::screen::gameplay_main::events::{ResourcesTrigger, RollTrigger};

pub(super) fn on_roll_ui_update_roll(
    // bevy system
    event: Trigger<RollTrigger>,
    mut roll_q: Query<(&mut Text2d, &mut TextColor, &RollText)>,
) {
    for (mut text, mut color, roll) in roll_q.iter_mut() {
        *text = format!("{}", event.roll[roll.index]).into();
        *color = bevy::color::palettes::basic::GRAY.into();
    }
}

pub(super) fn on_roll_ui_update_resources(
    // bevy system
    event: Trigger<ResourcesTrigger>,
    mut roll_q: Query<(&mut TextColor, &RollText)>,
) {
    for (mut color, roll) in roll_q.iter_mut() {
        *color = match event.local_erg[roll.index].cmp(&event.remote_erg[roll.index]) {
            Ordering::Less => bevy::color::palettes::basic::RED,
            Ordering::Equal => bevy::color::palettes::basic::YELLOW,
            Ordering::Greater => bevy::color::palettes::basic::GREEN,
        }
        .into();
    }
}
