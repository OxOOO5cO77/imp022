use bevy::prelude::*;

use hall_lib::core::{ActorIdType, ActorIndexType, AuthLevel, MissionNodeIntent, MissionNodeKind, MissionNodeLinkDir, PickedCardTarget};
use hall_lib::view::{GameMissionPlayerView, MAX_ACTOR_COUNT, MAX_CONTENT_COUNT, MAX_LINK_COUNT};

use crate::manager::{ScreenLayout, WarehouseManager};
use crate::screen::gameplay_main::components::{CardDropTarget, MissionNodeButton, MissionNodeContentButton};
use crate::screen::gameplay_main::nodes::shared;
use crate::screen::gameplay_main::on_card_drop;
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::shared::{on_out_reset_color, GameMissionNodePlayerViewExt, MissionNodeKindExt};
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor, UiFxTrackedSize};

struct BaseNodeLink {
    container: Entity,
    title: Entity,
    remote_id: Entity,
    lock: Entity,
    unlock: Entity,
}

impl BaseNodeLink {
    fn new(layout: &ScreenLayout, name: &str, link_name: &str) -> Self {
        let container = layout.entity(&format!("{name}/{link_name}"));
        let title = layout.entity(&format!("{name}/{link_name}/title"));
        let remote_id = layout.entity(&format!("{name}/{link_name}/remote_id"));
        let lock = layout.entity(&format!("{name}/{link_name}/lock"));
        let unlock = layout.entity(&format!("{name}/{link_name}/unlock"));

        Self {
            container,
            title,
            remote_id,
            lock,
            unlock,
        }
    }
}

struct BaseNodeActor {
    container: Entity,
    bg: Entity,
    text: Entity,
}

impl BaseNodeActor {
    fn new(layout: &ScreenLayout, name: &str, actor_slot: &str) -> Self {
        let container = layout.entity(&format!("{name}/{actor_slot}"));
        let bg = layout.entity(&format!("{name}/{actor_slot}/bg"));
        let text = layout.entity(&format!("{name}/{actor_slot}/text"));
        Self {
            container,
            bg,
            text,
        }
    }
}

#[derive(Component)]
struct ActorInfoHolder {
    id: ActorIdType,
    auth: AuthLevel,
}

impl ActorInfoHolder {
    fn new(id: ActorIdType, auth: AuthLevel) -> Self {
        Self {
            id,
            auth,
        }
    }
}

pub(crate) struct BaseNode {
    links: [BaseNodeLink; MAX_LINK_COUNT],
    content: [Entity; MAX_CONTENT_COUNT],
    actors: [BaseNodeActor; MAX_ACTOR_COUNT],
}

trait NodeLinkEntityCommandsExt {
    fn observe_link_button(self) -> Self;
    fn observe_actor(self, actor_id: ActorIdType, auth: AuthLevel, index: usize) -> Self;
}

impl NodeLinkEntityCommandsExt for &mut EntityCommands<'_> {
    fn observe_link_button(self) -> Self {
        self //
            .queue(shared::local_observe(BaseNode::on_click_link))
            .queue(shared::local_observe(shared::on_over_node_action))
            .queue(shared::local_observe(on_out_reset_color))
    }

    fn observe_actor(self, actor_id: ActorIdType, auth: AuthLevel, index: usize) -> Self {
        self //
            .queue(shared::local_observe(on_over_actor_action))
            .queue(shared::local_observe(on_out_actor_action))
            .queue(shared::local_observe(on_card_drop))
            .insert(ActorInfoHolder::new(actor_id, auth))
            .insert(CardDropTarget::new(PickedCardTarget::Actor(index as ActorIndexType)))
    }
}

