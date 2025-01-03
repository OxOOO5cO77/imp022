use crate::data::rules;
use crate::data::DbCard;
use crate::save_load::save_data_single;
use hall::data::hall::HallCard;
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
        launch_code: rules::parse_rules(&card.rules_launch),
        run_code: rules::parse_rules(&card.rules_run),
    }
}

pub(crate) fn output_cards_for_hall(cards: &[DbCard]) -> Result<(), Error> {
    let hall_cards = cards.iter().map(make_hall_card).collect::<Vec<_>>();
    save_data_single(hall_cards, "output/hall_cards.ron")
}
