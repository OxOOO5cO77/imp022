use bevy::prelude::{Component, Entity};

#[derive(Debug, Copy, Clone)]
pub(crate) enum StatRowKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
    Build,
    Detail,
}

#[derive(Debug, Component)]
pub(crate) enum Slot {
    StatRow(StatRowKind),
    Build,
    Detail,
    Card,
    Empty(Entity),
}
