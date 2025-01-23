use std::cmp::PartialEq;
use std::collections::HashMap;

use bevy::prelude::*;

use hall::core::{AttributeKind, Attributes, CardTargetMachineKind, CardTargetValue, DelayType, LaunchInstruction, MissionNodeKind, PickedCardTarget, TokenKind};
use hall::message::*;
use vagabond::data::VagabondCard;

use crate::manager::{AtlasManager, ScreenLayoutManager, ScreenLayoutManagerParams};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::gameplay_init::GameplayInitHandoff;
use crate::screen::gameplay_main::nodes::*;
use crate::screen::gameplay_main::{components::*, events::*, resources::*, systems::*};
use crate::screen::shared::{on_out_reset_color, on_update_card_tooltip, AppScreenExt, CardLayout, CardTooltip, GameMissionNodePlayerViewExt, MissionNodeKindExt, UpdateCardTooltipEvent};
use crate::system::ui_effects::{Blinker, SetColorEvent, TextTip, UiFxTrackedColor};
use crate::system::AppState;

mod components;
mod events;
mod nodes;
mod resources;
mod systems;

const SCREEN_LAYOUT: &str = "gameplay";

const BLINKER_COUNT: f32 = 2.0;
const BLINKER_SPEED: f32 = 24.0;
const GLOWER_SPEED: f32 = 4.0;

const PROCESS_QUEUE_SIZE: DelayType = 10;
const HAND_SIZE: usize = 5;
const RUNNING_PROGRAM_COUNT: usize = 6;
const TTY_MESSAGE_COUNT: usize = 9;

pub struct GameplayMainPlugin;

impl Plugin for GameplayMainPlugin {
    //noinspection Duplicates
    fn build(&self, app: &mut App) {
        app //
            .add_screen(AppState::Gameplay)
            .with_enter(gameplay_enter)
            .with_update(gameplay_update)
            .with_post_update(cleanup_indicator_post_update)
            .with_exit(gameplay_exit);
    }
}

const INDICATOR_Z: f32 = 100.0;

#[derive(Clone, Copy, PartialEq)]
enum WaitKind {
    One,
    All,
}

#[derive(Default, Clone, Copy, PartialEq)]
enum VagabondGamePhase {
    #[default]
    Start,
    Pick,
    Play,
    Draw,
    Wait(WaitKind),
}

trait PickableEntityCommandsExtension {
    fn observe_pickable_row(self, kind: AttributeKind) -> Self;
    fn observe_next_button(self) -> Self;
    fn observe_hand_card(self, hand_index: usize) -> Self;
    fn observe_process(self, kind: MachineKind, queue_index: DelayType) -> Self;
    fn observe_running(self, kind: MachineKind, running_index: usize) -> Self;
}

impl PickableEntityCommandsExtension for &mut EntityCommands<'_> {
    fn observe_pickable_row(self, kind: AttributeKind) -> Self {
        self //
            .insert((AttributeRow::new(kind), PickingBehavior::default()))
            .observe(on_click_attr)
            .observe(on_over_attr)
            .observe(on_out_reset_color)
    }
    fn observe_next_button(self) -> Self {
        self //
            .insert(PickingBehavior::default())
            .observe(on_click_next)
            .observe(on_over_next)
            .observe(on_out_reset_color)
    }
    fn observe_hand_card(self, hand_index: usize) -> Self {
        self //
            .insert((HandCard::new(hand_index), PickingBehavior::default()))
            .observe(on_card_drag_start)
            .observe(on_card_drag)
            .observe(on_card_drag_end)
    }
    fn observe_process(self, kind: MachineKind, queue_index: DelayType) -> Self {
        self //
            .insert((kind, MachineQueueItem::new(queue_index), PickingBehavior::default()))
            .observe(on_over_process)
            .observe(on_out_hide_card_tooltip)
    }
    fn observe_running(self, kind: MachineKind, running_index: usize) -> Self {
        self //
            .insert((kind, MachineRunning::new(running_index), PickingBehavior::default()))
            .observe(on_over_running)
            .observe(on_out_hide_card_tooltip)
    }
}

