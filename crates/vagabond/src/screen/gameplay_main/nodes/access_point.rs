use bevy::prelude::{Commands, Entity, Query, Text2d};

use hall::view::GameMissionPlayerView;

use crate::manager::{DataManager, ScreenLayout, WarehouseManager};

pub(crate) struct AccessPoint {
    institution: Entity,
    access_point: Entity,
    location: Entity,
}

impl AccessPoint {
    pub(super) fn build_layout(_commands: &mut Commands, layout: &ScreenLayout, name: &str) -> Self {
        let institution = layout.entity(&format!("{name}/institution"));
        let access_point = layout.entity(&format!("{name}/access_point"));
        let location = layout.entity(&format!("{name}/location"));
        Self {
            institution,
            access_point,
            location,
        }
    }

    pub(crate) fn activate(&self, _commands: &mut Commands, mission: &GameMissionPlayerView, text_q: &mut Query<&mut Text2d>, dm: &DataManager, wm: &mut WarehouseManager) -> Option<()> {
        let institution = dm.deduce_institution(mission.id);
        let access_point = format!("Access Point {:03}", mission.id & 0xFF);
        let location = match wm.fetch_location(mission.id) {
            Ok(response) => response.location.as_ref()?.location(),
            Err(_) => "<Unknown>".to_string(),
        };

        if let Ok([mut ins_text, mut ap_text, mut loc_text]) = text_q.get_many_mut([self.institution, self.access_point, self.location]) {
            *ins_text = institution?.title.as_str().into();
            *ap_text = access_point.into();
            *loc_text = location.into();
        }

        Some(())
    }
}
