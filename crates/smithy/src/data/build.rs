#![allow(clippy::upper_case_acronyms)]

use crate::data::shared::extract_cards;
use hall::data::core::{Build, BuildNumberType, CardSlot, CompanyType, GeneralType, MarketType, SpecificType};
use serde::Deserialize;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct DbBuild {
    pub number: BuildNumberType,
    pub build: Build,
    pub title: String,
    pub cards: Vec<CardSlot>,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "type_build")]
enum DbBuildType {
    ANT,
    BRD,
    CPU,
    DSK,
}

fn compose_build(kind: DbBuildType, company: CompanyType, market: MarketType) -> Build {
    match kind {
        DbBuildType::ANT => Build::ANT(company, market),
        DbBuildType::BRD => Build::BRD(company, market),
        DbBuildType::CPU => Build::CPU(company, market),
        DbBuildType::DSK => Build::DSK(company, market),
    }
}

fn row_to_build(row: &PgRow) -> DbBuild {
    DbBuild {
        number: row.get::<i32, _>("number") as BuildNumberType,
        build: compose_build(row.get("kind"), row.get::<i32, _>("company") as CompanyType, row.get::<i32, _>("market") as SpecificType),
        title: row.get("title"),
        cards: extract_cards(row, 15),
    }
}

pub(crate) async fn process_build(pool: &Pool<Postgres>) -> Result<(Vec<DbBuild>, HashMap<CompanyType, String>, HashMap<MarketType, String>), sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM build").fetch_all(pool).await?;

    let builds = rows.iter().map(row_to_build).collect::<Vec<DbBuild>>();
    let company_rows = sqlx::query("SELECT id,name FROM \"build/company\"").fetch_all(pool).await?;
    let company = company_rows.iter().map(|row| (row.get::<i32, _>("id") as GeneralType, row.get("name"))).collect::<HashMap<CompanyType, String>>();
    let market_rows = sqlx::query("SELECT id,name FROM \"build/market\"").fetch_all(pool).await?;
    let market = market_rows.iter().map(|row| (row.get::<i32, _>("id") as SpecificType, row.get("name"))).collect::<HashMap<MarketType, String>>();

    Ok((builds, company, market))
}
