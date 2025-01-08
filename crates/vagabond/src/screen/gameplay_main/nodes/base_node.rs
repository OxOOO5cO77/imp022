use bevy::prelude::{Click, Commands, Entity, EntityCommands, Over, PickingBehavior, Pointer, Query, Res, ResMut, Trigger, Visibility};

use hall::data::core::{MissionNodeContent, MissionNodeLink, MissionNodeLinkDir};
use hall::data::game::GameMissionNodePlayerView;

use crate::manager::ScreenLayout;
use crate::screen::gameplay_main::components::{MissionNodeContentButton, MissionNodeLinkButton};
use crate::screen::gameplay_main::nodes::{local_observe, MissionNodeAction};
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::gameplay_main::VagabondGamePhase;
use crate::screen::shared::on_out_reset_color;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

pub(crate) struct BaseNode {
    link: [Entity; 4],
    content: [Entity; 4],
}

trait NodeLinkEntityCommandsExt {
    fn observe_link_button(self) -> Self;
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
        for (link, dir) in LINKS {
            commands.entity(layout.entity(&format!("{name}/{link}/frame"))).insert((MissionNodeLinkButton::new(*dir), PickingBehavior::default()));
        }

        let link = LINKS.map(|(link, _)| layout.entity(&format!("{name}/{link}")));

        const CONTENT: &[&str; 4] = &["content1", "content2", "content3", "content4"];
        let content = CONTENT.map(|content| commands.entity(layout.entity(&format!("{name}/{content}"))).insert((MissionNodeContentButton, PickingBehavior::default())).id());

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
            MissionNodeAction::Link(entity, _, color) => {
                commands.entity(*entity).trigger(SetColorEvent::new(*entity, *color)).insert(UiFxTrackedColor::from(*color));
            }
            MissionNodeAction::None => {}
        }
    }

    fn on_click_link(
        //
        event: Trigger<Pointer<Click>>,
        mut commands: Commands,
        button_q: Query<(&MissionNodeLinkButton, &UiFxTrackedColor)>,
        mut context: ResMut<GameplayContext>,
    ) {
        if context.phase != VagabondGamePhase::Start {
            return;
        }

        if let Ok((button, new_color)) = button_q.get(event.target) {
            let (new_action, old_color) = match context.node_action {
                MissionNodeAction::Link(_, current_dir, old_color) if current_dir == button.dir => (MissionNodeAction::None, Some(old_color)),
                _ => {
                    Self::deselect(&mut commands, &context);
                    (MissionNodeAction::Link(event.target, button.dir, new_color.color), None)
                }
            };

            context.node_action = new_action;

            let color = match context.node_action {
                MissionNodeAction::None => old_color.unwrap_or(bevy::color::palettes::basic::BLUE),
                _ => bevy::color::palettes::basic::GREEN,
            };

            commands.entity(event.target).trigger(SetColorEvent::new(event.target, color)).insert(UiFxTrackedColor::from(color));
        }
    }

    fn on_over_link(
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
}
