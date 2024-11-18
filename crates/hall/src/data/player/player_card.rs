use std::mem::size_of;

use serde::{Deserialize, Serialize};

use crate::data::hall::HallCard;
use shared_data::game::card;
use shared_data::game::card::Rarity;
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

pub type PackedCardType = u16;

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct PlayerCard {
    pub rarity: Rarity,
    pub number: card::CardNumberType,
    pub set: card::SetType,
}

const BITS_FOR_NUMBER: PackedCardType = 8;
const BITS_FOR_SET: PackedCardType = 6;
const BITS_FOR_RARITY: PackedCardType = 2;

const SHIFT_FOR_NUMBER: PackedCardType = 0;
const SHIFT_FOR_SET: PackedCardType = SHIFT_FOR_NUMBER + BITS_FOR_NUMBER;
const SHIFT_FOR_RARITY: PackedCardType = SHIFT_FOR_SET + BITS_FOR_SET;

const MASK_FOR_NUMBER: PackedCardType = (1 << BITS_FOR_NUMBER) - 1;
const MASK_FOR_SET: PackedCardType = (1 << BITS_FOR_SET) - 1;
const MASK_FOR_RARITY: PackedCardType = (1 << BITS_FOR_RARITY) - 1;

impl PlayerCard {
    fn pack(&self) -> PackedCardType {
        let packed_rarity = match self.rarity {
            Rarity::Common => 0,
            Rarity::Uncommon => 1,
            Rarity::Rare => 2,
            Rarity::Legendary => 3,
        } as PackedCardType;
        let packed_set = self.set as PackedCardType;
        let packed_number = self.number as PackedCardType;
        (packed_rarity << SHIFT_FOR_RARITY) | (packed_set << SHIFT_FOR_SET) | (packed_number << SHIFT_FOR_NUMBER)
    }

    fn unpack(packed: PackedCardType) -> Self {
        let unpacked_number = (packed >> SHIFT_FOR_NUMBER) & MASK_FOR_NUMBER;
        let unpacked_set = (packed >> SHIFT_FOR_SET) & MASK_FOR_SET;
        let unpacked_rarity = (packed >> SHIFT_FOR_RARITY) & MASK_FOR_RARITY;
        Self {
            number: unpacked_number as card::CardNumberType,
            set: unpacked_set as card::SetType,
            rarity: match unpacked_rarity {
                1 => Rarity::Uncommon,
                2 => Rarity::Rare,
                3 => Rarity::Legendary,
                _ => Rarity::Common,
            },
        }
    }
    pub fn size_in_bytes() -> usize {
        size_of::<PackedCardType>()
    }
}

impl From<&HallCard> for PlayerCard {
    fn from(card: &HallCard) -> PlayerCard {
        PlayerCard {
            rarity: card.rarity,
            number: card.number,
            set: card.set,
        }
    }
}

impl Bufferable for PlayerCard {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.pack().push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        Self::unpack(PackedCardType::pull_from(buf))
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<PackedCardType>()
    }
}

#[cfg(test)]
mod test {
    use crate::data::player::player_card::PlayerCard;
    use shared_data::game::card::Rarity;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    #[test]
    fn test_player_card() {
        let orig_card = PlayerCard {
            rarity: Rarity::Legendary,
            number: 34,
            set: 12,
        };

        let mut buf = VSizedBuffer::new(orig_card.size_in_buffer());
        buf.push(&orig_card);
        let new_card = buf.pull::<PlayerCard>();

        assert_eq!(orig_card.rarity, new_card.rarity);
        assert_eq!(orig_card.number, new_card.number);
        assert_eq!(orig_card.set, new_card.set);
    }
}
