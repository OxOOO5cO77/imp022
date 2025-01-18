use bevy::prelude::{Click, Commands, Entity, EntityCommands, Hwba, PickingBehavior, Pointer, Query, ResMut, Text2d, Trigger, Visibility};

use hall::core::{ActorIdType, MissionNodeIntent, MissionNodeKind, MissionNodeLinkDir, MissionNodeLinkState};
use hall::view::{GameMissionPlayerView, MAX_CONTENT_COUNT, MAX_LINK_COUNT, MAX_LINK_DAMAGE, MAX_USER_COUNT};

use crate::manager::{ScreenLayout, WarehouseManager};
use crate::screen::gameplay_main::components::{MissionNodeButton, MissionNodeContentButton};
use crate::screen::gameplay_main::nodes::shared;
use crate::screen::gameplay_main::resources::GameplayContext;
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

struct BaseNodeUser {
    container: Entity,
    bg: Entity,
    text: Entity,
}

impl BaseNodeUser {
    fn new(layout: &ScreenLayout, name: &str, user_name: &str) -> Self {
        let container = layout.entity(&format!("{name}/{user_name}"));
        let bg = layout.entity(&format!("{name}/{user_name}/bg"));
        let text = layout.entity(&format!("{name}/{user_name}/text"));
        Self {
            container,
            bg,
            text,
        }
    }
}

pub(crate) struct BaseNode {
    links: [BaseNodeLink; MAX_LINK_COUNT],
    content: [Entity; MAX_CONTENT_COUNT],
    users: [BaseNodeUser; MAX_USER_COUNT],
}

trait NodeLinkEntityCommandsExt {
    fn observe_link_button(self) -> Self;
}

impl NodeLinkEntityCommandsExt for &mut EntityCommands<'_> {
    fn observe_link_button(self) -> Self {
        self //
            .queue(shared::local_observe(BaseNode::on_click_link))
            .queue(shared::local_observe(shared::on_over_node_action))
            .queue(shared::local_observe(on_out_reset_color))
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
            commands.entity(layout.entity(&format!("{name}/{link}/frame"))).insert((MissionNodeButton::new(*dir), PickingBehavior::default()));
        }
        let links = LINKS.map(|(link, _)| BaseNodeLink::new(layout, name, link));

        const USERS: &[&str; MAX_USER_COUNT] = &["user0", "user1", "user2", "user3", "user4", "user5", "user6", "user7"];
        let users = USERS.map(|user| BaseNodeUser::new(layout, name, user));

        const CONTENT: &[&str; MAX_CONTENT_COUNT] = &["content1", "content2", "content3", "content4"];
        let content = CONTENT.map(|content| commands.entity(layout.entity(&format!("{name}/{content}"))).insert((MissionNodeContentButton, PickingBehavior::default())).id());

        Self {
            links,
            content,
            users,
        }
    }

    pub(crate) fn activate(&self, commands: &mut Commands, mission: &GameMissionPlayerView, text_q: &mut Query<&mut Text2d>, wm: &mut WarehouseManager) {
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

        for (idx, user) in self.users.iter().enumerate() {
            let visible = idx < current_node.users.len();
            if visible {
                let hue = pick_hue(current_node.users[idx]);
                let color = Hwba::hwb(hue, 0.25, 0.25);
                commands.entity(user.bg).trigger(SetColorEvent::new(user.bg, color.into()));
                if let Ok(mut text_letter) = text_q.get_mut(user.text) {
                    if let Ok(response) = wm.fetch_player(current_node.users[idx]) {
                        if let Some(bio) = response.player_bio.as_ref() {
                            *text_letter = bio.name.chars().next().unwrap_or('?').to_string().into();
                        }
                    }
                }
            }
            commands.entity(user.container).insert(Self::is_visible(visible));
        }
    }

    fn is_visible(locked: bool) -> Visibility {
        if locked {
            Visibility::Visible
        } else {
            Visibility::Hidden
        }
    }

    fn on_click_link(
        //
        event: Trigger<Pointer<Click>>,
        mut commands: Commands,
        button_q: Query<(&MissionNodeButton<MissionNodeLinkDir>, &UiFxTrackedColor)>,
        mut context: ResMut<GameplayContext>,
    ) {
        shared::click_common(&mut commands, &mut context, event.target, button_q.get(event.target), MissionNodeIntent::Link);
    }
}

fn pick_hue(id: ActorIdType) -> f32 {
    let mut hue = 0;
    hue += id & 0xFFFF;
    hue += (id >> 16) & 0xFFFF;
    hue += (id >> 32) & 0xFFFF;
    hue += (id >> 48) & 0xFFFF;
    (hue as f32 / 0xFFFF as f32) * 360.0
}
