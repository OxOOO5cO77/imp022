use bevy::prelude::{Commands, Query, Res, ResMut, On};

use hall_lib::core::AttributeValues;

use crate::network::client_gate::GateIFace;
use crate::screen::compose_main::resources::ComposeContext;
use crate::screen::compose_main::{ComposeState, FinishPlayerTrigger, PartHolder, PopulatePlayerUi, Slot, StatRowKind};

fn attributes_from_holder(holder: &PartHolder) -> AttributeValues {
    AttributeValues::from_array(holder.part.as_ref().map(|o| o.values).unwrap_or_default())
}

fn seed_from_holder(holder: &PartHolder) -> u64 {
    holder.part.as_ref().map(|o| o.seed).unwrap_or_default()
}

pub(super) fn on_finish_player(
    // bevy system
    _event: On<FinishPlayerTrigger>,
    mut commands: Commands,
    holder_q: Query<(&PartHolder, &Slot)>,
    gate: Res<GateIFace>,
    mut context: ResMut<ComposeContext>,
) {
    let mut parts = [0, 0, 0, 0, 0, 0, 0, 0];

    for (holder, holder_kind) in holder_q.iter() {
        if let Some(idx) = match holder_kind {
            Slot::StatRow(row) => match row {
                StatRowKind::Analyze => {
                    context.attributes.analyze = attributes_from_holder(holder);
                    Some(0)
                }
                StatRowKind::Breach => {
                    context.attributes.breach = attributes_from_holder(holder);
                    Some(1)
                }
                StatRowKind::Compute => {
                    context.attributes.compute = attributes_from_holder(holder);
                    Some(2)
                }
                StatRowKind::Disrupt => {
                    context.attributes.disrupt = attributes_from_holder(holder);
                    Some(3)
                }
                StatRowKind::Build => Some(5),
                StatRowKind::Detail => Some(7),
            },
            Slot::Build => Some(4),
            Slot::Detail => Some(6),
            Slot::Card => None,
            Slot::Empty(_) => None,
        } {
            parts[idx] = seed_from_holder(holder);
        }
    }

    if parts.iter().all(|&o| o != 0) {
        if context.state == ComposeState::Build {
            context.state = ComposeState::Ready;
        }
        gate.send_game_build(parts, context.state == ComposeState::Committed);
    } else {
        context.state = ComposeState::Build;
        commands.trigger(PopulatePlayerUi::Hide);
    }
}