fn gameplay_enter(
    // bevy system
    mut commands: Commands,
    mut handoff: ResMut<GameplayInitHandoff>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    mut slm_params: ScreenLayoutManagerParams,
) {
    let layout = slm.build(&mut commands, SCREEN_LAYOUT, &am, &mut slm_params, Some(GameplaySystems::observe));

    let container = commands.entity(layout.entity("text_tip")).insert_text_tip_container(layout.entity("text_tip/text")).id();
    commands.entity(layout.entity("attributes/a")).insert_text_tip(container, "Analyze");
    commands.entity(layout.entity("attributes/b")).insert_text_tip(container, "Breach");
    commands.entity(layout.entity("attributes/c")).insert_text_tip(container, "Compute");
    commands.entity(layout.entity("attributes/d")).insert_text_tip(container, "Disrupt");

    const LOCAL_ATTR: &[&[&str]] = &[
        //
        &["attributes/aa", "attributes/ab", "attributes/ac", "attributes/ad"],
        &["attributes/ba", "attributes/bb", "attributes/bc", "attributes/bd"],
        &["attributes/ca", "attributes/cb", "attributes/cc", "attributes/cd"],
        &["attributes/da", "attributes/db", "attributes/dc", "attributes/dd"],
    ];

    for (row_idx, row) in LOCAL_ATTR.iter().enumerate() {
        for (col_idx, name) in row.iter().enumerate() {
            commands.entity(layout.entity(name)).insert(PlayerStateText::Attribute(row_idx, col_idx));
        }
    }

    const ROLL: &[&str] = &["ea", "eb", "ec", "ed"];

    for (roll_idx, roll) in ROLL.iter().enumerate() {
        commands.entity(layout.entity(roll)).insert(RollText::new(roll_idx));
    }

    const REMOTE_ATTR: &[&str] = &["ra", "rb", "rc", "rd"];

    for (remote_idx, remote) in REMOTE_ATTR.iter().enumerate() {
        commands.entity(layout.entity(remote)).insert(RemoteAttrText::new(remote_idx));
    }
    commands.entity(layout.entity("r_icon")).insert((RemoteAttrIcon, Visibility::Hidden));

    const ERG: &[&str] = &["la", "lb", "lc", "ld"];

    for (erg_idx, erg) in ERG.iter().enumerate() {
        commands.entity(layout.entity(erg)).insert(PlayerStateText::Erg(erg_idx));
    }

    commands.entity(layout.entity("deck")).insert(PlayerStateText::Deck);
    commands.entity(layout.entity("heap")).insert(PlayerStateText::Heap);

    commands.entity(layout.entity("phase_start")).insert(PhaseIcon::new(VagabondGamePhase::Start)).insert_text_tip(container, "Start");
    commands.entity(layout.entity("phase_pick")).insert(PhaseIcon::new(VagabondGamePhase::Pick)).insert_text_tip(container, "Pick");
    commands.entity(layout.entity("phase_play")).insert(PhaseIcon::new(VagabondGamePhase::Play)).insert_text_tip(container, "Play");
    commands.entity(layout.entity("phase_draw")).insert(PhaseIcon::new(VagabondGamePhase::Draw)).insert_text_tip(container, "Draw");

    commands.entity(layout.entity("next")).observe_next_button();

    commands.entity(layout.entity("attributes/row_a")).observe_pickable_row(AttributeKind::Analyze);
    commands.entity(layout.entity("attributes/row_b")).observe_pickable_row(AttributeKind::Breach);
    commands.entity(layout.entity("attributes/row_c")).observe_pickable_row(AttributeKind::Compute);
    commands.entity(layout.entity("attributes/row_d")).observe_pickable_row(AttributeKind::Disrupt);

    const MACHINES: &[(&str, MachineKind, PickedCardTarget)] = &[("local", MachineKind::Local, PickedCardTarget::MachineLocal), ("remote", MachineKind::Remote, PickedCardTarget::MachineRemote)];

    for (machine_name, machine_kind, target) in MACHINES {
        commands.entity(layout.entity(machine_name)).insert((*machine_kind, CardDropTarget::new(*target), PickingBehavior::default())).observe(on_card_drop);

        commands.entity(layout.entity(&format!("{machine_name}/title"))).insert((*machine_kind, MachineText::new(MachineTextKind::Title)));
        commands.entity(layout.entity(&format!("{machine_name}/id"))).insert((*machine_kind, MachineText::new(MachineTextKind::Id)));

        commands.entity(layout.entity(&format!("{machine_name}/free_space"))).insert((*machine_kind, MachineText::new(MachineTextKind::Vitals(0))));
        commands.entity(layout.entity(&format!("{machine_name}/thermal_capacity"))).insert((*machine_kind, MachineText::new(MachineTextKind::Vitals(1))));
        commands.entity(layout.entity(&format!("{machine_name}/system_health"))).insert((*machine_kind, MachineText::new(MachineTextKind::Vitals(2))));
        commands.entity(layout.entity(&format!("{machine_name}/open_ports"))).insert((*machine_kind, MachineText::new(MachineTextKind::Vitals(3))));

        for queue_index in 0..PROCESS_QUEUE_SIZE {
            commands.entity(layout.entity(&format!("{machine_name}/queue{queue_index}"))).observe_process(*machine_kind, queue_index);
        }

        for running_index in 0..RUNNING_PROGRAM_COUNT {
            let running = CardLayout::build(&mut commands, layout, &format!("{machine_name}/running{running_index}"));
            commands.entity(running).observe_running(*machine_kind, running_index);
        }
    }

    for card_index in 0..HAND_SIZE {
        let built = CardLayout::build(&mut commands, layout, &format!("card{card_index}"));
        commands.entity(built).observe_hand_card(card_index);
    }

    for msg_index in 0..TTY_MESSAGE_COUNT {
        commands.entity(layout.entity(&format!("l_tty{msg_index}"))).insert(TTYMessageText::new(MachineKind::Local, msg_index));
        commands.entity(layout.entity(&format!("r_tty{msg_index}"))).insert(TTYMessageText::new(MachineKind::Remote, msg_index));
    }

    const NODES: &[(&str, MissionNodeKind)] = &[
        //
        ("node/a", MissionNodeKind::AccessPoint),
        ("node/b", MissionNodeKind::Backend),
        ("node/c", MissionNodeKind::Control),
        ("node/d", MissionNodeKind::Database),
        ("node/e", MissionNodeKind::Engine),
        ("node/f", MissionNodeKind::Frontend),
        ("node/g", MissionNodeKind::Gateway),
        ("node/h", MissionNodeKind::Hardware),
    ];
    let base_node = BaseNode::build_layout(&mut commands, layout, "node");
    let layouts = NODES.iter().map(|(name, kind)| (*kind, MissionNodeLayouts::build_layout(&mut commands, layout, name, *kind))).collect::<HashMap<_, _>>();
    let node_layouts = NodeLayouts {
        base_node,
        layouts,
    };
    commands.insert_resource(node_layouts);

    let tooltip = CardLayout::build(&mut commands, layout, "tooltip");
    let tooltip_id = commands.entity(tooltip).insert(Visibility::Hidden).observe(on_update_card_tooltip).id();
    commands.insert_resource(CardTooltip::new(tooltip_id));

    let context = GameplayContext {
        player_id: handoff.id.clone(),
        ..default()
    };
    commands.insert_resource(context);

    let initial_response = handoff.initial_response.take().unwrap();

    let local_name = handoff.name.clone();
    let player_id = handoff.id.clone();

    let current_node = initial_response.mission.current();
    let remote_name = current_node.kind.as_str();
    let remote_id = current_node.make_id();
    update_mission_info(&mut commands, remote_name, &remote_id, &player_id);

    recv_update_state(&mut commands, *initial_response);

    commands.trigger(MachineInfoTrigger::new(MachineKind::Local, local_name, handoff.id.clone()));

    commands.trigger(GamePhaseTrigger::new(VagabondGamePhase::Start));

    commands.remove_resource::<GameplayInitHandoff>();
}

