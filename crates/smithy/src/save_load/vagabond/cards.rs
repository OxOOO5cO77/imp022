use crate::data::rules;
use crate::data::DbCard;
use crate::save_load::save_data_single;
use std::io::Error;
use vagabond::data::VagabondCard;

fn make_vagabond_card(card: &DbCard) -> VagabondCard {
    VagabondCard {
        title: card.title.clone(),
        rarity: card.rarity,
        number: card.number,
        set: card.set,
        kind: card.kind,
        cost: card.cost,
        delay: card.delay,
        priority: card.priority,
        launch_rules: rules::parse_rules(&card.rules_launch),
        run_rules: rules::parse_rules(&card.rules_run),
    }
}

pub(crate) fn output_cards_for_vagabond(cards: &[DbCard]) -> Result<(), Error> {
    let vagabond_cards = cards.iter().map(make_vagabond_card).collect::<Vec<_>>();
    save_data_single(vagabond_cards, "output/vagabond_cards.ron")
}