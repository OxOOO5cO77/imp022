use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub accuracy: u8,
    pub boost: u8,
    pub celerity: u8,
    pub duration: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub analyze: Attribute,
    pub breach: Attribute,
    pub compute: Attribute,
    pub disrupt: Attribute,
}

impl Attribute {
    pub fn to_array(&self) -> [u8; 4] {
        [self.accuracy, self.boost, self.celerity, self.duration]
    }

    pub fn from_values(part: &Option<[u8; 4]>) -> Option<Self> {
        let part = part.as_ref()?;
        Some(
            Self {
                accuracy: part[0],
                boost: part[1],
                celerity: part[2],
                duration: part[3],
            }
        )
    }
}


impl Attributes {
    pub fn from_values(access: &Option<[u8; 4]>, breach: &Option<[u8; 4]>, compute: &Option<[u8; 4]>, disrupt: &Option<[u8; 4]>) -> Option<Self> {
        Some(
            Self {
                analyze: Attribute::from_values(access)?,
                breach: Attribute::from_values(breach)?,
                compute: Attribute::from_values(compute)?,
                disrupt: Attribute::from_values(disrupt)?,
            }
        )
    }
}
