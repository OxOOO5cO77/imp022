use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Sprite, Text2d, Trigger};

use hall::data::core::Attributes;
use hall::data::game::{GameMachinePlayerView, GameProcessPlayerView};
use vagabond::data::{VagabondMachine, VagabondProcess};

use crate::manager::DataManager;
use crate::screen::gameplay_main::components::{MachineKind, MachineQueueItem, MachineRunning, MachineText, MachineTextKind};
use crate::screen::gameplay_main::events::{MachineInfoTrigger, MachineStateTrigger};
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::shared::CardPopulateEvent;

fn convert_process(process: &GameProcessPlayerView, dm: &DataManager) -> Option<VagabondProcess> {
    let vagabond_process = VagabondProcess {
        card: dm.convert_card(&process.player_card)?,
        priority: process.priority,
        local: process.local,
    };
    Some(vagabond_process)
}

fn cache_game_machine(machine: &GameMachinePlayerView, dm: &DataManager) -> VagabondMachine {
    VagabondMachine {
        vitals: machine.vitals,
        queue: machine.queue.iter().filter_map(|(process, delay)| convert_process(process, dm).map(|p| (p, *delay))).collect(),
        running: machine.running.iter().filter_map(|p| convert_process(p, dm)).collect(),
    }
}

pub(super) fn on_machine_ui_update_info(
    // bevy system
    event: Trigger<MachineInfoTrigger>,
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
) {
    for (machine_component, mut text, machine_text) in text_q.iter_mut() {
        if *machine_component == event.kind {
            match machine_text.kind {
                MachineTextKind::Title => *text = event.name.clone().into(),
                MachineTextKind::Id => *text = event.id.clone().into(),
                MachineTextKind::Vitals(_) => {}
            }
        }
    }
}

pub(super) fn on_machine_ui_update_state(
    // bevy system
    event: Trigger<MachineStateTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
    mut sprite_q: Query<(&MachineKind, &mut Sprite, &MachineQueueItem)>,
    running_q: Query<(Entity, &MachineKind, &MachineRunning)>,
    dm: Res<DataManager>,
    mut context: ResMut<GameplayContext>,
) {
    context.cached_local = cache_game_machine(&event.local, &dm);
    context.cached_remote = cache_game_machine(&event.remote, &dm);

    for (machine_component, mut text, machine_text) in text_q.iter_mut() {
        if let MachineTextKind::Vitals(index) = machine_text.kind {
            let machine = if *machine_component == MachineKind::Local {
                &event.local
            } else {
                &event.remote
            };
            *text = machine.vitals[index].to_string().into();
        }
    }

    for (machine_component, mut sprite, queue_item) in sprite_q.iter_mut() {
        let (machine, player_owned) = if *machine_component == MachineKind::Local {
            (&event.local, true)
        } else {
            (&event.remote, false)
        };

        sprite.color = if let Some(process) = machine.queue.iter().find(|(_, delay)| *delay == queue_item.delay).map(|(item, _)| item) {
            if process.local == player_owned {
                bevy::color::palettes::basic::GREEN
            } else {
                bevy::color::palettes::basic::RED
            }
        } else {
            bevy::color::palettes::basic::WHITE
        }
        .into();
    }

    for (entity, machine_component, running) in running_q.iter() {
        let machine = if *machine_component == MachineKind::Local {
            &event.local
        } else {
            &event.remote
        };
        let card = machine.running.get(running.index).and_then(|process| dm.convert_card(&process.player_card));
        commands.entity(entity).trigger(CardPopulateEvent::new(card, Attributes::from_arrays(context.cached_state.attr)));
    }
}
