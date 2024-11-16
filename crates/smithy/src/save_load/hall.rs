use crate::save_load::save_data_single;
use shared_data::game::opcode::OpCode;
use std::io::Error;
use hall::data::hall::{HallBuild, HallCard, HallDetail};
use crate::data::{DbBuild, DbCard, DbDetail};

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

fn make_hall_detail(detail_instance: &DbDetail) -> HallDetail {
    HallDetail {
        detail: detail_instance.detail,
        number: detail_instance.number,
        cards: detail_instance.cards.clone(),
    }
}

pub(crate) fn output_details_for_hall(details: &[DbDetail]) -> Result<(), Error> {
    let hall_details = details.iter().map(make_hall_detail).collect::<Vec<_>>();
    save_data_single(hall_details, "output/hall_details.ron")
}
