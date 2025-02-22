use bevy::prelude::{Query, ResMut, Text2d, Trigger};

use crate::screen::gameplay_main::TTY_MESSAGE_COUNT;
use crate::screen::gameplay_main::components::TTYMessageText;
use crate::screen::gameplay_main::events::TTYMessageTrigger;
use crate::screen::gameplay_main::resources::GameplayContext;

pub(super) fn on_tty_update(
    // bevy system
    event: Trigger<TTYMessageTrigger>,
    mut context: ResMut<GameplayContext>,
    mut text_q: Query<(&mut Text2d, &mut TTYMessageText)>,
) {
    let message = format!("[{:03}] {}", context.tick, event.message);
    let queue = context.tty.entry(event.kind).or_default();
    queue.push_front(message);
    if queue.len() > TTY_MESSAGE_COUNT {
        queue.pop_back();
    }

    for (mut text, tty) in text_q.iter_mut() {
        let queue = context.tty.entry(tty.kind).or_default();
        if let Some(message) = queue.get(tty.slot) {
            *text = message.clone().into();
        } else {
            *text = String::default().into();
        }
    }
}
