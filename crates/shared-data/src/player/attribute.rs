use serde::{Deserialize, Serialize};

pub type ValueType = u8;

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub accuracy: ValueType,
    pub boost: ValueType,
    pub celerity: ValueType,
    pub duration: ValueType,
}

#[derive(Serialize, Deserialize)]
pub struct Attributes {
    pub analyze: Attribute,
    pub breach: Attribute,
    pub compute: Attribute,
    pub disrupt: Attribute,
}

impl Attribute {
    pub fn to_array(&self) -> [ValueType; 4] {
        [self.accuracy, self.boost, self.celerity, self.duration]
    }

    pub fn from_values(part: &[ValueType; 4]) -> Self {
        Self {
            accuracy: part[0],
            boost: part[1],
            celerity: part[2],
            duration: part[3],
        }
    }
}


impl Attributes {
    pub fn from_values(access: &[ValueType; 4], breach: &[ValueType; 4], compute: &[ValueType; 4], disrupt: &[ValueType; 4]) -> Self {
        Self {
            analyze: Attribute::from_values(access),
            breach: Attribute::from_values(breach),
            compute: Attribute::from_values(compute),
            disrupt: Attribute::from_values(disrupt),
        }
    }
}
