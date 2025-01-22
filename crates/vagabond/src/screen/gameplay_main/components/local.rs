use bevy::prelude::Component;

use hall::core::AttributeKind;

#[derive(Component)]
pub(crate) enum PlayerStateText {
    Attribute(usize, usize),
    Erg(usize),
    Deck,
    Heap,
}

#[derive(Component)]
pub(crate) struct AttributeRow {
    pub(crate) kind: AttributeKind,
}

impl AttributeRow {
    pub(crate) fn new(kind: AttributeKind) -> Self {
        Self {
            kind,
        }
    }
}

#[derive(Component)]
pub(crate) struct HandCard {
    pub(crate) index: usize,
}

impl HandCard {
    pub(crate) fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}
