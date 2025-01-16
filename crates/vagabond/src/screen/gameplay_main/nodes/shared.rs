use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::{Bundle, Commands, Entity, EntityCommand, Event, Observer, Over, Pointer, Res, Srgba, Trigger, World};

use crate::screen::gameplay_main::components::MissionNodeLocalObserver;
use crate::screen::gameplay_main::nodes::MissionNodeAction;
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::gameplay_main::VagabondGamePhase;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};
use hall::core::MissionNodeIntent;

pub(crate) fn on_over_node_action(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    context: Res<GameplayContext>,
) {
    if context.phase != VagabondGamePhase::Start {
        return;
    }
    commands.entity(event.target).trigger(SetColorEvent::new(event.target, bevy::color::palettes::basic::WHITE));
}

pub(crate) fn deselect_node_action(commands: &mut Commands, context: &GameplayContext) {
    if let Some(entity) = context.node_action.entity {
        commands.entity(entity).trigger(SetColorEvent::new(entity, context.node_action.color)).insert(UiFxTrackedColor::from(context.node_action.color));
    }
}

pub(crate) fn click_common<T>(
    //
    commands: &mut Commands,
    context: &mut GameplayContext,
    target: Entity,
    color: Srgba,
    data: T,
    callback: fn(data: T) -> MissionNodeIntent,
) {
    if context.phase != VagabondGamePhase::Start {
        return;
    }

    deselect_node_action(commands, context);

    let (intent, entity) = if context.node_action.entity.is_some_and(|e| e == target) {
        (None, None)
    } else {
        let color = bevy::color::palettes::basic::GREEN;
        commands.entity(target).trigger(SetColorEvent::new(target, color)).insert(UiFxTrackedColor::from(color));
        (Some(callback(data)), Some(target))
    };

    context.node_action = MissionNodeAction::new(intent, entity, color);
}

// local copy of observe to decorate observers with NodeLocalObserver to easily dispose before reactivation
pub(crate) fn local_observe<E: Event, B: Bundle, M>(observer: impl IntoObserverSystem<E, B, M>) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Ok(mut world_entity) = world.get_entity_mut(entity) {
            world_entity.world_scope(|w| {
                w.spawn(Observer::new(observer).with_entity(entity)).insert(MissionNodeLocalObserver);
            });
        }
    }
}
