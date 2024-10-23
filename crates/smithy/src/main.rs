use crate::data::build::process_build;
use crate::data::card::process_card;
use crate::data::detail::process_detail;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

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
        process_build(&args, &pool).await?;
    }

    if args.detail {
        process_detail(&args, &pool).await?;
    }

    if args.card {
        process_card(&args, &pool).await?;
    }

    Ok(())
}


