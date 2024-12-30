use crate::manager::ScreenLayout;
use bevy::prelude::{Commands, Component, Entity};

#[derive(Component)]
pub(crate) struct PartLayout {
    pub(crate) build: [Entity; 4],
    pub(crate) detail: [Entity; 4],
    pub(crate) values: [Entity; 4],
}

impl PartLayout {
    pub(crate) fn new() -> Self {
        Self {
            build: [Entity::PLACEHOLDER; 4],
            detail: [Entity::PLACEHOLDER; 4],
            values: [Entity::PLACEHOLDER; 4],
        }
    }

    pub(crate) fn populate_full(commands: &mut Commands, layout: &ScreenLayout, name: &str) -> Self {
        let mut part_layout = Self::new();
        let ant = commands.entity(layout.entity(&format!("{}/ant", name))).id();
        let brd = commands.entity(layout.entity(&format!("{}/brd", name))).id();
        let cpu = commands.entity(layout.entity(&format!("{}/cpu", name))).id();
        let dsk = commands.entity(layout.entity(&format!("{}/dsk", name))).id();
        part_layout.build = [ant, brd, cpu, dsk];

        let ins = commands.entity(layout.entity(&format!("{}/ins", name))).id();
        let rol = commands.entity(layout.entity(&format!("{}/rol", name))).id();
        let loc = commands.entity(layout.entity(&format!("{}/loc", name))).id();
        let dis = commands.entity(layout.entity(&format!("{}/dis", name))).id();
        part_layout.detail = [ins, rol, loc, dis];

        let a = commands.entity(layout.entity(&format!("{}/a", name))).id();
        let b = commands.entity(layout.entity(&format!("{}/b", name))).id();
        let c = commands.entity(layout.entity(&format!("{}/c", name))).id();
        let d = commands.entity(layout.entity(&format!("{}/d", name))).id();
        part_layout.values = [a, b, c, d];

        part_layout
    }
}
