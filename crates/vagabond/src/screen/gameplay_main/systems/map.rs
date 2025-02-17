use std::collections::HashMap;

use bevy::prelude::{Commands, Query, Res, Text2d, Trigger, Visibility};

use crate::screen::gameplay_main::events::ShowMapTrigger;
use crate::screen::gameplay_main::resources::{GameplayContext, GameplayMap};
use crate::screen::shared::MissionNodeKindExt;
use crate::system::ui_effects::SetColorEvent;

pub(super) fn on_show_map(
    // bevy system
    _event: Trigger<ShowMapTrigger>,
    mut commands: Commands,
    mut text_q: Query<&mut Text2d>,
    map: Res<GameplayMap>,
    context: Res<GameplayContext>,
) {
    commands.entity(map.main).insert(Visibility::Visible);
    let known_nodes = context.cached_mission.node_map.iter().map(|n| (n.id, n)).collect::<HashMap<_, _>>();
    for (id, map_node) in &map.node {
        let visible = if let Some(node) = known_nodes.get(id) {
            if let Ok([mut text_id, mut text_kind]) = text_q.get_many_mut([map_node.text_id, map_node.text_kind]) {
                *text_id = format!("{id:02}").into();
                *text_kind = node.kind.as_single_letter().into();
            }
            for link in &node.links {
                let pair = if *id > link.target {
                    (link.target, *id)
                } else {
                    (*id, link.target)
                };
                if let Some(entity) = map.links.get(&pair) {
                    commands.entity(*entity).insert(Visibility::Inherited);
                }
            }
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
        let color = if context.cached_mission.current_node == *id {
            bevy::color::palettes::basic::GREEN
        } else {
            bevy::color::palettes::basic::GRAY
        };
        commands.entity(map_node.entity).insert(visible);
        commands.entity(map_node.frame).trigger(SetColorEvent::new(map_node.frame, color));
    }
}