impl BaseNode {
    pub(crate) fn build_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str) -> Self {
        const LINKS: &[(&str, MissionNodeLinkDir); MAX_LINK_COUNT] = &[
            //
            ("link_n", MissionNodeLinkDir::North),
            ("link_e", MissionNodeLinkDir::East),
            ("link_s", MissionNodeLinkDir::South),
            ("link_w", MissionNodeLinkDir::West),
        ];
        for (link, dir) in LINKS {
            commands.entity(layout.entity(&format!("{name}/{link}/frame"))).insert((MissionNodeButton::new(*dir), PickingBehavior::default()));
        }
        let links = LINKS.map(|(link, _)| BaseNodeLink::new(layout, name, link));

        const ACTORS: &[&str; MAX_ACTOR_COUNT] = &["actor0", "actor1", "actor2", "actor3", "actor4", "actor5", "actor6", "actor7"];
        let actors = ACTORS.map(|actor| BaseNodeActor::new(layout, name, actor));
        for actor in &actors {
            commands.entity(actor.container).insert(PickingBehavior::default());
        }

        const CONTENT: &[&str; MAX_CONTENT_COUNT] = &["content1", "content2", "content3", "content4"];
        let content = CONTENT.map(|content| commands.entity(layout.entity(&format!("{name}/{content}"))).insert((MissionNodeContentButton, PickingBehavior::default())).id());

        let tooltip_entity = commands.entity(layout.entity(&format!("{name}/tooltip"))).insert(Visibility::Hidden).observe(on_update_actor_tooltip).id();
        let tooltip = ActorTooltip {
            container: tooltip_entity,
            name: layout.entity(&format!("{name}/tooltip/name")),
            location: layout.entity(&format!("{name}/tooltip/location")),
            auth: layout.entity(&format!("{name}/tooltip/auth")),
        };
        commands.insert_resource(tooltip);

        Self {
            links,
            content,
            actors,
        }
    }

    pub(crate) fn activate(&self, commands: &mut Commands, mission: &GameMissionPlayerView, text_q: &mut Query<&mut Text2d>, wm: &mut WarehouseManager) {
        let current_node = mission.current();

        const DIRS: &[MissionNodeLinkDir; MAX_LINK_COUNT] = &[MissionNodeLinkDir::North, MissionNodeLinkDir::East, MissionNodeLinkDir::South, MissionNodeLinkDir::West];
        for (idx, dir) in DIRS.iter().enumerate() {
            let visible = current_node.links.iter().any(|link| link.direction == *dir);
            commands.entity(self.links[idx].container).insert(Self::is_visible(visible)).observe_link_button();
        }

        for (idx, link) in self.links.iter().enumerate() {
            let link_dir = current_node.links.iter().find(|l| l.direction == DIRS[idx]);
            let node_target = link_dir.map(|l| l.target).and_then(|target| mission.get_node(target));
            let kind = node_target.map_or(MissionNodeKind::Unknown, |n| n.kind);
            let remote_id = node_target.map_or("???:???:????:???:???".to_string(), |n| n.make_id());
            if let Ok([mut text_title, mut text_remote_id]) = text_q.get_many_mut([link.title, link.remote_id]) {
                *text_title = kind.as_str().into();
                *text_remote_id = remote_id.into();
            }
            commands.entity(link.lock).insert(Self::is_visible(link_dir.is_some_and(|l| l.locked)));
            commands.entity(link.unlock).insert(Self::is_visible(link_dir.is_some_and(|l| !l.locked)));
        }

        for (idx, e) in self.content.iter().enumerate() {
            let visible = idx < current_node.content.len();
            commands.entity(*e).insert(Self::is_visible(visible));
        }

        for (idx, actor_ui) in self.actors.iter().enumerate() {
            let visible = idx < current_node.actors.len();
            if visible {
                let actor = &current_node.actors[idx];
                commands.entity(actor_ui.container).observe_actor(actor.id, actor.auth_level, idx);
                let hue = pick_hue(actor.id);
                let color = Hwba::hwb(hue, 0.25, 0.25);
                commands.entity(actor_ui.bg).trigger(SetColorEvent::new(actor_ui.bg, color.into()));
                if let Ok(mut text_letter) = text_q.get_mut(actor_ui.text) {
                    if let Ok(response) = wm.fetch_player(actor.id) {
                        if let Some(bio) = response.player_bio.as_ref() {
                            *text_letter = bio.name.chars().next().unwrap_or('?').to_string().into();
                        }
                    }
                }
            }
            commands.entity(actor_ui.container).insert(Self::is_visible(visible));
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

fn on_over_actor_action(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    holder_q: Query<&ActorInfoHolder>,
    tooltip: Res<ActorTooltip>,
) {
    if let Ok(holder) = holder_q.get(event.target) {
        commands.trigger_targets(UpdateActorTooltipEvent::new(event.pointer_location.position, holder.id, holder.auth), tooltip.container);
    }
}

fn on_out_actor_action(
    //
    _event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    tooltip: Res<ActorTooltip>,
) {
    commands.entity(tooltip.container).insert(Visibility::Hidden);
}

#[derive(Resource)]
pub(crate) struct ActorTooltip {
    pub(crate) container: Entity,
    pub(crate) name: Entity,
    pub(crate) location: Entity,
    pub(crate) auth: Entity,
}

#[derive(Event)]
pub(crate) struct UpdateActorTooltipEvent {
    position: Vec2,
    id: ActorIdType,
    auth: AuthLevel,
}

impl UpdateActorTooltipEvent {
    pub(crate) fn new(position: Vec2, id: ActorIdType, auth: AuthLevel) -> Self {
        Self {
            position,
            id,
            auth,
        }
    }
}

fn on_update_actor_tooltip(
    // bevy system
    event: Trigger<UpdateActorTooltipEvent>,
    mut commands: Commands,
    mut tooltip_q: Query<(&mut Transform, &GlobalTransform, &UiFxTrackedSize)>,
    mut text_q: Query<&mut Text2d>,
    window_q: Query<&Window>,
    tooltip: Res<ActorTooltip>,
    mut wm: ResMut<WarehouseManager>,
) {
    let target = event.entity();
    let window = window_q.single();

    if let Ok((mut transform, global_transform, tooltip_size)) = tooltip_q.get_mut(target) {
        if let Some(bio) = wm.fetch_player(event.id).ok().and_then(|bio| bio.player_bio.as_ref()) {
            if let Ok([mut name, mut location, mut auth]) = text_q.get_many_mut([tooltip.name, tooltip.location, tooltip.auth]) {
                *name = bio.name.as_str().into();
                *location = bio.birthplace().into();
                *auth = event.auth.as_str().into();
            }

            let offset = global_transform.translation().xy() - transform.translation.xy();

            let x = event.position.x.clamp(0.0, window.width() - tooltip_size.x);
            let y = event.position.y.clamp(0.0, window.height() - tooltip_size.y);
            transform.translation = (Vec2::new(x, -y) - offset).extend(transform.translation.z);

            commands.entity(tooltip.container).insert(Visibility::Visible);
        }
    }
}
