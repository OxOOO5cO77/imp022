use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres, Row};

use hall::core::{BuildNumberType, CardSlot, Detail, GeneralType, SpecificType};

use crate::data::shared::extract_cards;

#[derive(FromRow)]
pub(crate) struct DbDetail {
    pub number: BuildNumberType,
    pub detail: Detail,
    pub title: String,
    pub cards: Vec<CardSlot>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_detail")]
enum DbDetailType {
    Distro,
    Institution,
    Location,
    Role,
}

fn compose_detail(kind: DbDetailType, general: GeneralType, specific: SpecificType) -> Detail {
    match kind {
        DbDetailType::Distro => Detail::Distro(general, specific),
        DbDetailType::Institution => Detail::Institution(general, specific),
        DbDetailType::Location => Detail::Location(general, specific),
        DbDetailType::Role => Detail::Role(general, specific),
    }
}

fn row_to_detail(row: &PgRow) -> DbDetail {
    DbDetail {
        number: row.get::<i32, _>("number") as BuildNumberType,
        detail: compose_detail(row.get("kind"), row.get::<i32, _>("general") as GeneralType, row.get::<i32, _>("specific") as SpecificType),
        title: row.get("title"),
        cards: extract_cards(row, 15),
    }
}

pub(crate) async fn process_detail(pool: &Pool<Postgres>) -> Result<(Vec<DbDetail>, Vec<(GeneralType, String, String)>, Vec<(SpecificType, String, String)>), sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM detail").fetch_all(pool).await?;

    let details = rows.iter().map(row_to_detail).collect::<Vec<DbDetail>>();

    let general_rows = sqlx::query("SELECT id,name,glyph FROM \"detail/general\"").fetch_all(pool).await?;
    let mut general = general_rows.iter().map(|row| (row.get::<i32, _>("id") as GeneralType, row.get("name"), row.get("glyph"))).collect::<Vec<(GeneralType, String, String)>>();
    let specific_rows = sqlx::query("SELECT id,name,glyph FROM \"detail/specific\"").fetch_all(pool).await?;
    let mut specific = specific_rows.iter().map(|row| (row.get::<i32, _>("id") as SpecificType, row.get("name"), row.get("glyph"))).collect::<Vec<(SpecificType, String, String)>>();

    general.sort();
    specific.sort();

    Ok((details, general, specific))
}
