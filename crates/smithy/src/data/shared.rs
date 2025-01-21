use sqlx::postgres::PgRow;
use sqlx::Row;

use hall::core::{BuildNumberType, CardSlot, Host, Rarity, Set, SetType, Slot};

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_rarity")]
pub(crate) enum DbRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl From<DbRarity> for Rarity {
    fn from(value: DbRarity) -> Self {
        match value {
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

impl From<DbCardSlot> for CardSlot {
    fn from(value: DbCardSlot) -> CardSlot {
        CardSlot(
            Set(value.set as SetType),
            value.rarity.into(),
            if value.number == 0 {
                Slot::Any
            } else {
                Slot::Number(value.number as BuildNumberType)
            },
        )
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_host")]
pub(crate) enum DbHost {
    None,
    Local,
    Remote,
}

impl From<DbHost> for Host {
    fn from(value: DbHost) -> Self {
        match value {
            DbHost::None => Host::None,
            DbHost::Local => Host::Local,
            DbHost::Remote => Host::Remote,
        }
    }
}

pub(crate) fn extract_cards(row: &PgRow, count: usize) -> Vec<CardSlot> {
    let mut cards: Vec<CardSlot> = Vec::new();
    for i in 1..=count {
        let row_name = format!("cardslot_{i}");
        let slot = row.get::<DbCardSlot, _>(row_name.as_str()).into();
        cards.push(slot);
    }
    cards
}
