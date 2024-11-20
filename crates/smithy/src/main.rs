use crate::data::build::process_build;
use crate::data::card::process_card;
use crate::data::detail::process_detail;
use crate::save_load::hall::*;
use crate::save_load::vagabond::*;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use crate::data::mission::process_mission;

mod data;
mod save_load;

#[derive(Parser)]
struct Args {
    #[arg(short = 'u', long)] user: String,
    #[arg(short = 'p', long)] password: String,
    #[arg(short = 'd', long)] database: String,
    #[arg(long)] build: bool,
    #[arg(long)] card: bool,
    #[arg(long)] detail: bool,
    #[arg(long)] mission: bool,
    #[arg(short = 'H', long)] hall: bool,
    #[arg(short = 'V', long)] vagabond: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let args = Args::parse();

    let connection = format!("postgres://{}:{}@{}/smithy", args.user, args.password, args.database);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(connection.as_str())
        .await
        .map_err(|e| {
            println!("[ERROR] sqlx error: {}", e);
            e
        })?
        ;

    if std::fs::metadata("output").is_err() {
        std::fs::create_dir("output")?;
    }

    if args.build {
        println!("[Smithy] BEGIN build");
        let (builds, company, market) = process_build(&pool).await?;
        if args.hall {
            output_builds_for_hall(&builds)?;
        }
        if args.vagabond {
            output_builds_for_vagabond(&builds, company, market)?;
        }
        println!("[Smithy] END build");
    }

    if args.detail {
        println!("[Smithy] BEGIN detail");
        let (details, general, specific) = process_detail(&pool).await?;
        if args.hall {
            output_details_for_hall(&details)?;
        }
        if args.vagabond {
            output_details_for_vagabond(&details, general, specific)?;
        }
        println!("[Smithy] END detail");
    }

    if args.card {
        println!("[Smithy] BEGIN card");
        let cards = process_card(&pool).await?;

        if args.hall {
            output_cards_for_hall(&cards)?;
        }
        if args.vagabond {
            output_cards_for_vagabond(&cards)?;
        }
        println!("[Smithy] END card");
    }

    if args.mission {
        println!("[Smithy] BEGIN mission");
        let missions = process_mission(&pool).await?;

        if args.hall {
            output_missions_for_hall(&missions)?;
        }
        if args.vagabond {
            output_missions_for_vagabond(&missions)?;
        }
        println!("[Smithy] END mission");
    }

    Ok(())
}


