use crate::screen::card_layout::CardPopulateEvent;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Entity, Event, Query, Resource, Transform, Trigger};
use hall::data::core::Attributes;
use vagabond::data::VagabondCard;

#[derive(Resource)]
pub(crate) struct CardTooltip(pub(crate) Entity);

#[derive(Event)]
pub(crate) struct UpdateCardTooltipEvent {
    position: Vec2,
    card: Option<VagabondCard>,
    attr: Attributes,
}

impl UpdateCardTooltipEvent {
    pub(crate) fn new(position: Vec2, card: Option<VagabondCard>, attributes: &Attributes) -> Self {
        Self {
            position,
            card,
            attr: attributes.clone(),
        }
    }
}

pub(crate) fn on_update_tooltip(
    // bevy system
    event: Trigger<UpdateCardTooltipEvent>,
    mut commands: Commands,
    mut tooltip_q: Query<&mut Transform>,
) {
    let target = event.entity();
    if let Ok(mut transform) = tooltip_q.get_mut(target) {
        transform.translation = Vec3::new(event.position.x, -event.position.y, transform.translation.z);
        commands.entity(target).trigger(CardPopulateEvent::new(event.card.clone(), event.attr.clone()));
    }
}
