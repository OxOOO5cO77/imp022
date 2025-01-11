use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Commands, Entity, Event, Query, Resource, Transform, Trigger, Window};

use hall::core::Attributes;
use vagabond::data::VagabondCard;

use crate::screen::shared::card_layout::CardPopulateEvent;
use crate::system::ui_effects::UiFxTrackedSize;

#[derive(Resource)]
pub(crate) struct CardTooltip {
    pub(crate) entity: Entity,
}

impl CardTooltip {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
        }
    }
}

#[derive(Event)]
pub(crate) struct UpdateCardTooltipEvent {
    position: Vec2,
    card: Option<VagabondCard>,
    attr: Attributes,
}

impl UpdateCardTooltipEvent {
    pub(crate) fn new(position: Vec2, card: Option<VagabondCard>, attr: Attributes) -> Self {
        Self {
            position,
            card,
            attr,
        }
    }
}

pub(crate) fn on_update_tooltip(
    // bevy system
    event: Trigger<UpdateCardTooltipEvent>,
    mut commands: Commands,
    mut tooltip_q: Query<(&mut Transform, &UiFxTrackedSize)>,
    window_q: Query<&Window>,
) {
    let target = event.entity();
    let window = window_q.single();

    if let Ok((mut transform, tooltip_size)) = tooltip_q.get_mut(target) {
        let x = event.position.x.clamp(0.0, window.width() - tooltip_size.x);
        let y = event.position.y.clamp(0.0, window.height() - tooltip_size.y);
        transform.translation = Vec3::new(x, -y, transform.translation.z);
        commands.entity(target).trigger(CardPopulateEvent::new(event.card.clone(), event.attr));
    }
}
