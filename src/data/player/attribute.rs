use serde::Deserialize;

#[derive(Default, Deserialize)]
pub(crate) struct Attribute {
    pub(crate) accuracy: u8,
    pub(crate) boost: u8,
    pub(crate) celerity: u8,
    pub(crate) duration: u8,
}

impl Attribute {
    pub(crate) fn to_array(&self) -> [u8; 4] {
        [self.accuracy, self.boost, self.celerity, self.duration]
    }
}

#[derive(Deserialize)]
pub(crate) struct Attributes {
    pub(crate) analyze: Attribute,
    pub(crate) breach: Attribute,
    pub(crate) compute: Attribute,
    pub(crate) disrupt: Attribute,
}
