use std::collections::VecDeque;
use std::mem::size_of;

use shared_data::types::{GameIdType, PartType, SeedType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::VSizedBuffer;

use crate::data::player_card::{PackedCardType, PlayerCard};

const PART_COUNT: usize = 8;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameBuildRequest {
    pub game_id: GameIdType,
    pub parts: [PartType; PART_COUNT],
}

impl Bufferable for GameBuildRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        for part in &self.parts {
            part.push_into(buf);
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let mut result = Self {
            game_id,
            parts: [0; PART_COUNT],
        };
        for i in 0..result.parts.len() {
            result.parts[i] = PartType::pull_from(buf);
        }
        result
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<GameIdType>() + (size_of::<PartType>() * self.parts.len())
    }
}

pub const CARD_COUNT: usize = 40;

pub struct GameBuildResponse {
    pub seed: SeedType,
    pub deck: VecDeque<PlayerCard>,
}

impl Bufferable for GameBuildResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.seed.push_into(buf);
        (self.deck.len() as u8).push_into(buf);
        for card in &self.deck {
            card.push_into(buf);
        }
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let seed = SeedType::pull_from(buf);
        let mut result = Self {
            seed,
            deck: VecDeque::new(),
        };
        let len = u8::pull_from(buf);
        for _ in 0..len {
            result.deck.push_back(PlayerCard::pull_from(buf));
        }
        result
    }

    fn size_in_buffer(&self) -> usize {
        size_of::<SeedType>() + size_of::<u8>() +(size_of::<PackedCardType>() * CARD_COUNT)
    }
}

#[cfg(test)]
mod test {
    use shared_data::player::card;
    use shared_data::player::card::Rarity::Legendary;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player_card::PlayerCard;
    use crate::message::gamebuild::{CARD_COUNT, GameBuildRequest, GameBuildResponse};

    #[test]
    fn test_request() {
        let orig = GameBuildRequest {
            game_id: 1234567890,
            parts: [1234567890, 1234567891, 1234567892, 1234567893, 1234567894, 1234567895, 1234567896, 1234567897],
        };

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameBuildRequest>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig, result);
    }

    #[test]
    fn test_response() {
        let mut orig = GameBuildResponse {
            seed: 1234567890,
            deck: Default::default(),
        };
        for i in 0..CARD_COUNT {
            orig.deck.push_back(PlayerCard {
                rarity: Legendary,
                number: i as card::NumberType,
                set: 1,
            });
        }

        let mut buf = VSizedBuffer::new(orig.size_in_buffer());
        buf.push(&orig);
        let result = buf.pull::<GameBuildResponse>();

        assert_eq!(buf.size(), orig.size_in_buffer());
        assert_eq!(orig.seed, result.seed);
        assert_eq!(orig.deck.len(), result.deck.len());
        assert_eq!(orig.deck.iter().last().unwrap().number, result.deck.iter().last().unwrap().number)
    }
}
