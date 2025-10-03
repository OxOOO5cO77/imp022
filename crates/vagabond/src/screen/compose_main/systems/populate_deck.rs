use bevy::prelude::{Commands, Entity, Query, Res, On, Visibility, With};

use crate::screen::compose_main::resources::ComposeContext;
use crate::screen::compose_main::{CardHeader, DeckGutterGroup, PopulatePlayerUi};
use crate::screen::shared::CardPopulateEvent;

pub(super) fn on_populate_deck_ui(
    // bevy system
    event: On<PopulatePlayerUi>,
    mut commands: Commands,
    header_q: Query<(Entity, &CardHeader)>,
    gutter_q: Query<Entity, With<DeckGutterGroup>>,
    context: Res<ComposeContext>,
) {
    let visibility = match *event {
        PopulatePlayerUi::Hide => {
            //header_q.iter().map(|(e, _)| e).collect::<Vec<_>>()
            for (entity, _header) in header_q.iter() {
                commands.trigger(CardPopulateEvent::empty(entity));
            }
            Visibility::Hidden
        }
        PopulatePlayerUi::Show(_) => {
            for (idx, card) in context.deck.iter().enumerate() {
                if let Some((entity, _)) = header_q.iter().find(|(_, h)| h.index == idx) {
                    commands.entity(entity).trigger(|e| CardPopulateEvent::new(e, Some(card.clone()), context.attributes));
                }
            }
            Visibility::Visible
        }
    };
    if let Ok(gutter) = gutter_q.single() {
        commands.entity(gutter).insert(visibility);
    }
}
