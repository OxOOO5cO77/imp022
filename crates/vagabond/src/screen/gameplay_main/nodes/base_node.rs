use bevy::prelude::{Click, Commands, Entity, EntityCommands, Over, PickingBehavior, Pointer, Query, Res, ResMut, Text2d, Trigger, Visibility};

use hall::core::{MissionNodeKind, MissionNodeLinkDir, MissionNodeLinkState};
use hall::view::{GameMissionPlayerView, MAX_CONTENT_COUNT, MAX_LINK_COUNT, MAX_LINK_DAMAGE};

use crate::manager::ScreenLayout;
use crate::screen::gameplay_main::components::{MissionNodeContentButton, MissionNodeLinkButton};
use crate::screen::gameplay_main::nodes::{local_observe, MissionNodeAction};
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::gameplay_main::VagabondGamePhase;
use crate::screen::shared::{on_out_reset_color, GameMissionNodePlayerViewExt, MissionNodeKindExt};
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

struct BaseNodeLink {
    container: Entity,
    title: Entity,
    remote_id: Entity,
    lock: Entity,
    unlock: Entity,
    damage: Entity,
}

impl BaseNodeLink {
    fn new(layout: &ScreenLayout, name: &str, link_name: &str) -> Self {
        let container = layout.entity(&format!("{name}/{link_name}"));
        let title = layout.entity(&format!("{name}/{link_name}/title"));
        let remote_id = layout.entity(&format!("{name}/{link_name}/remote_id"));
        let lock = layout.entity(&format!("{name}/{link_name}/lock"));
        let unlock = layout.entity(&format!("{name}/{link_name}/unlock"));
        let damage = layout.entity(&format!("{name}/{link_name}/damage"));

        Self {
            container,
            title,
            remote_id,
            lock,
            unlock,
            damage,
        }
    }
}

pub(crate) struct BaseNode {
    links: [BaseNodeLink; MAX_LINK_COUNT],
    content: [Entity; MAX_CONTENT_COUNT],
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
        const LINKS: &[(&str, MissionNodeLinkDir); MAX_LINK_COUNT] = &[
            //
            ("link_n", MissionNodeLinkDir::North),
            ("link_e", MissionNodeLinkDir::East),
            ("link_w", MissionNodeLinkDir::West),
            ("link_s", MissionNodeLinkDir::South),
        ];
        for (link, dir) in LINKS {
            commands.entity(layout.entity(&format!("{name}/{link}/frame"))).insert((MissionNodeLinkButton::new(*dir), PickingBehavior::default()));
        }

        let links = LINKS.map(|(link, _)| BaseNodeLink::new(layout, name, link));

        const CONTENT: &[&str; MAX_CONTENT_COUNT] = &["content1", "content2", "content3", "content4"];
        let content = CONTENT.map(|content| commands.entity(layout.entity(&format!("{name}/{content}"))).insert((MissionNodeContentButton, PickingBehavior::default())).id());

        Self {
            links,
            content,
        }
    }

    pub(crate) fn activate(&self, commands: &mut Commands, mission: &GameMissionPlayerView, text_q: &mut Query<&mut Text2d>) {
        let current_node = mission.current();

        const DIRS: &[MissionNodeLinkDir; MAX_LINK_COUNT] = &[MissionNodeLinkDir::North, MissionNodeLinkDir::East, MissionNodeLinkDir::West, MissionNodeLinkDir::South];
        for (idx, dir) in DIRS.iter().enumerate() {
            let visible = current_node.links.iter().any(|link| link.direction == *dir);
            commands.entity(self.links[idx].container).insert(Self::is_visible(visible)).observe_link_button();
        }

        for (idx, link) in self.links.iter().enumerate() {
            let link_dir = current_node.links.iter().find(|l| l.direction == DIRS[idx]);
            let node_target = link_dir.map(|l| l.target).and_then(|target| mission.get_node(target));
            let kind = node_target.map_or(MissionNodeKind::Unknown, |n| n.kind);
            let remote_id = node_target.map_or("???:???:????:???:???".to_string(), |n| n.make_id());
            let locked = link_dir.is_some_and(|l| l.state == MissionNodeLinkState::Closed);
            let unlocked = link_dir.is_some_and(|l| l.state == MissionNodeLinkState::Open);
            let damage = MAX_LINK_DAMAGE.saturating_sub(link_dir.map_or(0, |l| l.damage));
            if let Ok([mut text_title, mut text_remote_id, mut text_damage]) = text_q.get_many_mut([link.title, link.remote_id, link.damage]) {
                *text_title = kind.as_str().into();
                *text_remote_id = remote_id.into();
                *text_damage = damage.to_string().into();
            }
            commands.entity(link.lock).insert(Self::is_visible(locked));
            commands.entity(link.unlock).insert(Self::is_visible(unlocked));
            commands.entity(link.damage).insert(Self::is_visible(locked));
        }

        for (idx, e) in self.content.iter().enumerate() {
            let visible = idx < current_node.content.len();
            commands.entity(*e).insert(Self::is_visible(visible));
        }
    }

    fn is_visible(locked: bool) -> Visibility {
        if locked {
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
