use bevy::prelude::{Commands, Entity, Query, Res, Trigger, Visibility, With};

use crate::screen::compose_main::resources::ComposeContext;
use crate::screen::compose_main::{CardHeader, DeckGutterGroup, PopulatePlayerUi};
use crate::screen::shared::CardPopulateEvent;

pub(super) fn on_populate_deck_ui(
    // bevy system
    event: Trigger<PopulatePlayerUi>,
    mut commands: Commands,
    header_q: Query<(Entity, &CardHeader)>,
    gutter_q: Query<Entity, With<DeckGutterGroup>>,
    context: Res<ComposeContext>,
) {
    let visibility = match *event {
        PopulatePlayerUi::Hide => {
            commands.trigger_targets(CardPopulateEvent::default(), header_q.iter().map(|(e, _)| e).collect::<Vec<_>>());
            Visibility::Hidden
        }
        PopulatePlayerUi::Show(_) => {
            for (idx, card) in context.deck.iter().enumerate() {
                if let Some((entity, _)) = header_q.iter().find(|(_, h)| h.index == idx) {
                    commands.entity(entity).trigger(CardPopulateEvent::new(Some(card.clone()), context.attributes));
                }
            }
            Visibility::Visible
        }
    };
    if let Ok(gutter) = gutter_q.get_single() {
        commands.entity(gutter).insert(visibility);
    }
}
