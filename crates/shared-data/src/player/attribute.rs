use serde::{Deserialize, Serialize};

pub type ValueType = u8;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Attribute {
    pub accuracy: ValueType,
    pub boost: ValueType,
    pub celerity: ValueType,
    pub duration: ValueType,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
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

    pub fn from_array(array: [ValueType; 4]) -> Self {
        Self {
            accuracy: array[0],
            boost: array[1],
            celerity: array[2],
            duration: array[3],
        }
    }
}

impl Attributes {
    pub fn to_array(&self) -> [[ValueType; 4]; 4] {
        [self.analyze.to_array(), self.breach.to_array(), self.compute.to_array(), self.disrupt.to_array()]
    }

    pub fn from_array(array: [[ValueType; 4]; 4]) -> Self {
        Self {
            analyze: Attribute::from_array(array[0]),
            breach: Attribute::from_array(array[1]),
            compute: Attribute::from_array(array[2]),
            disrupt: Attribute::from_array(array[3]),
        }
    }
}
