use crate::data::player::PlayerCard;
use crate::message::CommandMessage;
use shared_data::types::{GameIdType, PartType, SeedType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, VSizedBuffer};

type PartsArray = [PartType; 8];

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct GameBuildRequest {
    pub game_id: GameIdType,
    pub parts: PartsArray,
    pub commit: bool,
}

impl CommandMessage for GameBuildRequest {
    const COMMAND: op::Command = op::Command::GameBuild;
}

impl Bufferable for GameBuildRequest {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.game_id.push_into(buf);
        self.parts.push_into(buf);
        self.commit.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let game_id = GameIdType::pull_from(buf);
        let parts = PartsArray::pull_from(buf);
        let commit = bool::pull_from(buf);
        Self {
            game_id,
            parts,
            commit,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.game_id.size_in_buffer() + self.parts.size_in_buffer() + self.commit.size_in_buffer()
    }
}

pub struct GameBuildResponse {
    pub seed: SeedType,
    pub deck: Vec<PlayerCard>,
}

impl CommandMessage for GameBuildResponse {
    const COMMAND: op::Command = op::Command::GameBuild;
}

impl Bufferable for GameBuildResponse {
    fn push_into(&self, buf: &mut VSizedBuffer) {
        self.seed.push_into(buf);
        self.deck.push_into(buf);
    }

    fn pull_from(buf: &mut VSizedBuffer) -> Self {
        let seed = SeedType::pull_from(buf);
        let deck = Vec::<PlayerCard>::pull_from(buf);
        Self {
            seed,
            deck,
        }
    }

    fn size_in_buffer(&self) -> usize {
        self.seed.size_in_buffer() + self.deck.size_in_buffer()
    }
}

#[cfg(test)]
mod test {
    use shared_data::game::card;
    use shared_data::game::card::Rarity::Legendary;
    use shared_net::sizedbuffers::Bufferable;
    use shared_net::VSizedBuffer;

    use crate::data::player::player_card::PlayerCard;
    use crate::message::game_build::{GameBuildRequest, GameBuildResponse};

    pub const CARD_COUNT: usize = 40;

    #[test]
    fn test_request() {
        let orig = GameBuildRequest {
            game_id: 1234567890,
            parts: [1234567890, 1234567891, 1234567892, 1234567893, 1234567894, 1234567895, 1234567896, 1234567897],
            commit: true,
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
            deck: Vec::default(),
        };
        for i in 0..CARD_COUNT {
            orig.deck.push(PlayerCard {
                rarity: Legendary,
                number: i as card::CardNumberType,
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