fn on_click_next(_event: Trigger<Pointer<Click>>, mut context: ResMut<GameplayContext>, gate: Res<GateIFace>) {
    let wait = match context.phase {
        VagabondGamePhase::Start => gate.send_game_choose_intent(context.node_action.intent),
        VagabondGamePhase::Pick => gate.send_game_choose_attr(context.attr_pick),
        VagabondGamePhase::Play => gate.send_game_play_cards(&context.card_picks),
        VagabondGamePhase::Draw => gate.send_game_end_turn(),
        VagabondGamePhase::Wait(_) => false,
    };
    if wait {
        context.phase = VagabondGamePhase::Wait(WaitKind::One);
    }
}

fn on_over_next(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    context: Res<GameplayContext>,
    color_q: Query<&UiFxTrackedColor>,
) {
    if let Ok(source_color) = color_q.get(event.target) {
        let color = match context.phase {
            VagabondGamePhase::Pick => {
                if context.attr_pick.is_some() {
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::RED
                }
            }
            VagabondGamePhase::Wait(WaitKind::One) => bevy::color::palettes::basic::RED,
            VagabondGamePhase::Wait(WaitKind::All) => bevy::color::palettes::basic::YELLOW,
            _ => source_color.color,
        };
        commands.entity(event.target).trigger(SetColorEvent::new(event.target, color));
    }
}

