use crate::manager::ScreenLayout;
use crate::screen::util;
use bevy::prelude::{Commands, Component, Entity, Query, Sprite, Text2d, With};
use vagabond::data::VagabondCard;

#[derive(Component)]
pub(crate) struct CardText;

#[derive(Component, Clone)]
pub(crate) struct CardLayout {
    pub(crate) slot: usize,
    pub(crate) frame: Entity,
    pub(crate) title: Entity,
    pub(crate) cost: Entity,
    pub(crate) delay: Entity,
    pub(crate) priority: Entity,
    pub(crate) launch: Entity,
    pub(crate) run: Entity,
}

impl CardLayout {
    pub(crate) fn build(commands: &mut Commands, screen_layout: &ScreenLayout, slot: usize, base_name: &str) -> Entity {
        let card_layout = Self {
            slot,
            frame: commands.entity(screen_layout.entity(&format!("{}/frame", base_name))).id(),
            title: commands.entity(screen_layout.entity(&format!("{}/title", base_name))).insert(CardText).id(),
            cost: commands.entity(screen_layout.entity(&format!("{}/cost", base_name))).insert(CardText).id(),
            delay: commands.entity(screen_layout.entity(&format!("{}/delay", base_name))).insert(CardText).id(),
            priority: commands.entity(screen_layout.entity(&format!("{}/priority", base_name))).insert(CardText).id(),
            launch: commands.entity(screen_layout.entity(&format!("{}/launch", base_name))).insert(CardText).id(),
            run: commands.entity(screen_layout.entity(&format!("{}/run", base_name))).insert(CardText).id(),
        };

        commands.entity(screen_layout.entity(base_name)).insert(card_layout).id()
    }

    pub(crate) fn populate(
        //
        &self,
        card: VagabondCard,
        text_q: &mut Query<&mut Text2d, With<CardText>>,
        sprite_q: &mut Query<&mut Sprite>,
    ) {
        if let Ok(mut title_text) = text_q.get_mut(self.title) {
            *title_text = card.title.into();
        }
        if let Ok(mut cost_text) = text_q.get_mut(self.cost) {
            *cost_text = util::map_kind_to_cost(card.kind, card.cost).into();
        }
        if let Ok(mut launch_text) = text_q.get_mut(self.launch) {
            *launch_text = card.launch_rules.into();
        }
        if let Ok(mut run_text) = text_q.get_mut(self.run) {
            *run_text = card.run_rules.into();
        }
        if let Ok(mut delay_text) = text_q.get_mut(self.delay) {
            *delay_text = card.delay.to_string().into();
        }
        if let Ok(mut priority_text) = text_q.get_mut(self.priority) {
            *priority_text = card.priority.to_string().into();
        }

        if let Ok(mut frame) = sprite_q.get_mut(self.frame) {
            frame.color = util::map_kind_to_color(card.kind);
        }
    }
}
