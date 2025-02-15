use shared_net::{Bufferable, SizedBuffer, SizedBufferError};

use crate::core::{AttributeArrays, AttributeValueType, PriorityType};
use crate::player::PlayerCard;

#[derive(Default)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameProcessPlayerView {
    pub player_card: PlayerCard,
    pub priority: PriorityType,
    pub local: bool,
    pub attributes: AttributeArrays,
}

type PackedPriorityLocalType = u8;
type PackedAttributesType = u64;

impl GameProcessPlayerView {
    fn pack_priority_local(&self) -> PackedPriorityLocalType {
        self.priority << 1
            | if self.local {
                1
            } else {
                0
            }
    }
    fn unpack_priority_local(packed: PackedPriorityLocalType) -> (PriorityType, bool) {
        let priority = packed >> 1;
        let local = (packed & 0x01) == 1;
        (priority, local)
    }

    fn pack_attributes(&self) -> PackedAttributesType {
        let mut packed = 0u64;
        for array in &self.attributes {
            for attribute in array {
                packed <<= 4;
                packed |= (attribute & 0x0F) as PackedAttributesType;
            }
        }
        packed
    }
    fn unpack_attributes(packed: PackedAttributesType) -> AttributeArrays {
        let mut result = AttributeArrays::default();
        let mut remain = packed;
        for outer in result.iter_mut().rev() {
            for inner in outer.iter_mut().rev() {
                *inner = (remain & 0x0F) as AttributeValueType;
                remain >>= 4;
            }
        }
        result
    }
}

impl Bufferable for GameProcessPlayerView {
    fn push_into(&self, buf: &mut SizedBuffer) -> Result<usize, SizedBufferError> {
        let mut pushed = 0;
        pushed += self.player_card.push_into(buf)?;
        pushed += self.pack_priority_local().push_into(buf)?;
        pushed += self.pack_attributes().push_into(buf)?;
        Ok(pushed)
    }

    fn pull_from(buf: &mut SizedBuffer) -> Result<Self, SizedBufferError> {
        let player_card = PlayerCard::pull_from(buf)?;
        let (priority, local) = Self::unpack_priority_local(PackedPriorityLocalType::pull_from(buf)?);
        let attributes = Self::unpack_attributes(PackedAttributesType::pull_from(buf)?);
        let result = Self {
            player_card,
            priority,
            local,
            attributes,
        };
        Ok(result)
    }

    fn size_in_buffer(&self) -> usize {
        self.player_card.size_in_buffer() + size_of::<PackedPriorityLocalType>() + size_of::<PackedAttributesType>()
    }
}

#[cfg(test)]
impl GameProcessPlayerView {
    pub fn test_default() -> Self {
        Self {
            player_card: PlayerCard {
                rarity: crate::core::Rarity::Legendary,
                number: 123,
                set: 2,
            },
            priority: 5,
            local: true,
            attributes: [[1, 1, 9, 9], [5, 5, 5, 5], [9, 9, 1, 1], [6, 7, 4, 9]],
        }
    }
}

#[cfg(test)]
mod test {
    use crate::view::GameProcessPlayerView;
    use shared_net::{SizedBuffer, SizedBufferError};

    #[test]
    fn test_process_player_view() -> Result<(), SizedBufferError> {
        let orig = GameProcessPlayerView::test_default();

        let mut buf = SizedBuffer::from(&orig)?;
        let result = buf.pull::<GameProcessPlayerView>()?;

        assert_eq!(orig.player_card, result.player_card);
        assert_eq!(orig.priority, result.priority);
        assert_eq!(orig.local, result.local);
        Ok(())
    }
}
