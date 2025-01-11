use sqlx::postgres::PgRow;
use sqlx::Row;

use hall::core::{BuildNumberType, CardSlot, Rarity, Set, SetType, Slot};

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_rarity")]
pub(crate) enum DbRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl DbRarity {
    pub(crate) fn to_rarity(&self) -> Rarity {
        match self {
            DbRarity::Common => Rarity::Common,
            DbRarity::Uncommon => Rarity::Uncommon,
            DbRarity::Rare => Rarity::Rare,
            DbRarity::Legendary => Rarity::Legendary,
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_cardslot")]
struct DbCardSlot {
    set: i32,
    rarity: DbRarity,
    number: i32,
}

impl DbCardSlot {
    fn to_cardslot(&self) -> CardSlot {
        CardSlot(
            Set(self.set as SetType),
            self.rarity.to_rarity(),
            if self.number == 0 {
                Slot::Any
            } else {
                Slot::Number(self.number as BuildNumberType)
            },
        )
    }
}

pub(crate) fn extract_cards(row: &PgRow, count: usize) -> Vec<CardSlot> {
    let mut cards: Vec<CardSlot> = Vec::new();
    for i in 1..=count {
        let row_name = format!("cardslot_{i}");
        let slot = row.get::<DbCardSlot, _>(row_name.as_str()).to_cardslot();
        cards.push(slot);
    }
    cards
}
