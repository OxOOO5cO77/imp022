use bevy::ecs::system::IntoObserverSystem;
use bevy::prelude::*;

use hall::data::core::{MissionNodeContent, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir};
use hall::data::game::GameMissionNodePlayerView;

use crate::manager::ScreenLayout;
use crate::screen::gameplay_main::components::{MissionNodeContentButton, MissionNodeDisplay, MissionNodeLinkButton};
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::shared::on_out_reset_color;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

mod access_point;
mod backend;

pub(super) enum MissionNodeAction {
    Link(Entity, MissionNodeLinkDir, Srgba),
}

pub(super) enum MissionNodeLayouts {
    MissionNodeA(access_point::AccessPoint),
    MissionNodeB(backend::Backend),
}

impl MissionNodeLayouts {
    pub(super) fn build_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str, kind: MissionNodeKind) -> Self {
        commands.entity(layout.entity(name)).insert(MissionNodeDisplay::new(kind));
        match kind {
            MissionNodeKind::AccessPoint => MissionNodeLayouts::MissionNodeA(access_point::AccessPoint::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Backend => MissionNodeLayouts::MissionNodeB(backend::Backend::build_layout(commands, layout, name, kind)),
            MissionNodeKind::Control => unimplemented!(),
            MissionNodeKind::Database => unimplemented!(),
            MissionNodeKind::Engine => unimplemented!(),
            MissionNodeKind::Frontend => unimplemented!(),
            MissionNodeKind::Gateway => unimplemented!(),
            MissionNodeKind::Hardware => unimplemented!(),
        }
    }
}

pub(crate) struct BaseNode {
    link: [Entity; 4],
    content: [Entity; 4],
}

trait NodeLinkEntityCommandsExt {
    fn observe_link_button(self) -> Self;
}

#[derive(Component)]
pub(crate) struct NodeLocalObserver;

// local copy of observe to decorate observers with NodeLocalObserver to easily dispose before reactivation
fn local_observe<E: Event, B: Bundle, M>(observer: impl IntoObserverSystem<E, B, M>) -> impl EntityCommand {
    move |entity: Entity, world: &mut World| {
        if let Ok(mut world_entity) = world.get_entity_mut(entity) {
            world_entity.world_scope(|w| {
                w.spawn(Observer::new(observer).with_entity(entity)).insert(NodeLocalObserver);
            });
        }
    }
}

impl NodeLinkEntityCommandsExt for &mut EntityCommands<'_> {
    fn observe_link_button(self) -> Self {
        self //
            .queue(local_observe(BaseNode::on_click_link))
            .queue(local_observe(BaseNode::on_over_link))
            .queue(local_observe(on_out_reset_color))
    }
}

impl BaseNode {
    pub(crate) fn build_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str) -> Self {
        const LINKS: &[(&str, MissionNodeLinkDir); 4] = &[
            //
            ("link_n", MissionNodeLinkDir::North),
            ("link_e", MissionNodeLinkDir::East),
            ("link_w", MissionNodeLinkDir::West),
            ("link_s", MissionNodeLinkDir::South),
        ];
        let link = LINKS.map(|(link, dir)| commands.entity(layout.entity(&format!("{}/{}", name, link))).insert((MissionNodeLinkButton::new(dir), PickingBehavior::default())).id());

        const CONTENT: &[&str; 4] = &["content1", "content2", "content3", "content4"];
        let content = CONTENT.map(|content| commands.entity(layout.entity(&format!("{}/{}", name, content))).insert((MissionNodeContentButton, PickingBehavior::default())).id());

        Self {
            link,
            content,
        }
    }

    pub(crate) fn activate(&self, commands: &mut Commands, node: &GameMissionNodePlayerView) {
        const DIRS: &[MissionNodeLinkDir; 4] = &[MissionNodeLinkDir::North, MissionNodeLinkDir::East, MissionNodeLinkDir::West, MissionNodeLinkDir::South];
        for (idx, dir) in DIRS.iter().enumerate() {
            commands.entity(self.link[idx]).insert(Self::node_link_visible(&node.links, *dir)).observe_link_button();
        }
        for (idx, e) in self.content.iter().enumerate() {
            commands.entity(*e).insert(Self::node_content_visible(&node.content, idx));
        }
    }

    fn node_link_visible(links: &[MissionNodeLink], dir: MissionNodeLinkDir) -> Visibility {
        if links.iter().any(|link| link.direction == dir) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }

    fn node_content_visible(content: &[MissionNodeContent], idx: usize) -> Visibility {
        if content.len() > idx {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }

    pub(crate) fn deselect(
        //
        commands: &mut Commands,
        context: &GameplayContext,
    ) {
        match &context.node_action {
            Some(MissionNodeAction::Link(entity, _, color)) => {
                commands.entity(*entity).trigger(SetColorEvent::new(*entity, *color)).insert(UiFxTrackedColor::from(*color));
            }
            None => {}
        }
    }

    fn on_click_link(
        //
        event: Trigger<Pointer<Click>>,
        mut commands: Commands,
        button_q: Query<(&MissionNodeLinkButton, &UiFxTrackedColor)>,
        mut context: ResMut<GameplayContext>,
    ) {
        if let Ok((button, new_color)) = button_q.get(event.target) {
            let (new_action, old_color) = match context.node_action {
                Some(MissionNodeAction::Link(_, current_dir, old_color)) if current_dir == button.dir => (None, Some(old_color)),
                _ => {
                    Self::deselect(&mut commands, &context);
                    (Some(MissionNodeAction::Link(event.target, button.dir, new_color.color)), None)
                }
            };

            context.node_action = new_action;

            let color = if context.node_action.is_some() {
                bevy::color::palettes::basic::GREEN
            } else {
                old_color.unwrap_or(bevy::color::palettes::basic::BLUE)
            };

            commands.entity(event.target).trigger(SetColorEvent::new(event.target, color)).insert(UiFxTrackedColor::from(color));
        }
    }

    fn on_over_link(
        //
        event: Trigger<Pointer<Over>>,
        mut commands: Commands,
    ) {
        commands.entity(event.target).trigger(SetColorEvent::new(event.target, bevy::color::palettes::basic::WHITE));
    }
}
