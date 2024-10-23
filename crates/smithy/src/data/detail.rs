use crate::data::common::extract_cards;
use crate::save_load::hall::output_details_for_hall;
use crate::save_load::vagabond::output_details_for_vagabond;
use crate::Args;
use shared_data::game::card::CardSlot;
use shared_data::player::build::NumberType;
use shared_data::player::detail::*;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres, Row};

#[derive(FromRow)]
pub(crate) struct DbDetail {
    pub number: NumberType,
    pub detail: Detail,
    pub title: String,
    pub cards: Vec<CardSlot>,
}

fn compose_dis_con(build_3: &str) -> Consumer {
    match build_3 {
        "Casual" => Consumer::Casual,
        "Content" => Consumer::Content,
        "Gaming" => Consumer::Gaming,
        "Productivity" => Consumer::Productivity,
        _ => Consumer::Any,
    }
}
fn compose_dis_fri(build_3: &str) -> Fringe {
    match build_3 {
        "Exotic" => Fringe::Exotic,
        "Niche" => Fringe::Niche,
        "Retro" => Fringe::Retro,
        "Source" => Fringe::Source,
        _ => Fringe::Any,
    }
}
fn compose_dis_har(build_3: &str) -> Hardened {
    match build_3 {
        "Anonymous" => Hardened::Anonymous,
        "Crypto" => Hardened::Crypto,
        "Government" => Hardened::Government,
        "Industry" => Hardened::Industry,
        _ => Hardened::Any,
    }
}
fn compose_dis_res(build_3: &str) -> Restricted {
    match build_3 {
        "Access" => Restricted::Access,
        "Distribution" => Restricted::Distribution,
        "Install" => Restricted::Install,
        "Use" => Restricted::Use,
        _ => Restricted::Any,
    }
}

fn compose_dis(build_2: &str, build_3: &str) -> Distro {
    match build_2 {
        "Consumer" => Distro::Consumer(compose_dis_con(build_3)),
        "Fringe" => Distro::Fringe(compose_dis_fri(build_3)),
        "Hardened" => Distro::Hardened(compose_dis_har(build_3)),
        "Restricted" => Distro::Restricted(compose_dis_res(build_3)),
        _ => Distro::Any,
    }
}

fn compose_ins_aca(build_3: &str) -> Academic {
    match build_3 {
        "CompSci" => Academic::CompSci,
        "Cybernetics" => Academic::Cybernetics,
        "Engineering" => Academic::Engineering,
        "Theoretical" => Academic::Theoretical,
        _ => Academic::Any,
    }
}

fn compose_ins_bur(build_3: &str) -> Bureaucratic {
    match build_3 {
        "Africa" => Bureaucratic::Africa,
        "Americas" => Bureaucratic::Americas,
        "Asia" => Bureaucratic::Asia,
        "Europe" => Bureaucratic::Europe,
        _ => Bureaucratic::Any,
    }
}

fn compose_ins_cor(build_3: &str) -> Corporate {
    match build_3 {
        "Consumer" => Corporate::Consumer,
        "Entertainment" => Corporate::Entertainment,
        "Industrial" => Corporate::Industrial,
        "Military" => Corporate::Military,
        _ => Corporate::Any,
    }
}

fn compose_ins_dec(build_3: &str) -> Decentralized {
    match build_3 {
        "Activist" => Decentralized::Activist,
        "Enthusiast" => Decentralized::Enthusiast,
        "Freelance" => Decentralized::Freelance,
        "OpenSource" => Decentralized::OpenSource,
        _ => Decentralized::Any,
    }
}

fn compose_ins(build_2: &str, build_3: &str) -> Institution {
    match build_2 {
        "Academic" => Institution::Academic(compose_ins_aca(build_3)),
        "Bureaucratic" => Institution::Bureaucratic(compose_ins_bur(build_3)),
        "Corporate" => Institution::Corporate(compose_ins_cor(build_3)),
        "Decentralized" => Institution::Decentralized(compose_ins_dec(build_3)),
        _ => Institution::Any
    }
}

fn compose_loc_off(build_3: &str) -> Office {
    match build_3 {
        "Campus" => Office::Campus,
        "Ephemeral" => Office::Ephemeral,
        "Satellite" => Office::Satellite,
        "Tower" => Office::Tower,
        _ => Office::Any,
    }
}

