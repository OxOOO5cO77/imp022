use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};
use shared_net::{Bufferable, VSizedBuffer};

pub type AttributeValueType = u8;
type AttributeKindType = u8;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
pub enum AttributeKind {
    #[num_enum(default)]
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
        let [accuracy, boost, celerity, duration] = array;
        Self {
            accuracy,
            boost,
            celerity,
            duration,
        }
    }
}

impl Attributes {
    pub fn to_arrays(&self) -> [[AttributeValueType; 4]; 4] {
        [self.analyze.to_array(), self.breach.to_array(), self.compute.to_array(), self.disrupt.to_array()]
    }

    pub fn from_arrays(array: [[AttributeValueType; 4]; 4]) -> Self {
        let [analyze, breach, compute, disrupt] = array.map(AttributeValues::from_array);
        Self {
            analyze,
            breach,
            compute,
            disrupt,
        }
    }

    pub fn get(&self, kind: AttributeKind) -> [AttributeValueType; 4] {
        match kind {
            AttributeKind::Analyze => self.analyze,
            AttributeKind::Breach => self.breach,
            AttributeKind::Compute => self.compute,
            AttributeKind::Disrupt => self.disrupt,
        }
        .to_array()
    }
}

impl Bufferable for AttributeKind {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        let attribute_kind: AttributeKindType = (*self).into();
        attribute_kind.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let attribute_kind = AttributeKindType::pull_from(buf);
        attribute_kind.into()
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<AttributeKindType>()
    }
}

#[cfg(test)]
mod test {
    use crate::data::core::AttributeKind;
    use shared_net::VSizedBuffer;

    #[test]
    fn attr_kind() {
        let orig1 = AttributeKind::Analyze;
        let orig2 = AttributeKind::Disrupt;

        let mut buf1 = VSizedBuffer::new(32);
        buf1.push(&orig1);
        buf1.push(&orig2);

        assert_eq!(orig1, buf1.pull::<AttributeKind>());

        let mut buf2 = VSizedBuffer::new(32);
        buf2.xfer::<AttributeKind>(&mut buf1);

        assert_eq!(orig2, buf2.pull::<AttributeKind>());
    }
}
