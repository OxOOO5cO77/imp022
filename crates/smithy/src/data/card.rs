use crate::data::shared::{DbHost, DbRarity};
use hall::core::{AttributeKind, CardNumberType, DelayType, ErgType, Host, PriorityType, Rarity, SetType};
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

pub(crate) struct DbCard {
    pub title: String,
    pub rarity: Rarity,
    pub number: CardNumberType,
    pub set: SetType,
    pub kind: AttributeKind,
    pub cost: ErgType,
    pub delay: DelayType,
    pub priority: PriorityType,
    pub host: Host,
    pub rules_launch: String,
    pub rules_run: String,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_kind")]
enum DbKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
}

impl DbKind {
    fn to_kind(&self) -> AttributeKind {
        match self {
            DbKind::Analyze => AttributeKind::Analyze,
            DbKind::Breach => AttributeKind::Breach,
            DbKind::Compute => AttributeKind::Compute,
            DbKind::Disrupt => AttributeKind::Disrupt,
        }
    }
}

fn row_to_card(row: &PgRow) -> DbCard {
    DbCard {
        title: row.get("title"),
        rarity: row.get::<DbRarity, _>("rarity").into(),
        number: row.get::<i32, _>("number") as CardNumberType,
        set: row.get::<i32, _>("set") as SetType,
        kind: row.get::<DbKind, _>("kind").to_kind(),
        cost: row.get::<i32, _>("cost") as ErgType,
        delay: row.get::<i32, _>("delay") as DelayType,
        priority: row.get::<i32, _>("priority") as PriorityType,
        host: row.get::<DbHost, _>("host").into(),
        rules_launch: row.get("rules_launch"),
        rules_run: row.get("rules_run"),
    }
}

pub(crate) async fn process_card(pool: &Pool<Postgres>) -> Result<Vec<DbCard>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM card").fetch_all(pool).await?;
    let cards = rows.iter().map(row_to_card).collect::<Vec<DbCard>>();

    Ok(cards)
}
