use crate::manager::AtlasManager;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};
use bevy::prelude::{Commands, Out, Pointer, Query, Sprite, Trigger};
use hall::data::core::{AttributeKind, MissionNodeKind};
use hall::data::game::GameMissionNodePlayerView;

pub(crate) trait GameMissionNodePlayerViewExt {
    fn as_str(&self) -> &'static str;
    fn kind_value(&self) -> usize;
    fn make_id(&self) -> String;
}

impl GameMissionNodePlayerViewExt for GameMissionNodePlayerView {
    fn as_str(&self) -> &'static str {
        match self.kind {
            MissionNodeKind::AccessPoint => "Access Point",
            MissionNodeKind::Backend => "Backend",
            MissionNodeKind::Control => "Control",
            MissionNodeKind::Database => "Database",
            MissionNodeKind::Engine => "Engine",
            MissionNodeKind::Frontend => "Frontend",
            MissionNodeKind::Gateway => "Gateway",
            MissionNodeKind::Hardware => "Hardware",
        }
    }

    fn kind_value(&self) -> usize {
        match self.kind {
            MissionNodeKind::AccessPoint => 1,
            MissionNodeKind::Backend => 2,
            MissionNodeKind::Control => 3,
            MissionNodeKind::Database => 4,
            MissionNodeKind::Engine => 5,
            MissionNodeKind::Frontend => 6,
            MissionNodeKind::Gateway => 7,
            MissionNodeKind::Hardware => 8,
        }
    }

    fn make_id(&self) -> String {
        const ENCODE_MAP: [char; 32] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'M', 'N', 'P', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];

        let mut output = String::new();
        output.push(ENCODE_MAP[self.kind_value() & 0x1F]);
        output.push(ENCODE_MAP[(self.id & 0x1F) as usize]);
        let mut remain = self.remote;
        while remain > 0 {
            output.push(ENCODE_MAP[(remain & 0x1F) as usize]);
            remain >>= 5;
        }
        format!("{:0<15}", output).chars().collect::<Vec<char>>().chunks(3).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().join(":")
    }
}

pub(crate) fn on_out_generic(
    //
    event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    color_q: Query<&UiFxTrackedColor>,
) {
    if let Ok(source_color) = color_q.get(event.target) {
        commands.entity(event.target).trigger(SetColorEvent::new(event.target, source_color.color));
    }
}

pub(crate) enum KindIconSize {
    Small,
    //    Medium,
    Large,
}

pub(crate) fn replace_kind_icon(sprite: &mut Sprite, kind: AttributeKind, kind_icon_size: KindIconSize, am: &AtlasManager) {
    let texture_letter = match kind {
        AttributeKind::Analyze => 'A',
        AttributeKind::Breach => 'B',
        AttributeKind::Compute => 'C',
        AttributeKind::Disrupt => 'D',
    };

    let texture_name = match kind_icon_size {
        KindIconSize::Small => format!("{}016", texture_letter),
        //        KindIconSize::Medium => format!("{}048", texture_letter),
        KindIconSize::Large => format!("{}064", texture_letter),
    };

    if let Some((atlas, image)) = am.get_atlas_texture("common", &texture_name) {
        sprite.image = image;
        sprite.texture_atlas = Some(atlas);
    }
}
