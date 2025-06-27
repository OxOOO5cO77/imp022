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
        let ant = commands.entity(layout.entity(&format!("{name}/ant"))).id();
        let brd = commands.entity(layout.entity(&format!("{name}/brd"))).id();
        let cpu = commands.entity(layout.entity(&format!("{name}/cpu"))).id();
        let dsk = commands.entity(layout.entity(&format!("{name}/dsk"))).id();
        part_layout.build = [ant, brd, cpu, dsk];

        let ins = commands.entity(layout.entity(&format!("{name}/ins"))).id();
        let rol = commands.entity(layout.entity(&format!("{name}/rol"))).id();
        let loc = commands.entity(layout.entity(&format!("{name}/loc"))).id();
        let dis = commands.entity(layout.entity(&format!("{name}/dis"))).id();
        part_layout.detail = [ins, rol, loc, dis];

        let a = commands.entity(layout.entity(&format!("{name}/a"))).id();
        let b = commands.entity(layout.entity(&format!("{name}/b"))).id();
        let c = commands.entity(layout.entity(&format!("{name}/c"))).id();
        let d = commands.entity(layout.entity(&format!("{name}/d"))).id();
        part_layout.values = [a, b, c, d];

        part_layout
    }
}
