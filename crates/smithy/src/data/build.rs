use crate::data::common::extract_cards;
use crate::save_load::hall::output_builds_for_hall;
use crate::save_load::vagabond::output_builds_for_vagabond;
use crate::Args;
use serde::Deserialize;
use shared_data::game::card::CardSlot;
use shared_data::player::build::{Build, Market, NumberType, ANT, BRD, CPU, DSC};
use sqlx::postgres::PgRow;
use sqlx::{Pool, Postgres, Row};

#[derive(Deserialize)]
pub(crate) struct DbBuild {
    pub number: NumberType,
    pub build: Build,
    pub title: String,
    pub cards: Vec<CardSlot>,
}

fn row_to_build(row: &PgRow) -> DbBuild {
    DbBuild {
        number: row.get::<i32, _>("number") as NumberType,
        build: compose_build(row.get("build_1"), row.get("build_2"), row.get("build_3")),
        title: row.get("title"),
        cards: extract_cards(row),
    }
}

fn compose_market(build_3: &str) -> Market {
    match build_3 {
        "Consumer" => Market::Consumer,
        "Enthusiast" => Market::Enthusiast,
        "Prosumer" => Market::Prosumer,
        "Professional" => Market::Professional,
        _ => Market::Any,
    }
}

fn compose_ant(build_2: &str, build_3: &str) -> ANT {
    match build_2 {
        "EXM" => ANT::EXM(compose_market(build_3)),
        "NetTECH" => ANT::NetTECH(compose_market(build_3)),
        "TransGlobal" => ANT::TransGlobal(compose_market(build_3)),
        "Uplink" => ANT::Uplink(compose_market(build_3)),
        _ => ANT::Any,
    }
}

fn compose_brd(build_2: &str, build_3: &str) -> BRD {
    match build_2 {
        "Axis" => BRD::Axis(compose_market(build_3)),
        "PeriPeri" => BRD::PeriPeri(compose_market(build_3)),
        "SilPath" => BRD::SilPath(compose_market(build_3)),
        "Wasbleibt" => BRD::Wasbleibt(compose_market(build_3)),
        _ => BRD::Any,
    }
}

fn compose_cpu(build_2: &str, build_3: &str) -> CPU {
    match build_2 {
        "CCC" => CPU::CCC(compose_market(build_3)),
        "Orbital" => CPU::Orbital(compose_market(build_3)),
        "RiscFree" => CPU::RiscFree(compose_market(build_3)),
        "Visor" => CPU::Visor(compose_market(build_3)),
        _ => CPU::Any,
    }
}

fn compose_dsc(build_2: &str, build_3: &str) -> DSC {
    match build_2 {
        "Evoke" => DSC::Evoke(compose_market(build_3)),
        "Kollectiv" => DSC::Kollectiv(compose_market(build_3)),
        "Vault" => DSC::Vault(compose_market(build_3)),
        "Warehaus" => DSC::Warehaus(compose_market(build_3)),
        _ => DSC::Any,
    }
}


fn compose_build(build_1: &str, build_2: &str, build_3: &str) -> Build {
    match build_1 {
        "ANT" => Build::ANT(compose_ant(build_2, build_3)),
        "BRD" => Build::BRD(compose_brd(build_2, build_3)),
        "CPU" => Build::CPU(compose_cpu(build_2, build_3)),
        "DSC" => Build::DSC(compose_dsc(build_2, build_3)),
        _ => Build::Any
    }
}

pub(crate) async fn process_build(args: &Args, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    println!("[Smithy] BEGIN build");
    let rows = sqlx::query("SELECT * FROM build").fetch_all(pool).await?;

    let builds = rows
        .iter()
        .map(row_to_build)
        .collect::<Vec<DbBuild>>()
        ;

    if args.hall {
        output_builds_for_hall(&builds)?;
    }
    if args.vagabond {
        output_builds_for_vagabond(&builds)?;
    }

    println!("[Smithy] END build");
    Ok(())
}
