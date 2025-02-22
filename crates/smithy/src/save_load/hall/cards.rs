use std::io::Error;

use hall_lib::hall::HallCard;

use crate::data::DbCard;
use crate::data::rules;
use crate::save_load::save_data_single;

fn make_hall_card(card: &DbCard) -> HallCard {
    HallCard {
        rarity: card.rarity,
        number: card.number,
        set: card.set,
        kind: card.kind,
        cost: card.cost,
        delay: card.delay,
        priority: card.priority,
        host: card.host,
        launch_code: rules::parse_rules_launch(&card.rules_launch),
        run_code: rules::parse_rules_run(&card.rules_run),
    }
}

pub(crate) fn output_cards_for_hall(cards: &[DbCard]) -> Result<(), Error> {
    let hall_cards = cards.iter().map(make_hall_card).collect::<Vec<_>>();
    save_data_single(hall_cards, "output/hall_cards.ron")
}
