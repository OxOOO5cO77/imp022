use crate::screen::card_layout::CardPopulateEvent;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Entity, Event, Query, Resource, Transform, Trigger};
use vagabond::data::VagabondCard;

#[derive(Resource)]
pub(crate) struct CardTooltip(pub(crate) Entity);

#[derive(Event)]
pub(crate) struct UpdateCardTooltipEvent {
    pub(crate) position: Vec2,
    pub(crate) card: Option<VagabondCard>,
}

impl UpdateCardTooltipEvent {
    pub(crate) fn new(position: Vec2, card: Option<VagabondCard>) -> Self {
        Self {
            position,
            card,
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
        commands.entity(target).trigger(CardPopulateEvent::from(event.card.clone()));
    }
}
