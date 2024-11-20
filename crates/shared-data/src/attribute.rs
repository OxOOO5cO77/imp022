use serde::{Deserialize, Serialize};

pub type AttributeValueType = u8;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AttributeKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}


#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct AttributeValues {
    pub accuracy: AttributeValueType,
    pub boost: AttributeValueType,
    pub celerity: AttributeValueType,
    pub duration: AttributeValueType,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct Attributes {
    pub analyze: AttributeValues,
    pub breach: AttributeValues,
    pub compute: AttributeValues,
    pub disrupt: AttributeValues,
}

impl AttributeValues {
    pub fn to_array(&self) -> [AttributeValueType; 4] {
        [self.accuracy, self.boost, self.celerity, self.duration]
    }

    pub fn from_array(array: [AttributeValueType; 4]) -> Self {
        Self {
            accuracy: array[0],
            boost: array[1],
            celerity: array[2],
            duration: array[3],
        }
    }
}

impl Attributes {
    pub fn to_arrays(&self) -> [[AttributeValueType; 4]; 4] {
        [self.analyze.to_array(), self.breach.to_array(), self.compute.to_array(), self.disrupt.to_array()]
    }

    pub fn from_arrays(array: [[AttributeValueType; 4]; 4]) -> Self {
        Self {
            analyze: AttributeValues::from_array(array[0]),
            breach: AttributeValues::from_array(array[1]),
            compute: AttributeValues::from_array(array[2]),
            disrupt: AttributeValues::from_array(array[3]),
        }
    }

    pub fn get(&self, kind: AttributeKind) -> [AttributeValueType; 4] {
        match kind {
            AttributeKind::Analyze => self.analyze.to_array(),
            AttributeKind::Breach => self.breach.to_array(),
            AttributeKind::Compute => self.compute.to_array(),
            AttributeKind::Disrupt => self.disrupt.to_array(),
        }
    }
}
