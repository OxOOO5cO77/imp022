use crate::data::{DbBuild, DbCard, DbDetail, DbMission, DbMissionNode};
use crate::save_load::save_data_single;
use hall::data::core::{Instruction, InstructionValueType};
use hall::data::hall::{HallBuild, HallCard, HallDetail, HallMission, HallMissionNode};
use std::io::Error;
use std::str::Chars;

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

fn char_as_delta(c: char) -> InstructionValueType {
    ((c as i32) - ('a' as i32)) as InstructionValueType
}

pub fn process_code(c: char, chars: &mut Chars) -> Option<Instruction> {
    use Instruction::*;
    match c {
        'f' => {
            let amount_c = chars.next().unwrap_or_default();
            Some(ChangeFreeSpace(char_as_delta(amount_c)))
        }
        't' => {
            let amount_c = chars.next().unwrap_or_default();
            Some(ChangeThermalCapacity(char_as_delta(amount_c)))
        }
        's' => {
            let amount_c = chars.next().unwrap_or_default();
            Some(ChangeSystemHealth(char_as_delta(amount_c)))
        }
        'o' => {
            let amount_c = chars.next().unwrap_or_default();
            Some(ChangeOpenPorts(char_as_delta(amount_c)))
        }
        _ => None,
    }
}

fn parse_rules(rules: &str) -> Vec<Instruction> {
    let mut result = vec![];
    let mut chars = rules.chars();
    while let Some(c) = chars.next() {
        if let Some(instruction) = process_code(c, &mut chars) {
            result.push(instruction);
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

fn make_hall_mission_node(mission_node_instance: &DbMissionNode) -> HallMissionNode {
    HallMissionNode {
        id: mission_node_instance.node_id,
        kind: mission_node_instance.kind,
        state: mission_node_instance.state,
        links: mission_node_instance.links.clone(),
        content: vec![],
    }
}

fn make_hall_mission(mission_instance: &DbMission) -> HallMission {
    HallMission {
        id: mission_instance.mission_id,
        node: mission_instance.node.iter().map(make_hall_mission_node).collect(),
        objective: vec![],
    }
}

pub(crate) fn output_missions_for_hall(missions: &[DbMission]) -> Result<(), Error> {
    let hall_missions = missions.iter().map(make_hall_mission).collect::<Vec<_>>();
    save_data_single(hall_missions, "output/hall_missions.ron")
}
