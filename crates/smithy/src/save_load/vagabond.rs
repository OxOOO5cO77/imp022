use std::io::Error;
use vagabond::data::vagabond_build::VagabondBuild;
use vagabond::data::vagabond_card::VagabondCard;
use vagabond::data::vagabond_detail::VagabondDetail;

use crate::data::build::DbBuild;
use crate::data::card::DbCard;
use crate::data::detail::DbDetail;
use crate::save_load::save_data_single;

fn make_vagabond_card(card: &DbCard) -> VagabondCard {
    VagabondCard {
        title: card.title.clone(),
        rarity: card.rarity,
        number: card.number,
        set: card.set,
        kind: card.kind,
        cost: card.cost,
        queue: card.delay,
        priority: card.priority,
        launch_rules: card.rules_launch.clone(),
        run_rules: card.rules_launch.clone(),
    }
}

pub(crate) fn output_cards_for_vagabond(cards: &[DbCard]) -> Result<(), Error> {
    let vagabond_cards = cards.iter().map(make_vagabond_card).collect::<Vec<_>>();
    save_data_single(vagabond_cards, "output/vagabond_cards.ron")
}

fn make_vagabond_build(build_instance: &DbBuild) -> VagabondBuild {
    VagabondBuild {
        build: build_instance.build,
        number: build_instance.number,
        title: build_instance.title.clone(),
    }
}

pub(crate) fn output_builds_for_vagabond(builds: &[DbBuild]) -> Result<(), Error> {
    let vagabond_builds = builds.iter().map(make_vagabond_build).collect::<Vec<_>>();
    save_data_single(vagabond_builds, "output/vagabond_builds.ron")
}

fn make_vagabond_detail(detail_instance: &DbDetail) -> VagabondDetail {
    VagabondDetail {
        detail: detail_instance.detail,
        number: detail_instance.number,
        title: detail_instance.title.clone(),
    }
}

pub(crate) fn output_details_for_vagabond(details: &[DbDetail]) -> Result<(), Error> {
    let vagabond_details = details.iter().map(make_vagabond_detail).collect::<Vec<_>>();
    save_data_single(vagabond_details, "output/vagabond_details.ron")
}
