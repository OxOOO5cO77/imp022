use bevy::ecs::query::QueryEntityError;
use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::{Bundle, Commands, Entity, EntityCommand, EntityWorldMut, Event, Observer, Over, Pointer, Res, On, EntityEvent};

use hall_lib::core::MissionNodeIntent;

use crate::screen::gameplay_main::components::{MissionNodeButton, MissionNodeLocalObserver};
use crate::screen::gameplay_main::nodes::MissionNodeAction;
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::gameplay_main::VagabondGamePhase;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

pub(crate) fn on_over_node_action(
    //
    event: On<Pointer<Over>>,
    mut commands: Commands,
    context: Res<GameplayContext>,
) {
    if context.phase != VagabondGamePhase::Start {
        return;
    }
    commands.entity(event.event_target()).trigger(|e| SetColorEvent::new(e, bevy::color::palettes::basic::WHITE));
}

pub(crate) fn deselect_node_action(commands: &mut Commands, context: &GameplayContext) {
    if let Some(entity) = context.node_action.entity {
        commands.entity(entity).trigger(|e| SetColorEvent::new(e, context.node_action.color)).insert(UiFxTrackedColor::from(context.node_action.color));
    }
}

pub(crate) fn click_common<T: Clone>(
    //
    commands: &mut Commands,
    context: &mut GameplayContext,
    target: Entity,
    query_result: Result<(&MissionNodeButton<T>, &UiFxTrackedColor), QueryEntityError>,
    callback: fn(data: T) -> MissionNodeIntent,
) {
    if context.phase != VagabondGamePhase::Start {
        return;
    }

    deselect_node_action(commands, context);

    if let Ok((button, new_color)) = query_result {
        let (intent, entity) = if context.node_action.entity.is_some_and(|e| e == target) {
            (None, None)
        } else {
            let color = bevy::color::palettes::basic::GREEN;
            commands.entity(target).trigger(|e| SetColorEvent::new(e, color)).insert(UiFxTrackedColor::from(color));
            (Some(callback(button.data.clone())), Some(target))
        };

        context.node_action = MissionNodeAction::new(intent, entity, new_color.color);
    }
}

// local copy of observe to decorate observers with NodeLocalObserver to easily dispose before reactivation
pub(crate) fn local_observe<E: Event, B: Bundle, M>(observer: impl IntoObserverSystem<E, B, M>) -> impl EntityCommand {
    move |mut entity_world: EntityWorldMut| {
        let entity = entity_world.id();
        entity_world.world_scope(|w| {
            w.spawn(Observer::new(observer).with_entity(entity)).insert(MissionNodeLocalObserver);
        });
    }
}
