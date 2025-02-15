use bevy::prelude::{Commands, Out, Pointer, Query, Trigger};

use hall_lib::core::{AttributeKind, MissionNodeKind};
use hall_lib::view::GameMissionNodePlayerView;

use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

pub(crate) trait MissionNodeKindExt {
    fn as_str(&self) -> &'static str;
}

impl MissionNodeKindExt for MissionNodeKind {
    fn as_str(&self) -> &'static str {
        match self {
            MissionNodeKind::Unknown => "???",
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
}

pub(crate) trait GameMissionNodePlayerViewExt {
    fn kind_value(&self) -> usize;
    fn make_id(&self) -> String;
}

impl GameMissionNodePlayerViewExt for GameMissionNodePlayerView {
    fn kind_value(&self) -> usize {
        let kind: u8 = self.kind.into();
        kind as usize
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

pub(crate) fn on_out_reset_color(
    //
    event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    color_q: Query<&UiFxTrackedColor>,
) {
    if let Ok(source_color) = color_q.get(event.target) {
        commands.entity(event.target).trigger(SetColorEvent::new(event.target, source_color.color));
    }
}

pub(crate) fn kind_icon(kind: AttributeKind) -> &'static str {
    match kind {
        AttributeKind::Analyze => "⏿",
        AttributeKind::Breach => "⎆",
        AttributeKind::Compute => "⌨",
        AttributeKind::Disrupt => "𐆅",
    }
}