fn compose_loc_pub(build_3: &str) -> Public {
    match build_3 {
        "Commercial" => Public::Commercial,
        "Education" => Public::Education,
        "Hospitality" => Public::Hospitality,
        "Municipal" => Public::Municipal,
        _ => Public::Any,
    }
}

fn compose_loc_res(build_3: &str) -> Residence {
    match build_3 {
        "Apartment" => Residence::Apartment,
        "Detached" => Residence::Detached,
        "Hotel" => Residence::Hotel,
        "Shared" => Residence::Shared,
        _ => Residence::Any,
    }
}

fn compose_loc_una(build_3: &str) -> Unauthorized {
    match build_3 {
        "Infrastructure" => Unauthorized::Infrastructure,
        "Office" => Unauthorized::Office,
        "Public" => Unauthorized::Public,
        "Residential" => Unauthorized::Residential,
        _ => Unauthorized::Any,
    }
}

fn compose_loc(build_2: &str, build_3: &str) -> Location {
    match build_2 {
        "Office" => Location::Office(compose_loc_off(build_3)),
        "Public" => Location::Public(compose_loc_pub(build_3)),
        "Residence" => Location::Residence(compose_loc_res(build_3)),
        "Unauthorized" => Location::Unauthorized(compose_loc_una(build_3)),
        _ => Location::Any
    }
}

fn compose_rol_dev(build_3: &str) -> Developer {
    match build_3 {
        "Art" => Developer::Art,
        "Production" => Developer::Production,
        "Programming" => Developer::Programming,
        "QA" => Developer::QA,
        _ => Developer::Any,
    }
}

fn compose_rol_it(build_3: &str) -> IT {
    match build_3 {
        "DevOps" => IT::DevOps,
        "Hardware" => IT::Hardware,
        "Security" => IT::Security,
        "Support" => IT::Support,
        _ => IT::Any,
    }
}

fn compose_rol_peo(build_3: &str) -> People {
    match build_3 {
        "Accounting" => People::Accounting,
        "Admin" => People::Admin,
        "HR" => People::HR,
        "Marketing" => People::Marketing,
        _ => People::Any,
    }
}

fn compose_rol_phy(build_3: &str) -> Physical {
    match build_3 {
        "Maintenance" => Physical::Maintenance,
        "Security" => Physical::Security,
        "Supply" => Physical::Supply,
        "Trades" => Physical::Trades,
        _ => Physical::Any,
    }
}

fn compose_rol(build_2: &str, build_3: &str) -> Role {
    match build_2 {
        "Developer" => Role::Developer(compose_rol_dev(build_3)),
        "IT" => Role::IT(compose_rol_it(build_3)),
        "People" => Role::People(compose_rol_peo(build_3)),
        "Physical" => Role::Physical(compose_rol_phy(build_3)),
        _ => Role::Any
    }
}

fn compose_detail(build_1: &str, build_2: &str, build_3: &str) -> Detail {
    match build_1 {
        "Distro" => Detail::Distro(compose_dis(build_2, build_3)),
        "Institution" => Detail::Institution(compose_ins(build_2, build_3)),
        "Location" => Detail::Location(compose_loc(build_2, build_3)),
        "Role" => Detail::Role(compose_rol(build_2, build_3)),
        _ => Detail::Any
    }
}


fn row_to_detail(row: &PgRow) -> DbDetail {
    DbDetail {
        number: row.get::<i32, _>("number") as NumberType,
        detail: compose_detail(row.get("detail_1"), row.get("detail_2"), row.get("detail_3")),
        title: row.get("title"),
        cards: extract_cards(row),

    }
}

pub(crate) async fn process_detail(args: &Args, pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    println!("[Smithy] BEGIN detail");

    let rows = sqlx::query("SELECT * FROM detail").fetch_all(pool).await?;

    let details = rows
        .iter()
        .map(row_to_detail)
        .collect::<Vec<DbDetail>>()
        ;

    if args.hall {
        output_details_for_hall(&details)?;
    }
    if args.vagabond {
        output_details_for_vagabond(&details)?;
    }

    println!("[Smithy] END detail");
    Ok(())
}
