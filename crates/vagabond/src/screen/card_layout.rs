use crate::manager::ScreenLayout;
use bevy::color::Color;
use bevy::prelude::{Commands, Component, Entity, Query, Sprite, Text2d, With};
use shared_data::attribute::AttributeKind;
use shared_data::card::ErgType;
use vagabond::data::VagabondCard;

#[derive(Component)]
pub(crate) struct CardLayoutPiece;

#[derive(Component, Clone)]
pub(crate) struct CardLayout {
    pub(crate) slot: usize,
    pub(crate) frame: Option<Entity>,
    pub(crate) title: Option<Entity>,
    pub(crate) cost: Option<Entity>,
    pub(crate) delay: Option<Entity>,
    pub(crate) priority: Option<Entity>,
    pub(crate) launch: Option<Entity>,
    pub(crate) run: Option<Entity>,
}

impl CardLayout {
    fn maybe_get_entity(commands: &mut Commands, screen_layout: &ScreenLayout, name: &str) -> Option<Entity> {
        screen_layout.get_entity(name).map(|entity| commands.entity(*entity)).map(|mut c| c.insert(CardLayoutPiece).id())
    }

    pub(crate) fn build(commands: &mut Commands, screen_layout: &ScreenLayout, base_name: &str, slot: usize) -> Entity {
        let card_layout = Self {
            slot,
            frame: Self::maybe_get_entity(commands, screen_layout, &format!("{}/frame", base_name)),
            title: Self::maybe_get_entity(commands, screen_layout, &format!("{}/title", base_name)),
            cost: Self::maybe_get_entity(commands, screen_layout, &format!("{}/cost", base_name)),
            delay: Self::maybe_get_entity(commands, screen_layout, &format!("{}/delay", base_name)),
            priority: Self::maybe_get_entity(commands, screen_layout, &format!("{}/priority", base_name)),
            launch: Self::maybe_get_entity(commands, screen_layout, &format!("{}/launch", base_name)),
            run: Self::maybe_get_entity(commands, screen_layout, &format!("{}/run", base_name)),
        };

        commands.entity(screen_layout.entity_or_default(base_name)).insert(card_layout).id()
    }

    pub(crate) fn populate(
        //
        &self,
        card: VagabondCard,
        text_q: &mut Query<&mut Text2d, With<CardLayoutPiece>>,
        sprite_q: &mut Query<&mut Sprite>,
    ) {
        if let Some(title) = self.title {
            if let Ok(mut title_text) = text_q.get_mut(title) {
                *title_text = card.title.into();
            }
        }
        if let Some(cost) = self.cost {
            if let Ok(mut cost_text) = text_q.get_mut(cost) {
                *cost_text = Self::map_kind_to_cost(card.kind, card.cost).into();
            }
        }
        if let Some(launch) = self.launch {
            if let Ok(mut launch_text) = text_q.get_mut(launch) {
                *launch_text = card.launch_rules.into();
            }
        }
        if let Some(run) = self.run {
            if let Ok(mut run_text) = text_q.get_mut(run) {
                *run_text = card.run_rules.into();
            }
        }
        if let Some(delay) = self.delay {
            if let Ok(mut delay_text) = text_q.get_mut(delay) {
                *delay_text = card.delay.to_string().into();
            }
        }
        if let Some(priority) = self.priority {
            if let Ok(mut priority_text) = text_q.get_mut(priority) {
                *priority_text = card.priority.to_string().into();
            }
        }

        if let Some(frame) = self.frame {
            if let Ok(mut frame_sprite) = sprite_q.get_mut(frame) {
                frame_sprite.color = Self::map_kind_to_color(card.kind);
            }
        }
    }

    fn map_kind_to_color(kind: AttributeKind) -> Color {
        match kind {
            AttributeKind::Analyze => Color::srgb_u8(128, 0, 128),
            AttributeKind::Breach => Color::srgb_u8(0, 128, 0),
            AttributeKind::Compute => Color::srgb_u8(0, 0, 128),
            AttributeKind::Disrupt => Color::srgb_u8(128, 128, 0),
        }
    }

    fn map_kind_to_cost(kind: AttributeKind, cost: ErgType) -> String {
        let letter = match kind {
            AttributeKind::Analyze => 'A',
            AttributeKind::Breach => 'B',
            AttributeKind::Compute => 'C',
            AttributeKind::Disrupt => 'D',
        };
        format!("{cost}{letter}")
    }
}
