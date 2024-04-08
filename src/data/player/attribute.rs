// A - This node info
// A - That node info
// A - program info
// A - item info
//
// B - Open node link
// B - Open root link
// B - Traverse node
// B - open item
//
// C - run local
// C - run remote
// C - add node
// C -
//
// D - crash program
// D - offline node
// D - disconnect player
// D - delete item

use crate::data::player::player_builder::PlayerPart;

pub(crate) struct Attribute {
    pub(crate) ability: u8,
    pub(crate) beat: u8,
    pub(crate) cadence: u8,
    pub(crate) duration: u8,
}

impl Attribute {
    pub(crate) fn from_part(part: &PlayerPart) -> Self {
        Self { ability: part.values[0], cadence: part.values[1], beat: part.values[2], duration: part.values[3] }
    }
    
    pub(crate) fn to_array(&self) -> [u8; 4] {
        [self.ability, self.beat, self.cadence, self.duration]
    }
}

pub(crate) struct Attributes {
    pub(crate) analyze: Attribute,
    pub(crate) breach: Attribute,
    pub(crate) compute: Attribute,
    pub(crate) disrupt: Attribute,
}

impl Attributes {
    pub(crate) fn from_parts(access: &PlayerPart, breach: &PlayerPart, compute: &PlayerPart, disrupt: &PlayerPart) -> Self {
        Self { analyze: Attribute::from_part(access), breach: Attribute::from_part(breach), compute: Attribute::from_part(compute), disrupt: Attribute::from_part(disrupt) }
    }
}
