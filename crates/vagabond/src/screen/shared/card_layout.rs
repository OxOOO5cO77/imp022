use crate::manager::{AtlasManager, ScreenLayout};
use crate::screen::shared::util;
use crate::screen::shared::util::KindIconSize;
use crate::system::ui_effects::UiFxTrackedColor;
use bevy::color::Color;
use bevy::prelude::{Commands, Component, Entity, Event, Query, Res, Sprite, Text2d, Trigger, Visibility, With};
use hall::data::core::{AttributeKind, Attributes, Instruction, ValueTarget};
use vagabond::data::VagabondCard;

#[derive(Component)]
pub(crate) struct CardLayoutPiece;

#[derive(Event, Default)]
pub(crate) struct CardPopulateEvent {
    card: Option<VagabondCard>,
    attr: Attributes,
}

impl CardPopulateEvent {
    pub(crate) fn new(card: Option<VagabondCard>, attr: Attributes) -> Self {
        Self {
            card,
            attr,
        }
    }
}

#[derive(Component, Clone)]
pub(crate) struct CardLayout {
    pub(crate) frame: Option<Entity>,
    pub(crate) icon: Option<Entity>,
    pub(crate) title: Option<Entity>,
    pub(crate) cost: Option<Entity>,
    pub(crate) delay: Option<Entity>,
    pub(crate) priority: Option<Entity>,
    pub(crate) launch: Option<Entity>,
    pub(crate) run: Option<Entity>,
}

impl CardLayout {
    pub(crate) fn build(commands: &mut Commands, screen_layout: &ScreenLayout, base_name: &str) -> Entity {
        let card_layout = Self {
            frame: Self::maybe_get_entity(commands, screen_layout, &format!("{}/frame", base_name)),
            icon: Self::maybe_get_entity(commands, screen_layout, &format!("{}/icon", base_name)),
            title: Self::maybe_get_entity(commands, screen_layout, &format!("{}/title", base_name)),
            cost: Self::maybe_get_entity(commands, screen_layout, &format!("{}/cost", base_name)),
            delay: Self::maybe_get_entity(commands, screen_layout, &format!("{}/delay", base_name)),
            priority: Self::maybe_get_entity(commands, screen_layout, &format!("{}/priority", base_name)),
            launch: Self::maybe_get_entity(commands, screen_layout, &format!("{}/launch", base_name)),
            run: Self::maybe_get_entity(commands, screen_layout, &format!("{}/run", base_name)),
        };

        commands.entity(screen_layout.entity(base_name)).insert(card_layout).observe(Self::on_populate).id()
    }
}

impl CardLayout {
    fn maybe_get_entity(commands: &mut Commands, screen_layout: &ScreenLayout, name: &str) -> Option<Entity> {
        screen_layout.entity_option(name).map(|entity| commands.entity(*entity)).map(|mut c| c.insert(CardLayoutPiece).id())
    }

    fn on_populate(
        // bevy system
        event: Trigger<CardPopulateEvent>,
        mut commands: Commands,
        layout_q: Query<&CardLayout>,
        mut text_q: Query<&mut Text2d, With<CardLayoutPiece>>,
        mut sprite_q: Query<&mut Sprite>,
        am: Res<AtlasManager>,
    ) {
        let target = event.entity();
        match (&event.card, layout_q.get(target)) {
            (Some(card), Ok(layout)) => {
                layout.title.map(|title| text_q.get_mut(title).map(|mut title_text| *title_text = card.title.clone().into()));
                layout.cost.map(|cost| text_q.get_mut(cost).map(|mut cost_text| *cost_text = card.cost.to_string().into()));
                layout.launch.map(|launch| text_q.get_mut(launch).map(|mut launch_text| *launch_text = Self::explain_rules(&card.launch_rules, &event.attr).into()));
                layout.run.map(|run| text_q.get_mut(run).map(|mut run_text| *run_text = Self::explain_rules(&card.run_rules, &event.attr).into()));
                layout.delay.map(|delay| text_q.get_mut(delay).map(|mut delay_text| *delay_text = card.delay.to_string().into()));
                layout.priority.map(|priority| text_q.get_mut(priority).map(|mut priority_text| *priority_text = card.priority.to_string().into()));
                layout.icon.map(|icon| sprite_q.get_mut(icon).map(|mut icon_sprite| util::replace_kind_icon(&mut icon_sprite, card.kind, KindIconSize::Small, &am)));
                layout.frame.map(|frame| {
                    let color = Self::map_kind_to_color(card.kind);
                    commands.entity(frame).insert(UiFxTrackedColor::from(color.to_srgba()));
                    sprite_q.get_mut(frame).map(|mut frame_sprite| frame_sprite.color = color)
                });

                commands.entity(target).insert(Visibility::Visible);
            }
            _ => {
                commands.entity(target).insert(Visibility::Hidden);
            }
        };
    }

    fn map_kind_to_color(kind: AttributeKind) -> Color {
        match kind {
            AttributeKind::Analyze => Color::srgb_u8(128, 0, 128),
            AttributeKind::Breach => Color::srgb_u8(0, 128, 0),
            AttributeKind::Compute => Color::srgb_u8(0, 0, 128),
            AttributeKind::Disrupt => Color::srgb_u8(128, 128, 0),
        }
    }

    fn explain_rules(rules: &[Instruction], attr: &Attributes) -> String {
        rules.iter().filter_map(|rule| Self::explain_rule(rule, attr)).collect::<Vec<String>>().join("\n")
    }

    fn explain_rule(rule: &Instruction, attr: &Attributes) -> Option<String> {
        match rule {
            Instruction::NoOp => None,
            Instruction::TTL(value) => Some(format!("TTL:{}", value.resolve(attr))),
            Instruction::INC(target, value) => Some(format!("{} -> {}", value.resolve(attr), Self::explain_target(target))),
            Instruction::DEC(target, value) => Some(format!("-{} -> {}", value.resolve(attr), Self::explain_target(target))),
        }
    }

    fn explain_target(target: &ValueTarget) -> &str {
        match target {
            ValueTarget::None => "???",
            ValueTarget::FreeSpace => "Free Space",
            ValueTarget::ThermalCapacity => "Thermal Capacity",
            ValueTarget::SystemHealth => "System Health",
            ValueTarget::OpenPorts => "Open Ports",
        }
    }
}
