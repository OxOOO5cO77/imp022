use bevy::color::Color;
use shared_data::attribute::AttributeKind;
use shared_data::card::ErgType;

pub(crate) fn map_kind_to_color(kind: AttributeKind) -> Color {
    match kind {
        AttributeKind::Analyze => Color::srgb_u8(128, 0, 128),
        AttributeKind::Breach => Color::srgb_u8(0, 128, 0),
        AttributeKind::Compute => Color::srgb_u8(0, 0, 128),
        AttributeKind::Disrupt => Color::srgb_u8(128, 128, 0),
    }
}

pub(crate) fn map_kind_to_cost(kind: AttributeKind, cost: ErgType) -> String {
    let letter = match kind {
        AttributeKind::Analyze => 'A',
        AttributeKind::Breach => 'B',
        AttributeKind::Compute => 'C',
        AttributeKind::Disrupt => 'D',
    };
    format!("{cost}{letter}")
}
