use std::io::Error;
use vagabond::data::vagabond_build::VagabondBuild;
use vagabond::data::vagabond_card::VagabondCard;
use vagabond::data::vagabond_category::VagabondCategory;

use crate::data::build_instance::BuildInstance;
use crate::data::card::Card;
use crate::data::category_instance::CategoryInstance;
use crate::save_load::save_data_single;

fn make_vagabond_card(card: &Card) -> VagabondCard {
    VagabondCard {
        title: card.title.clone(),
        rarity: card.rarity,
        number: card.number,
        set: card.set,
        kind: card.kind,
        cost: card.cost,
        rules: card.rules.clone(),
    }
}

pub(crate) fn output_cards_for_vagabond(cards: &[Card]) -> Result<(), Error> {
    let vagabond_cards = cards.iter().map(make_vagabond_card).collect::<Vec<_>>();
    save_data_single(vagabond_cards, "output/vagabond_cards.ron")
}

fn make_vagabond_build(build_instance: &BuildInstance) -> VagabondBuild {
    VagabondBuild {
        build: build_instance.build,
        number: build_instance.number,
        title: build_instance.title.clone(),
    }
}

pub(crate) fn output_builds_for_vagabond(builds: &[BuildInstance]) -> Result<(), Error> {
    let vagabond_builds = builds.iter().map(make_vagabond_build).collect::<Vec<_>>();
    save_data_single(vagabond_builds, "output/vagabond_builds.ron")
}

fn make_vagabond_category(category_instance: &CategoryInstance) -> VagabondCategory {
    VagabondCategory {
        category: category_instance.category,
        number: category_instance.number,
        title: category_instance.title.clone(),
    }
}

pub(crate) fn output_categories_for_vagabond(categories: &[CategoryInstance]) -> Result<(), Error> {
    let vagabond_categories = categories.iter().map(make_vagabond_category).collect::<Vec<_>>();
    save_data_single(vagabond_categories, "output/vagabond_categories.ron")
}
