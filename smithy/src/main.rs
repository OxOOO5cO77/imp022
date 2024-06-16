use std::io::Error;

use crate::data::build_instance::BuildInstance;
use crate::data::card::Card;
use crate::data::category_instance::CategoryInstance;
use crate::save_load::hall::*;
use crate::save_load::load_data;
use crate::save_load::vagabond::*;

mod data;
mod save_load;

fn main() -> Result<(), Error> {
    if std::fs::metadata("output").is_err() {
        std::fs::create_dir("output")?;
    }
    println!("[Smithy] BEGIN build");
    process_build()?;
    println!("[Smithy] END build");

    println!("[Smithy] BEGIN category");
    process_category()?;
    println!("[Smithy] END category");

    println!("[Smithy] BEGIN card");
    process_card()?;
    println!("[Smithy] END card");

    Ok(())
}

fn process_build() -> Result<(), Error> {
    let builds: Vec<BuildInstance> = load_data("assets/data/build")?;

    output_builds_for_hall(&builds)?;
    output_builds_for_vagabond(&builds)?;

    Ok(())
}

fn process_category() -> Result<(), Error> {
    let categories: Vec<CategoryInstance> = load_data("assets/data/category")?;

    output_categories_for_hall(&categories)?;
    output_categories_for_vagabond(&categories)?;

    Ok(())
}

fn process_card() -> Result<(), Error> {
    let cards: Vec<Card> = load_data("assets/data/card")?;

    output_cards_for_hall(&cards)?;
    output_cards_for_vagabond(&cards)?;

    Ok(())
}
