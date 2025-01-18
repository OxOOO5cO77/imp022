use num_enum::{FromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::{AttributeArray, AttributeArrays, AttributeValueType};

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

#[repr(u8)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, FromPrimitive, IntoPrimitive)]
pub enum AttributeValueKind {
    #[num_enum(default)]
    Amplitude,
    Beat,
    Control,
    Duration,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
pub struct AttributeValues {
    pub amplitude: AttributeValueType,
    pub beat: AttributeValueType,
    pub control: AttributeValueType,
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
    pub fn to_array(&self) -> AttributeArray {
        [self.amplitude, self.beat, self.control, self.duration]
    }

    pub fn from_array(array: AttributeArray) -> Self {
        let [amplitude, beat, control, duration] = array;
        Self {
            amplitude,
            beat,
            control,
            duration,
        }
    }

    pub fn get_value(&self, value: AttributeValueKind) -> AttributeValueType {
        match value {
            AttributeValueKind::Amplitude => self.amplitude,
            AttributeValueKind::Beat => self.beat,
            AttributeValueKind::Control => self.control,
            AttributeValueKind::Duration => self.duration,
        }
    }
}

impl Attributes {
    pub fn to_arrays(&self) -> AttributeArrays {
        [self.analyze.to_array(), self.breach.to_array(), self.compute.to_array(), self.disrupt.to_array()]
    }

    pub fn from_arrays(array: AttributeArrays) -> Self {
        let [analyze, breach, compute, disrupt] = array.map(AttributeValues::from_array);
        Self {
            analyze,
            breach,
            compute,
            disrupt,
        }
    }

    pub fn get_value(&self, kind: AttributeKind, value: AttributeValueKind) -> AttributeValueType {
        match kind {
            AttributeKind::Analyze => self.analyze,
            AttributeKind::Breach => self.breach,
            AttributeKind::Compute => self.compute,
            AttributeKind::Disrupt => self.disrupt,
        }
        .get_value(value)
    }
    pub fn get_values(&self, kind: AttributeKind) -> AttributeArray {
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
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let attribute_kind: AttributeKindType = (*self).into();
        attribute_kind.push_into(buf)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let attribute_kind = AttributeKindType::pull_from(buf)?;
        Ok(attribute_kind.into())
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<AttributeKindType>()
    }
}

#[cfg(test)]
mod test {
    use crate::core::AttributeKind;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn attr_kind() -> Result<(), SizedBufferError> {
        let orig1 = AttributeKind::Analyze;
        let orig2 = AttributeKind::Disrupt;

        let mut buf1 = SizedBuffer::new(32);
        buf1.push(&orig1)?;
        buf1.push(&orig2)?;

        assert_eq!(orig1, buf1.pull::<AttributeKind>()?);

        let mut buf2 = SizedBuffer::new(32);
        buf2.xfer::<AttributeKind>(&mut buf1)?;

        assert_eq!(orig2, buf2.pull::<AttributeKind>()?);
        Ok(())
    }
}
