use bevy::prelude::{Commands, Component, Entity, Event, Query, Srgba, Text2d, Trigger, Visibility, With};

use hall::core::{AttributeKind, Attributes, CardTargetValue, Host, LaunchInstruction, RunInstruction, ValueTarget};
use vagabond::data::VagabondCard;

use crate::manager::ScreenLayout;
use crate::screen::shared::util;
use crate::system::ui_effects::{SetColorEvent, UiFxTrackedColor};

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

    pub(crate) fn empty() -> Self {
        Self {
            card: None,
            attr: Attributes::default(),
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
    pub(crate) host: Option<Entity>,
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
            host: Self::maybe_get_entity(commands, screen_layout, &format!("{}/host", base_name)),
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
    ) {
        let target = event.entity();
        match (&event.card, layout_q.get(target)) {
            (Some(card), Ok(layout)) => {
                layout.title.map(|title| text_q.get_mut(title).map(|mut title_text| *title_text = card.title.clone().into()));
                layout.cost.map(|cost| text_q.get_mut(cost).map(|mut cost_text| *cost_text = card.cost.to_string().into()));
                layout.launch.map(|launch| text_q.get_mut(launch).map(|mut launch_text| *launch_text = Self::explain_rules_launch(&card.launch_rules, &event.attr).into()));
                layout.run.map(|run| text_q.get_mut(run).map(|mut run_text| *run_text = Self::explain_rules_run(&card.run_rules, &event.attr).into()));
                layout.delay.map(|delay| text_q.get_mut(delay).map(|mut delay_text| *delay_text = card.delay.to_string().into()));
                layout.priority.map(|priority| text_q.get_mut(priority).map(|mut priority_text| *priority_text = card.priority.to_string().into()));
                layout.host.map(|host| text_q.get_mut(host).map(|mut host_text| *host_text = Self::explain_host(card.host).into()));
                layout.icon.map(|icon| text_q.get_mut(icon).map(|mut icon_text| *icon_text = util::kind_icon(card.kind).into()));
                if let Some(frame) = layout.frame {
                    let color = Self::map_kind_to_color(card.kind);
                    commands.entity(frame).insert(UiFxTrackedColor::from(color)).trigger(SetColorEvent::new(frame, color));
                }

                commands.entity(target).insert(Visibility::Visible);
            }
            _ => {
                commands.entity(target).insert(Visibility::Hidden);
            }
        };
    }

    fn map_kind_to_color(kind: AttributeKind) -> Srgba {
        match kind {
            AttributeKind::Analyze => Srgba::rgb_u8(128, 0, 128),
            AttributeKind::Breach => Srgba::rgb_u8(0, 128, 0),
            AttributeKind::Compute => Srgba::rgb_u8(0, 0, 128),
            AttributeKind::Disrupt => Srgba::rgb_u8(128, 128, 0),
        }
    }

    fn explain_rules_launch(rules: &[LaunchInstruction], attr: &Attributes) -> String {
        rules.iter().filter_map(|rule| Self::explain_rule_launch(rule, attr)).collect::<Vec<String>>().join("\n")
    }

    fn explain_rule_launch(rule: &LaunchInstruction, attr: &Attributes) -> Option<String> {
        match rule {
            LaunchInstruction::NoOp => None,
            LaunchInstruction::Targ(target) => {
                let str = match target {
                    CardTargetValue::None => "No target",
                    CardTargetValue::Machine => "Target: Machine",
                    CardTargetValue::Actor => "Target: User",
                };
                Some(str.to_string())
            }
            LaunchInstruction::Loop(value) => Some(format!("Loop {} times", value.resolve(attr))),
        }
    }

    fn explain_rules_run(rules: &[RunInstruction], attr: &Attributes) -> String {
        rules.iter().filter_map(|rule| Self::explain_rule_run(rule, attr)).collect::<Vec<String>>().join("\n")
    }

    fn explain_rule_run(rule: &RunInstruction, attr: &Attributes) -> Option<String> {
        match rule {
            RunInstruction::NoOp => None,
            RunInstruction::IncV(target, value) => Some(format!("Increase {} by {}", Self::explain_target(target), value.resolve(attr))),
            RunInstruction::DecV(target, value) => Some(format!("Decrease {} by {}", Self::explain_target(target), value.resolve(attr))),
            RunInstruction::Cred => Some("Copy Credentials".to_string()),
        }
    }

    fn explain_target(target: &ValueTarget) -> char {
        match target {
            ValueTarget::None => '?',
            ValueTarget::FreeSpace => '🖫',
            ValueTarget::ThermalCapacity => '🌡',
            ValueTarget::SystemHealth => '✚',
            ValueTarget::OpenPorts => '🖧',
        }
    }

    fn explain_host(host: Host) -> &'static str {
        match host {
            Host::None => "Runs Immediately",
            Host::Local => "Runs Locally",
            Host::Remote => "Runs Remotely",
        }
    }
}
