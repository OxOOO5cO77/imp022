use bevy::prelude::{Click, Commands, Entity, EntityCommands, Pickable, Pointer, Query, ResMut, Text2d, Trigger};

use hall_lib::core::{AccessPointIntent, MissionNodeIntent};
use hall_lib::view::GameMissionPlayerView;

use crate::manager::{DataManager, ScreenLayout, WarehouseManager};
use crate::screen::gameplay_main::components::MissionNodeButton;
use crate::screen::gameplay_main::nodes::shared;
use crate::screen::gameplay_main::resources::GameplayContext;
use crate::screen::shared::on_out_reset_color;
use crate::system::ui_effects::UiFxTrackedColor;

pub(crate) struct AccessPoint {
    institution: Entity,
    access_point: Entity,
    location: Entity,
    auth_button: Entity,
    next_button: Entity,
    prev_button: Entity,
}

trait NodeLinkEntityCommandsExt {
    fn observe_access_point_button(self) -> Self;
}

impl NodeLinkEntityCommandsExt for &mut EntityCommands<'_> {
    fn observe_access_point_button(self) -> Self {
        self //
            .queue(shared::local_observe(AccessPoint::on_click_access_point_button))
            .queue(shared::local_observe(shared::on_over_node_action))
            .queue(shared::local_observe(on_out_reset_color))
    }
}

impl AccessPoint {
    pub(super) fn build_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str) -> Self {
        let institution = layout.entity(&format!("{name}/institution"));
        let access_point = layout.entity(&format!("{name}/access_point"));
        let location = layout.entity(&format!("{name}/location"));

        let auth_button = commands.entity(layout.entity(&format!("{name}/authorize_bg"))).insert((MissionNodeButton::new(AccessPointIntent::Authenticate), Pickable::default())).id();
        let next_button = commands.entity(layout.entity(&format!("{name}/next_bg"))).insert((MissionNodeButton::new(AccessPointIntent::TransferNext), Pickable::default())).id();
        let prev_button = commands.entity(layout.entity(&format!("{name}/prev_bg"))).insert((MissionNodeButton::new(AccessPointIntent::TransferPrev), Pickable::default())).id();

        Self {
            institution,
            access_point,
            location,
            auth_button,
            next_button,
            prev_button,
        }
    }

    pub(crate) fn activate(&self, commands: &mut Commands, mission: &GameMissionPlayerView, text_q: &mut Query<&mut Text2d>, dm: &DataManager, wm: &mut WarehouseManager) -> Option<()> {
        let current_node = mission.current();
        let institution = dm.convert_institution(mission.institution);
        let access_point = format!("Access Point {:03}", current_node.id);
        let location = match wm.fetch_location(mission.id) {
            Ok(response) => response.location.as_ref()?.location(),
            Err(_) => "<Unknown>".to_string(),
        };

        commands.entity(self.auth_button).observe_access_point_button();
        commands.entity(self.next_button).observe_access_point_button();
        commands.entity(self.prev_button).observe_access_point_button();

        if let Ok([mut ins_text, mut ap_text, mut loc_text]) = text_q.get_many_mut([self.institution, self.access_point, self.location]) {
            *ins_text = institution?.title.as_str().into();
            *ap_text = access_point.into();
            *loc_text = location.into();
        }

        Some(())
    }

    fn on_click_access_point_button(
        //
        event: Trigger<Pointer<Click>>,
        mut commands: Commands,
        button_q: Query<(&MissionNodeButton<AccessPointIntent>, &UiFxTrackedColor)>,
        mut context: ResMut<GameplayContext>,
    ) {
        shared::click_common(&mut commands, &mut context, event.target, button_q.get(event.target), MissionNodeIntent::AccessPoint);
    }
}
