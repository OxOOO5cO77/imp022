// ASk - avoid detection
// ASp - find connections faster
// ASt - access connections faster
// ASu - keep connections open longer
//
// BSk - combat hit rate
// BSp - combat action modifier
// BSt - combat damage
// BSu - prevent retreat
//
// CSk - program complexity
// CSp - program optimization
// CSt - program power
// CSu - program duration
//
// DSk -
// DSp -
// DSt -
// DSu -

use crate::data::player_builder::PlayerPart;

pub(crate) struct Attribute {
    pub(crate) ability: u8,
    pub(crate) beat: u8,
    pub(crate) cadence: u8,
    pub(crate) duration: u8,
}

impl Attribute {
    pub(crate) fn from_part(part: &PlayerPart) -> Self {
        Self {
            ability: part.values[0],
            cadence: part.values[1],
            beat: part.values[2],
            duration: part.values[3],
        }
    }
}

pub(crate) struct Attributes {
    pub(crate) access: Attribute,
    pub(crate) breach: Attribute,
    pub(crate) compute: Attribute,
    pub(crate) disrupt: Attribute,
}

impl Attributes {
    pub(crate) fn from_parts(access: &PlayerPart, breach: &PlayerPart, compute: &PlayerPart, disrupt: &PlayerPart) -> Self {
        Self {
            access: Attribute::from_part(access),
            breach: Attribute::from_part(breach),
            compute: Attribute::from_part(compute),
            disrupt: Attribute::from_part(disrupt),
        }
    }

}