fn on_click_attr(
    //
    event: Trigger<Pointer<Click>>,
    mut commands: Commands,
    attr_q: Query<&AttributeRow>,
) {
    if let Ok(attr_row) = attr_q.get(event.target) {
        commands.trigger(ChooseAttrTrigger::new(Some(attr_row.kind)));
    }
}

fn on_over_attr(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    context: Res<GameplayContext>,
    color_q: Query<&UiFxTrackedColor>,
) {
    if let Ok(source_color) = color_q.get(event.target) {
        let color = if VagabondGamePhase::Pick == context.phase {
            bevy::color::palettes::basic::GREEN
        } else {
            source_color.color
        };
        commands.entity(event.target).trigger(SetColorEvent::new(event.target, color));
    }
}

fn map_kind_to_index(kind: AttributeKind) -> usize {
    let kind: u8 = kind.into();
    kind as usize
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_card_drag_start(
    // bevy system
    event: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    sprite_q: Query<(&CardLayout, &mut Sprite, &mut Transform, &HandCard, Option<&IndicatorTracker>), With<PickingBehavior>>,
    bg_q: Query<(&UiFxTrackedColor, Option<&Blinker>), Without<CardLayout>>,
    mut indicator_q: Query<(Entity, &mut Indicator)>,
    mut context: ResMut<GameplayContext>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    if context.phase != VagabondGamePhase::Play {
        return;
    }

    let target = event.target;

    if let Ok((layout, sprite, transform, hand, tracker)) = sprite_q.get(target) {
        let card = context.hand.get(hand.index).cloned();
        if tracker.is_none() && card.as_ref().is_none_or(|card| card.cost > context.cached_state.erg[map_kind_to_index(card.kind)]) {
            if let Some(frame) = layout.frame {
                if let Ok((source_color, blink)) = bg_q.get(frame) {
                    if let Some(blink) = blink {
                        blink.remove(&mut commands, frame);
                    }
                    commands.entity(frame).insert(Blinker::new(source_color.color, bevy::color::palettes::basic::RED, BLINKER_COUNT, BLINKER_SPEED));
                }
            }
            return;
        }

        commands.entity(target).insert(PickingBehavior::IGNORE);

        if let Some(size) = sprite.custom_size {
            let translation = Vec3::new(transform.translation.x + (size.x / 2.0), transform.translation.y - (size.y / 2.0), INDICATOR_Z);
            let offset = Vec2::new(event.pointer_location.position.x - translation.x, -(event.pointer_location.position.y + translation.y));
            if tracker.is_none() {
                commands.spawn(Indicator::make_bundle(target, translation, offset, meshes, materials)).insert(IndicatorActive);
                commands.entity(target).insert(IndicatorTracker);
            } else if let Some((entity, mut indicator)) = indicator_q.iter_mut().find(|(_, i)| i.parent == target) {
                if let Some(card) = card {
                    context.cached_state.erg[map_kind_to_index(card.kind)] += card.cost;
                    commands.trigger(PlayerErgTrigger::new(context.cached_state.erg));
                }
                context.card_picks.remove(&(hand.index as CardIdxType));
                indicator.target = None;
                indicator.offset = offset;
                commands.entity(entity).insert(IndicatorActive);
            }
        }
    }
}

fn on_card_drag(
    // bevy system
    event: Trigger<Pointer<Drag>>,
    mut indicator_q: Query<(&mut Transform, &Indicator), With<IndicatorActive>>,
) {
    if let Ok((mut transform, indicator)) = indicator_q.get_single_mut() {
        let distance = Vec2::new(event.distance.x + indicator.offset.x, event.distance.y - indicator.offset.y);
        let length = distance.length();
        let angle = distance.x.atan2(distance.y);
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale = Vec3::new(1.0, length, 1.0);
        transform.translation.x = indicator.translation.x + (distance.x / 2.0);
        transform.translation.y = indicator.translation.y - (distance.y / 2.0);
    }
}

fn on_card_drag_end(
    // bevy system
    event: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    indicator_q: Query<Entity, With<IndicatorActive>>,
) {
    commands.entity(event.target).insert(PickingBehavior::default());
    if let Ok(entity) = indicator_q.get_single() {
        commands.entity(entity).remove::<IndicatorActive>();
    }
}

fn on_card_drop(
    // bevy system
    event: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    mut indicator_q: Query<&mut Indicator, With<IndicatorActive>>,
    mut drop_q: Query<&CardDropTarget>,
    hand_q: Query<&HandCard>,
    mut context: ResMut<GameplayContext>,
) {
    let dropped_on = event.target;

    if let Ok(mut indicator) = indicator_q.get_single_mut() {
        if let Ok(drop) = drop_q.get_mut(dropped_on) {
            if let Ok(hand) = hand_q.get(indicator.parent) {
                if let Some(card) = context.hand.get(hand.index).cloned() {
                    if valid_target(&card, drop.target) {
                        indicator.target = Some(drop.target);
                        context.add_card_pick(hand.index, drop.target);
                        context.cached_state.erg[map_kind_to_index(card.kind)] -= card.cost;
                        commands.trigger(PlayerErgTrigger::new(context.cached_state.erg));
                    }
                }
            }
        }
    }
}

fn valid_target(card: &VagabondCard, target: PickedCardTarget) -> bool {
    card.launch_rules.iter().any(|instruction| match instruction {
        LaunchInstruction::Targ(targ) => match targ {
            CardTargetValue::None => false,
            CardTargetValue::Machine(targ_machine) => match targ_machine {
                CardTargetMachineKind::Any => matches!(target, PickedCardTarget::MachineLocal | PickedCardTarget::MachineRemote),
                CardTargetMachineKind::Local => matches!(target, PickedCardTarget::MachineLocal),
                CardTargetMachineKind::Remote => matches!(target, PickedCardTarget::MachineRemote),
            },
            CardTargetValue::Actor => matches!(target, PickedCardTarget::Actor(_)),
        },
        _ => false,
    })
}

fn on_over_process(
    // bevy system
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    queue_q: Query<(&MachineKind, &MachineQueueItem)>,
    tooltip: Res<CardTooltip>,
    context: Res<GameplayContext>,
) {
    if let Ok((machine_kind, queue_item)) = queue_q.get(event.target) {
        let cached = match machine_kind {
            MachineKind::Local => &context.cached_local,
            MachineKind::Remote => &context.cached_remote,
        };
        if let Some(process) = cached.queue.iter().find(|(_, d)| queue_item.delay == *d).map(|(c, _)| c) {
            let card = Some(process.card.clone());
            let attributes = Attributes::from_arrays(process.attributes);
            commands.trigger_targets(UpdateCardTooltipEvent::new(event.pointer_location.position, card, attributes), tooltip.entity);
        }
    }
}

fn on_over_running(
    // bevy system
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    running_q: Query<(&MachineKind, &MachineRunning)>,
    tooltip: Res<CardTooltip>,
    context: Res<GameplayContext>,
) {
    if let Ok((machine_kind, running)) = running_q.get(event.target) {
        let cached = match machine_kind {
            MachineKind::Local => &context.cached_local,
            MachineKind::Remote => &context.cached_remote,
        };
        if let Some(process) = cached.running.get(running.index) {
            let card = Some(process.card.clone());
            let attributes = Attributes::from_arrays(process.attributes);
            commands.trigger_targets(UpdateCardTooltipEvent::new(event.pointer_location.position, card, attributes), tooltip.entity);
        }
    }
}

fn on_out_hide_card_tooltip(
    // bevy system
    _event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    tooltip: Res<CardTooltip>,
) {
    commands.entity(tooltip.entity).insert(Visibility::Hidden);
}

fn cleanup_indicator(commands: &mut Commands, indicator: Entity, parent: Entity) {
    commands.entity(indicator).despawn_recursive();
    commands.entity(parent).insert(PickingBehavior::default()).remove::<IndicatorTracker>();
}

fn cleanup_indicator_post_update(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<Pointer<DragEnd>>,
    indicator_q: Query<(Entity, &Indicator)>,
) {
    for event in receive.read() {
        if let Some((entity, indicator)) = indicator_q.iter().find(|(_, i)| i.parent == event.target) {
            if indicator.target.is_none() {
                cleanup_indicator(&mut commands, entity, indicator.parent);
            }
        }
    }
}

fn gameplay_update(
    // bevy system
    mut commands: Commands,
    mut gate: ResMut<GateIFace>,
    mut context: ResMut<GameplayContext>,
) {
    let new_phase = match gate.grx.try_recv() {
        Ok(GateCommand::GameChooseIntent(gate_response)) => recv_choose_intent(&mut commands, *gate_response),
        Ok(GateCommand::GameRoll(gate_response)) => recv_roll(&mut commands, *gate_response),
        Ok(GateCommand::GameChooseAttr(gate_response)) => recv_choose_attr(&mut commands, *gate_response),
        Ok(GateCommand::GameResources(gate_response)) => recv_resources(&mut commands, *gate_response),
        Ok(GateCommand::GamePlayCard(gate_response)) => recv_play_card(&mut commands, *gate_response),
        Ok(GateCommand::GameResolveCards(gate_response)) => recv_resolve_cards(&mut commands, *gate_response),
        Ok(GateCommand::GameEndTurn(gate_response)) => recv_end_turn(&mut commands, *gate_response),
        Ok(GateCommand::GameTick(gate_response)) => recv_tick(&mut commands, *gate_response, &mut context),
        Ok(GateCommand::GameEndGame(gate_response)) => recv_end_game(&mut commands, *gate_response),
        Ok(GateCommand::GameUpdateMission(gate_response)) => recv_update_mission(&mut commands, *gate_response, &context),
        Ok(GateCommand::GameUpdateTokens(gate_response)) => recv_update_tokens(&mut commands, *gate_response),
        Ok(GateCommand::GameUpdateState(gate_response)) => recv_update_state(&mut commands, *gate_response),
        Err(_) => None,
        Ok(GateCommand::Hello) => None,
        Ok(GateCommand::GameActivate(_)) => None,
        Ok(GateCommand::GameBuild(_)) => None,
        Ok(GateCommand::GameStartGame(_)) => None,
    };
    if let Some(phase) = new_phase {
        context.phase = phase;
        commands.trigger(GamePhaseTrigger::new(phase));
    }
}

fn recv_choose_intent(commands: &mut Commands, response: GameChooseIntentResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, "TURN STARTED"));
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_roll(commands: &mut Commands, response: GameRollMessage) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, "CHOOSE ATTR"));
    commands.trigger(RollTrigger::new(response.roll));
    commands.trigger(ChooseAttrTrigger::new(None));
    Some(VagabondGamePhase::Pick)
}

