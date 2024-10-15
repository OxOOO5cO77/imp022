use crate::save_load::hall::output_cards_for_hall;
use crate::save_load::vagabond::output_cards_for_vagabond;
use crate::Args;
use shared_data::game::card::*;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use crate::data::common::DbRarity;

pub(crate) struct DbCard {
    pub title: String,
    pub rarity: Rarity,
    pub number: NumberType,
    pub set: SetType,
    pub kind: Kind,
    pub cost: CostType,
    pub delay: DelayType,
    pub priority: PriorityType,
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
    fn to_kind(&self) -> Kind {
        match self {
            DbKind::Analyze => Kind::Analyze,
            DbKind::Breach => Kind::Breach,
            DbKind::Compute => Kind::Compute,
            DbKind::Disrupt => Kind::Disrupt,
        }
    }
}

fn row_to_card(row: &PgRow) -> DbCard {
    DbCard {
        title: row.get("title"),
        rarity: row.get::<DbRarity, _>("rarity").to_rarity(),
        number: row.get::<i32, _>("number") as NumberType,
        set: row.get::<i32, _>("set") as SetType,
        kind: row.get::<DbKind, _>("kind").to_kind(),
        cost: row.get::<i32, _>("cost") as CostType,
        delay: row.get::<i32, _>("delay") as DelayType,
        priority: row.get::<i32, _>("priority") as PriorityType,
        rules_launch: row.get("rules_launch"),
        rules_run: row.get("rules_run"),
    }
}

pub(crate) async fn process_card(args: &Args, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    println!("[Smithy] BEGIN card");

    let rows = sqlx::query("SELECT * FROM card").fetch_all(pool).await?;
    let cards = rows
        .iter()
        .map(row_to_card)
        .collect::<Vec<DbCard>>()
        ;

    if args.hall {
        output_cards_for_hall(&cards)?;
    }
    if args.vagabond {
        output_cards_for_vagabond(&cards)?;
    }
    println!("[Smithy] END card");
    Ok(())
}
