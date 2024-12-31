use crate::manager::ScreenLayout;
use crate::screen::gameplay_main::components::{MissionNodeContentButton, MissionNodeDisplay, MissionNodeLinkButton};
use crate::screen::shared::on_out_reset_color;
use bevy::prelude::{info, Click, Commands, Entity, EntityCommands, Over, PickingBehavior, Pointer, Query, Trigger, Visibility};
use hall::data::core::{MissionNodeContent, MissionNodeKind, MissionNodeLink, MissionNodeLinkDir};
use hall::data::game::GameMissionNodePlayerView;

mod access_point;
mod backend;

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

impl NodeLinkEntityCommandsExt for &mut EntityCommands<'_> {
    fn observe_link_button(self) -> Self {
        self //
            .observe(BaseNode::on_click_link)
            .observe(BaseNode::on_over_link)
            .observe(on_out_reset_color)
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
            commands.entity(*e).insert(Self::node_content_visible(&node.content, idx)).observe_link_button();
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

    fn on_click_link(
        //
        event: Trigger<Pointer<Click>>,
        button_q: Query<&MissionNodeLinkButton>,
    ) {
        if let Ok(button) = button_q.get(event.target) {
            info!("button: {:?}", button.dir);
        }
    }

    fn on_over_link(
        //
        _event: Trigger<Pointer<Over>>,
    ) {
    }
}
