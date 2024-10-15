use crate::data::build::DbBuild;
use crate::data::card::DbCard;
use crate::data::category::DbCategory;
use crate::save_load::save_data_single;
use hall::data::hall_build::HallBuild;
use hall::data::hall_card::HallCard;
use hall::data::hall_category::HallCategory;
use shared_data::game::opcode::OpCode;
use std::io::Error;

fn make_hall_card(card: &DbCard) -> HallCard {
    HallCard {
        rarity: card.rarity,
        number: card.number,
        set: card.set,
        kind: card.kind,
        cost: card.cost,
        delay: card.delay,
        priority: card.priority,
        launch_code: parse_rules(&card.rules_launch),
        run_code: parse_rules(&card.rules_run),
    }
}

fn parse_rules(rules: &str) -> Vec<OpCode> {
    let mut result= vec![];
    let mut chars = rules.chars();
    while let Some(c) = chars.next() {
        if let Some(opcode) = OpCode::process(c, &mut chars) {
            result.push(opcode);
        }
    }
    result
}

pub(crate) fn output_cards_for_hall(cards: &[DbCard]) -> Result<(), Error> {
    let hall_cards = cards.iter().map(make_hall_card).collect::<Vec<_>>();
    save_data_single(hall_cards, "output/hall_cards.ron")
}

fn make_hall_build(build_instance: &DbBuild) -> HallBuild {
    HallBuild {
        build: build_instance.build,
        number: build_instance.number,
        cards: build_instance.cards.clone(),
    }
}

pub(crate) fn output_builds_for_hall(builds: &[DbBuild]) -> Result<(), Error> {
    let hall_builds = builds.iter().map(make_hall_build).collect::<Vec<_>>();
    save_data_single(hall_builds, "output/hall_builds.ron")
}

fn make_hall_category(category_instance: &DbCategory) -> HallCategory {
    HallCategory {
        category: category_instance.category,
        number: category_instance.number,
        cards: category_instance.cards.clone(),
    }
}

pub(crate) fn output_categories_for_hall(categories: &[DbCategory]) -> Result<(), Error> {
    let hall_categories = categories.iter().map(make_hall_category).collect::<Vec<_>>();
    save_data_single(hall_categories, "output/hall_categories.ron")
}