fn recv_choose_attr(commands: &mut Commands, response: GameChooseAttrResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, "ATTR CHOSEN"));
    if !response.success {
        commands.trigger(ChooseAttrTrigger::new(None));
    }

    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resources(commands: &mut Commands, response: GameResourcesMessage) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, "PLAY CARDS"));
    commands.trigger(PlayerErgTrigger::new(response.player_state_view.erg));
    commands.trigger(ResourcesTrigger::new(&response));
    commands.trigger(PlayerStateTrigger::new(response.player_state_view));
    Some(VagabondGamePhase::Play)
}

fn recv_play_card(commands: &mut Commands, response: GamePlayCardResponse) -> Option<VagabondGamePhase> {
    let success = response.success.iter().all(|&success| success);
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, "CARDS PLAYED"));
    success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resolve_cards(commands: &mut Commands, _response: GameResolveCardsMessage) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, "DRAW CARDS"));
    Some(VagabondGamePhase::Draw)
}

fn recv_end_turn(commands: &mut Commands, response: GameEndTurnResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, "END TURN"));
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_tick(commands: &mut Commands, response: GameTickMessage, context: &mut GameplayContext) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, "START TURN"));
    context.reset(response.tick);
    Some(VagabondGamePhase::Start)
}

fn recv_end_game(commands: &mut Commands, _response: GameEndGameResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, "END GAME"));
    None
}

