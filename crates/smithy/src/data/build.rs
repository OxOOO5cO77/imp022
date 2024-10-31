#![allow(clippy::upper_case_acronyms)]

use crate::data::common::extract_cards;
use serde::Deserialize;
use shared_data::game::card::CardSlot;
use shared_data::player::build::{Build, CompanyType, MarketType, NumberType};
use shared_data::player::detail::SpecificType;
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

#[derive(Deserialize)]
pub(crate) struct DbBuild {
    pub number: NumberType,
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
        number: row.get::<i32, _>("number") as NumberType,
        build: compose_build(row.get("kind"), row.get::<i32, _>("company") as CompanyType, row.get::<i32, _>("market") as SpecificType),
        title: row.get("title"),
        cards: extract_cards(row),
    }
}

pub(crate) async fn process_build(pool: &Pool<Postgres>) -> Result<Vec<DbBuild>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM build").fetch_all(pool).await?;

    let builds = rows
        .iter()
        .map(row_to_build)
        .collect::<Vec<DbBuild>>()
        ;

    Ok(builds)
}
