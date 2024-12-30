use bevy::prelude::{Commands, Entity, Query, Res, Trigger};

use hall::data::core::Attributes;

use crate::manager::DataManager;
use crate::screen::gameplay_main::components::HandCard;
use crate::screen::gameplay_main::events::PlayerStateTrigger;
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::shared::CardPopulateEvent;

pub(super) fn on_hand_ui_update(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut commands: Commands,
    hand_q: Query<(Entity, &HandCard)>,
    dm: Res<DataManager>,
    context: Res<GameplayContext>,
) {
    for (entity, hand) in &hand_q {
        let card = event.state.hand.get(hand.index).and_then(|o| dm.convert_card(o));
        commands.entity(entity).trigger(CardPopulateEvent::new(card, Attributes::from_arrays(context.cached_state.attr)));
    }
}
