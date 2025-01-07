use crate::system::ui_effects::{Hider, UiFxTrackedSize};
use bevy::math::Vec3;
use bevy::picking::PickingBehavior;
use bevy::prelude::{Commands, Component, Entity, EntityCommands, Out, Over, Pointer, Query, Transform, Trigger, Visibility, Window};
use bevy::text::Text2d;

const TEXT_TIP_DELAY_SHOW: f32 = 2.0;
const TEXT_TIP_DELAY_HIDE: f32 = 0.25;

#[derive(Component)]
struct TextTipContainer {
    text_entity: Entity,
}

impl TextTipContainer {
    fn new(text_entity: Entity) -> Self {
        Self {
            text_entity,
        }
    }
}

#[derive(Component)]
struct TextTipComponent {
    text: String,
    container_entity: Entity,
}

impl TextTipComponent {
    fn new(container_entity: Entity, text: String) -> Self {
        Self {
            text,
            container_entity,
        }
    }
}

pub(crate) trait TextTip {
    fn insert_text_tip_container(self, text_entity: Entity) -> Self;
    fn insert_text_tip(self, container_entity: Entity, text: &str) -> Self;
}

impl TextTip for &mut EntityCommands<'_> {
    fn insert_text_tip_container(self, text_entity: Entity) -> Self {
        self //
            .insert((TextTipContainer::new(text_entity), Visibility::Hidden))
    }
    fn insert_text_tip(self, container_entity: Entity, text: &str) -> Self {
        self //
            .insert((TextTipComponent::new(container_entity, text.to_string()), PickingBehavior::default()))
            .observe(on_over_text_tip)
            .observe(on_out_text_tip)
    }
}

fn on_over_text_tip(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    text_tip_q: Query<&TextTipComponent>,
    mut container_q: Query<(&mut Transform, &UiFxTrackedSize, &TextTipContainer)>,
    mut text_q: Query<&mut Text2d>,
    window_q: Query<&Window>,
) {
    let window = window_q.single();
    if let Ok(text_tip) = text_tip_q.get(event.target) {
        if let Ok((mut container_transform, container_size, container)) = container_q.get_mut(text_tip.container_entity) {
            if let Ok(mut text) = text_q.get_mut(container.text_entity) {
                *text = text_tip.text.clone().into();
            }
            let x = event.pointer_location.position.x.clamp(0.0, window.width() - container_size.x);
            let y = event.pointer_location.position.y.clamp(0.0, window.height() - container_size.y);
            container_transform.translation = Vec3::new(x, -y, container_transform.translation.z);

            commands.entity(text_tip.container_entity).insert(Hider::new(TEXT_TIP_DELAY_SHOW, Visibility::Visible));
        }
    }
}

fn on_out_text_tip(
    //
    event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    text_tip_q: Query<&TextTipComponent>,
) {
    if let Ok(text_tip) = text_tip_q.get(event.target) {
        commands.entity(text_tip.container_entity).insert(Hider::new(TEXT_TIP_DELAY_HIDE, Visibility::Hidden));
    }
}
