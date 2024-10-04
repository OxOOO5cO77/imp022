use std::io::Error;

use clap::Parser;

use crate::data::build_instance::BuildInstance;
use crate::data::card::Card;
use crate::data::category_instance::CategoryInstance;
use crate::save_load::hall::*;
use crate::save_load::load_data;
use crate::save_load::vagabond::*;

mod data;
mod save_load;

#[derive(Parser)]
struct Args {
    #[arg(short = 'b', long)] build: bool,
    #[arg(short = 'c', long)] category: bool,
    #[arg(short = 'd', long)] card: bool,
    #[arg(short = 'H', long)] hall: bool,
    #[arg(short = 'V', long)] vagabond: bool,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    if std::fs::metadata("output").is_err() {
        std::fs::create_dir("output")?;
    }

    if args.build {
        process_build(&args)?;
    }

    if args.category {
        process_category(&args)?;
    }

    if args.card {
        process_card(&args)?;
    }

    Ok(())
}

fn process_build(args: &Args) -> Result<(), Error> {
    println!("[Smithy] BEGIN build");
    let builds: Vec<BuildInstance> = load_data("assets/data/build")?;

    if args.hall {
        output_builds_for_hall(&builds)?;
    }
    if args.vagabond {
        output_builds_for_vagabond(&builds)?;
    }

    println!("[Smithy] END build");
    Ok(())
}

fn process_category(args: &Args) -> Result<(), Error> {
    println!("[Smithy] BEGIN category");
    let categories: Vec<CategoryInstance> = load_data("assets/data/category")?;

    if args.hall {
        output_categories_for_hall(&categories)?;
    }
    if args.vagabond {
        output_categories_for_vagabond(&categories)?;
    }
    println!("[Smithy] END category");
    Ok(())
}

fn process_card(args: &Args) -> Result<(), Error> {
    println!("[Smithy] BEGIN card");
    let cards: Vec<Card> = load_data("assets/data/card")?;

    if args.hall {
        output_cards_for_hall(&cards)?;
    }
    if args.vagabond {
        output_cards_for_vagabond(&cards)?;
    }
    println!("[Smithy] END card");
    Ok(())
}