fn update_mission_info(commands: &mut Commands, remote_name: &str, remote_id: &str, player_id: &str) {
    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, &format!("Connected to {remote_id}")));
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, &format!("Connection from {player_id}")));

    commands.trigger(MachineInfoTrigger::new(MachineKind::Remote, remote_name.to_string(), remote_id.to_string()));
}

fn recv_update_mission(commands: &mut Commands, response: GameUpdateMissionMessage, context: &GameplayContext) -> Option<VagabondGamePhase> {
    if response.new {
        let current_node = context.cached_mission.current();
        let remote_name = current_node.kind.as_str();
        let remote_id = current_node.make_id();

        update_mission_info(commands, remote_name, &remote_id, &context.player_id);
    }
    None
}

fn recv_update_tokens(commands: &mut Commands, response: GameUpdateTokensMessage) -> Option<VagabondGamePhase> {
    match response.token.kind {
        TokenKind::Invalid => {}
        TokenKind::Authorization(level) => {
            commands.trigger(TTYMessageTrigger::new(MachineKind::Local, &format!("{level} Authorization")));
            commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, &format!("Authorized {level}")));
        }
        TokenKind::Credentials(level) => {
            commands.trigger(TTYMessageTrigger::new(MachineKind::Local, &format!("{level} Credentials")));
        }
    }

    None
}

fn recv_update_state(commands: &mut Commands, response: GameUpdateStateResponse) -> Option<VagabondGamePhase> {
    commands.trigger(PlayerErgTrigger::new(response.player_state.erg));
    commands.trigger(PlayerStateTrigger::new(response.player_state));
    commands.trigger(MachineStateTrigger::new(response.local_machine, response.remote_machine));
    commands.trigger(MissionTrigger::new(response.mission));
    None
}

fn gameplay_exit(
    // bevy system
    mut commands: Commands,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    commands.remove_resource::<CardTooltip>();
    commands.remove_resource::<GameplayContext>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
