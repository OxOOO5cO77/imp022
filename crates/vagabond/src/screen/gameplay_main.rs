use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, VecDeque};

use bevy::prelude::*;

use hall::data::core::{AttributeKind, Attributes, DelayType, MissionNodeIdType, MissionNodeKind};
use hall::data::game::{GameMachinePlayerView, GameProcessPlayerView, TickType};
use hall::data::player::PlayerStatePlayerView;
use hall::message::*;
use vagabond::data::{VagabondCard, VagabondMachine, VagabondProcess};

use crate::manager::{AtlasManager, DataManager, ScreenLayoutManager, ScreenLayoutManagerParams};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::gameplay_init::GameplayInitHandoff;
use crate::screen::gameplay_main::events::*;
use crate::screen::shared::{on_out_generic, on_update_tooltip, replace_kind_icon, CardLayout, CardPopulateEvent, CardTooltip, GameMissionNodePlayerViewExt, KindIconSize, UpdateCardTooltipEvent};
use crate::system::ui_effects::{Blinker, Glower, SetColorEvent, TextTip, UiFxTrackedColor};
use crate::system::AppState;

mod events;

const SCREEN_LAYOUT: &str = "gameplay_main";

const BLINKER_COUNT: f32 = 2.0;
const BLINKER_SPEED: f32 = 24.0;
const GLOWER_SPEED: f32 = 4.0;

const PROCESS_QUEUE_SIZE: DelayType = 10;
const HAND_SIZE: usize = 5;
const RUNNING_PROGRAM_COUNT: usize = 6;
const TTY_MESSAGE_COUNT: usize = 9;

pub struct GameplayMainPlugin;

impl Plugin for GameplayMainPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, gameplay_update.run_if(in_state(AppState::Gameplay)))
            .add_systems(PostUpdate, cleanup_indicator_post_update.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit);
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

#[derive(Resource)]
struct GameplayContext {
    tick: TickType,
    phase: VagabondGamePhase,
    attr_pick: Option<AttributeKind>,
    card_picks: HashMap<CardIdxType, CardTarget>,
    current_remote: MissionNodeIdType,
    hand: Vec<VagabondCard>,
    tty: HashMap<MachineKind, VecDeque<String>>,
    cached_state: PlayerStatePlayerView,
    cached_local: VagabondMachine,
    cached_remote: VagabondMachine,
}

impl Default for GameplayContext {
    fn default() -> Self {
        Self {
            tick: Default::default(),
            phase: Default::default(),
            attr_pick: None,
            card_picks: Default::default(),
            current_remote: 1,
            hand: Default::default(),
            tty: Default::default(),
            cached_state: Default::default(),
            cached_local: Default::default(),
            cached_remote: Default::default(),
        }
    }
}

impl GameplayContext {
    fn reset(&mut self, tick: TickType) {
        self.attr_pick = None;
        self.card_picks.clear();
        self.tick = tick;
    }

    fn add_card_pick(&mut self, index: usize, target: MachineKind) {
        let card_idx = index as CardIdxType;
        let card_target = match target {
            MachineKind::Local => CardTarget::Local,
            MachineKind::Remote => CardTarget::Remote(self.current_remote),
        };
        self.card_picks.insert(card_idx, card_target);
    }
}

#[derive(Component)]
struct PhaseIcon {
    phase: VagabondGamePhase,
}

impl PhaseIcon {
    fn new(phase: VagabondGamePhase) -> Self {
        Self {
            phase,
        }
    }
}

#[derive(Component)]
struct RemoteAttrText(usize);

#[derive(Component)]
struct RemoteAttrIcon;

#[derive(Component)]
struct RollText(usize);

#[derive(Component)]
enum PlayerStateText {
    Attribute(usize, usize),
    Erg(usize),
    Deck,
    Heap,
}

#[derive(Component)]
struct MachineQueueItem {
    delay: DelayType,
}

impl MachineQueueItem {
    fn new(delay: DelayType) -> Self {
        Self {
            delay,
        }
    }
}

#[derive(Component)]
enum MachineTextKind {
    Title,
    Id,
    Vitals(usize),
}

#[derive(Component)]
struct MachineText(MachineTextKind);

#[derive(Component)]
struct MachineRunning {
    index: usize,
}

impl MachineRunning {
    fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

#[derive(Component, Copy, Clone, Hash, Eq, PartialEq)]
enum MachineKind {
    Local,
    Remote,
}

struct MachineInfo {
    name: String,
    id: String,
}

#[derive(Component)]
struct MissionNodeDisplay {
    kind: MissionNodeKind,
}

impl MissionNodeDisplay {
    fn new(kind: MissionNodeKind) -> Self {
        Self {
            kind,
        }
    }
}

#[derive(Component)]
struct TTYMessageText {
    kind: MachineKind,
    slot: usize,
}

impl TTYMessageText {
    fn new(kind: MachineKind, slot: usize) -> Self {
        Self {
            kind,
            slot,
        }
    }
}

#[derive(Component)]
struct AttributeRow(AttributeKind);

#[derive(Component)]
struct HandCard {
    index: usize,
}

impl HandCard {
    fn new(slot: usize) -> Self {
        Self {
            index: slot,
        }
    }
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
            .insert((AttributeRow(kind), PickingBehavior::default()))
            .observe(on_click_attr)
            .observe(on_over_attr)
            .observe(on_out_generic)
    }
    fn observe_next_button(self) -> Self {
        self //
            .insert(PickingBehavior::default())
            .observe(on_click_next)
            .observe(on_over_next)
            .observe(on_out_generic)
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
            .observe(on_out_process)
    }
    fn observe_running(self, kind: MachineKind, running_index: usize) -> Self {
        self //
            .insert((kind, MachineRunning::new(running_index), PickingBehavior::default()))
            .observe(on_over_process)
            .observe(on_out_process)
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
    let (layout, base_id) = slm.build(&mut commands, SCREEN_LAYOUT, &am, &mut slm_params);

    // spawn observers
    commands.entity(base_id).with_children(|parent| {
        parent.spawn(Observer::new(on_tty_update));
        parent.spawn(Observer::new(on_tty_update));
        parent.spawn(Observer::new(on_roll_ui_update_roll));
        parent.spawn(Observer::new(on_roll_ui_update_resources));
        parent.spawn(Observer::new(on_indicator_ui_update));
        parent.spawn(Observer::new(on_hand_ui_update));
        parent.spawn(Observer::new(on_erg_ui_update));
        parent.spawn(Observer::new(on_phase_ui_update));
        parent.spawn(Observer::new(on_local_state_update_player));
        parent.spawn(Observer::new(on_mission_ui_update));
        parent.spawn(Observer::new(on_local_ui_update_attr));
        parent.spawn(Observer::new(on_local_ui_update_player));
        parent.spawn(Observer::new(on_remote_ui_update_roll));
        parent.spawn(Observer::new(on_remote_ui_update_resources));
        parent.spawn(Observer::new(on_machine_ui_update_info));
        parent.spawn(Observer::new(on_machine_ui_update_state));
    });

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
        commands.entity(layout.entity(roll)).insert(RollText(roll_idx));
    }

    const REMOTE_ATTR: &[&str] = &["ra", "rb", "rc", "rd"];

    for (remote_idx, remote) in REMOTE_ATTR.iter().enumerate() {
        commands.entity(layout.entity(remote)).insert(RemoteAttrText(remote_idx));
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

    const MACHINES: &[(&str, MachineKind)] = &[("local", MachineKind::Local), ("remote", MachineKind::Remote)];

    for (machine_name, machine_kind) in MACHINES {
        commands.entity(layout.entity(machine_name)).insert((*machine_kind, PickingBehavior::default())).observe(on_card_drop);

        commands.entity(layout.entity(&format!("{}/title", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Title)));
        commands.entity(layout.entity(&format!("{}/id", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Id)));

        commands.entity(layout.entity(&format!("{}/free_space", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Vitals(0))));
        commands.entity(layout.entity(&format!("{}/thermal_capacity", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Vitals(1))));
        commands.entity(layout.entity(&format!("{}/system_health", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Vitals(2))));
        commands.entity(layout.entity(&format!("{}/open_ports", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Vitals(3))));

        for queue_index in 0..PROCESS_QUEUE_SIZE {
            commands.entity(layout.entity(&format!("{}/queue{}", machine_name, queue_index))).observe_process(*machine_kind, queue_index);
        }

        for running_index in 0..RUNNING_PROGRAM_COUNT {
            let running = CardLayout::build(&mut commands, layout, &format!("{}/running{}", machine_name, running_index));
            commands.entity(running).observe_running(*machine_kind, running_index);
        }
    }

    for card_index in 0..HAND_SIZE {
        let built = CardLayout::build(&mut commands, layout, &format!("card{}", card_index));
        commands.entity(built).observe_hand_card(card_index);
    }

    for msg_index in 0..TTY_MESSAGE_COUNT {
        commands.entity(layout.entity(&format!("l_tty{}", msg_index))).insert(TTYMessageText::new(MachineKind::Local, msg_index));
        commands.entity(layout.entity(&format!("r_tty{}", msg_index))).insert(TTYMessageText::new(MachineKind::Remote, msg_index));
    }

    const NODES: &[(MissionNodeKind, &str)] = &[
        //
        (MissionNodeKind::AccessPoint, "node_a"),
        (MissionNodeKind::Backend, "node_b"),
    ];

    for (kind, node) in NODES {
        commands.entity(layout.entity(node)).insert((MissionNodeDisplay::new(*kind), Visibility::Hidden));
    }

    let tooltip = CardLayout::build(&mut commands, layout, "tooltip");
    let tooltip_id = commands.entity(tooltip).insert(Visibility::Hidden).observe(on_update_tooltip).id();
    commands.insert_resource(CardTooltip::new(tooltip_id));

    commands.remove_resource::<GameplayInitHandoff>();
    commands.insert_resource(GameplayContext::default());

    let initial_response = handoff.initial_response.take().unwrap();
    let local_info = MachineInfo {
        name: handoff.name.clone(),
        id: handoff.id.clone(),
    };
    let remote_info = MachineInfo {
        name: initial_response.mission.node.as_str().to_string(),
        id: initial_response.mission.node.make_id(),
    };
    recv_update_state(&mut commands, *initial_response);

    commands.trigger(TTYMessageTrigger::new(MachineKind::Local, &format!("Connected to {}", remote_info.id)));
    commands.trigger(TTYMessageTrigger::new(MachineKind::Remote, &format!("Connection from {}", handoff.id)));

    commands.trigger(MachineInfoTrigger::new(MachineKind::Local, local_info));
    commands.trigger(MachineInfoTrigger::new(MachineKind::Remote, remote_info));

    commands.trigger(GamePhaseTrigger::new(VagabondGamePhase::Start));
}

fn on_click_next(_event: Trigger<Pointer<Click>>, mut context: ResMut<GameplayContext>, gate: Res<GateIFace>) {
    let wait = match context.phase {
        VagabondGamePhase::Start => gate.send_game_start_turn(),
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
    if let Ok(AttributeRow(kind)) = attr_q.get(event.target) {
        commands.trigger(ChooseAttrTrigger::new(Some(*kind)));
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

#[derive(Component)]
struct Indicator {
    translation: Vec3,
    offset: Vec2,
    parent: Entity,
    target: Option<MachineKind>,
}

#[derive(Component)]
struct IndicatorTracker;

#[derive(Component)]
struct IndicatorActive;

fn make_indicator_bundle(parent: Entity, translation: Vec3, offset: Vec2, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> impl Bundle {
    (
        Indicator {
            translation,
            offset,
            parent,
            target: None,
        },
        Mesh2d(meshes.add(Rectangle::new(16.0, 1.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::Srgba(Srgba::new(0.0, 0.75, 0.0, 0.35))))),
        Transform::from_translation(translation),
        PickingBehavior::IGNORE,
    )
}

fn on_indicator_ui_update(
    // bevy system
    event: Trigger<GamePhaseTrigger>,
    mut commands: Commands,
    indicator_q: Query<(Entity, &Indicator)>,
) {
    match event.phase {
        VagabondGamePhase::Start => {}
        VagabondGamePhase::Play => {}
        VagabondGamePhase::Draw => indicator_q.iter().for_each(|(e, i)| cleanup_indicator(&mut commands, e, i.parent)),
        _ => {}
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
                commands.spawn(make_indicator_bundle(target, translation, offset, meshes, materials)).insert(IndicatorActive);
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
    mut machine_q: Query<&MachineKind>,
    hand_q: Query<&HandCard>,
    mut context: ResMut<GameplayContext>,
) {
    let dropped_on = event.target;

    if let Ok(mut indicator) = indicator_q.get_single_mut() {
        indicator.target = machine_q.get_mut(dropped_on).ok().copied();
        if let Some(target) = indicator.target {
            if let Ok(hand) = hand_q.get(indicator.parent) {
                context.add_card_pick(hand.index, target);
                if let Some((kind, cost)) = context.hand.get_mut(hand.index).map(|card| (card.kind, card.cost)) {
                    context.cached_state.erg[map_kind_to_index(kind)] -= cost;
                    commands.trigger(PlayerErgTrigger::new(context.cached_state.erg));
                }
            }
        }
    }
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
        let card = cached.queue.iter().find(|(_, d)| queue_item.delay == *d).map(|(c, _)| c.card.clone());
        commands.trigger_targets(UpdateCardTooltipEvent::new(event.pointer_location.position, card, Attributes::from_arrays(context.cached_state.attr)), tooltip.entity);
    }
}

fn on_out_process(
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

fn on_roll_ui_update_roll(
    // bevy system
    event: Trigger<RollTrigger>,
    mut roll_q: Query<(&mut Text2d, &mut TextColor, &RollText)>,
) {
    for (mut text, mut color, RollText(index)) in roll_q.iter_mut() {
        *text = format!("{}", event.roll[*index]).into();
        *color = bevy::color::palettes::basic::GRAY.into();
    }
}

fn on_roll_ui_update_resources(
    // bevy system
    event: Trigger<ResourcesTrigger>,
    mut roll_q: Query<(&mut TextColor, &RollText)>,
) {
    for (mut color, RollText(index)) in roll_q.iter_mut() {
        *color = match event.local_erg[*index].cmp(&event.remote_erg[*index]) {
            Ordering::Less => bevy::color::palettes::basic::RED,
            Ordering::Equal => bevy::color::palettes::basic::YELLOW,
            Ordering::Greater => bevy::color::palettes::basic::GREEN,
        }
        .into();
    }
}

fn on_hand_ui_update(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut commands: Commands,
    hand_q: Query<(Entity, &HandCard)>,
    dm: Res<DataManager>,
    context: Res<GameplayContext>,
) {
    for (entity, hand) in &hand_q {
        let card = event.state.hand.get(hand.index).and_then(|o| dm.convert_card(o));
        commands.entity(entity).trigger(CardPopulateEvent::new(card, Attributes::from_arrays(context.cached_state.attr)));
    }
}

fn on_erg_ui_update(
    // bevy system
    event: Trigger<PlayerErgTrigger>,
    mut erg_q: Query<(&mut Text2d, &PlayerStateText)>,
) {
    for (mut erg_text, state_text) in erg_q.iter_mut() {
        if let PlayerStateText::Erg(index) = state_text {
            *erg_text = format!("{}", event.erg[*index]).into();
        }
    }
}

fn on_phase_ui_update(
    // bevy system
    event: Trigger<GamePhaseTrigger>,
    mut sprite_q: Query<(&mut Sprite, &PhaseIcon)>,
) {
    for (mut sprite, icon) in sprite_q.iter_mut() {
        let color = if event.phase == icon.phase {
            bevy::color::palettes::css::CHARTREUSE
        } else {
            Srgba::new(0.2, 0.2, 0.2, 1.0)
        };
        sprite.color = color.into();
    }
}

fn convert_process(process: &GameProcessPlayerView, dm: &DataManager) -> Option<VagabondProcess> {
    let vagabond_process = VagabondProcess {
        card: dm.convert_card(&process.player_card)?,
        priority: process.priority,
        local: process.local,
    };
    Some(vagabond_process)
}

fn cache_game_machine(machine: &GameMachinePlayerView, dm: &DataManager) -> VagabondMachine {
    VagabondMachine {
        vitals: machine.vitals,
        queue: machine.queue.iter().filter_map(|(process, delay)| convert_process(process, dm).map(|p| (p, *delay))).collect(),
        running: machine.running.iter().filter_map(|p| convert_process(p, dm)).collect(),
    }
}

fn on_local_state_update_player(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut context: ResMut<GameplayContext>,
    dm: Res<DataManager>,
) {
    context.cached_state = event.state.clone();
    context.hand = event.state.hand.iter().filter_map(|card| dm.convert_card(card)).collect();
}

fn on_local_ui_update_player(
    // bevy system
    event: Trigger<PlayerStateTrigger>,
    mut text_q: Query<(&mut Text2d, &PlayerStateText)>,
) {
    for (mut text, state_text) in text_q.iter_mut() {
        match state_text {
            PlayerStateText::Attribute(row, col) => *text = format!("{}", event.state.attr[*row][*col]).into(),
            PlayerStateText::Deck => *text = event.state.deck.to_string().into(),
            PlayerStateText::Heap => *text = event.state.heap.len().to_string().into(),
            _ => {}
        }
    }
}

fn on_local_ui_update_attr(
    // bevy system
    event: Trigger<ChooseAttrTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut TextColor, &PlayerStateText)>,
    mut row_q: Query<(Entity, &UiFxTrackedColor, Option<&Glower>), With<AttributeRow>>,
    mut context: ResMut<GameplayContext>,
) {
    if context.phase != VagabondGamePhase::Pick {
        return;
    }

    if event.kind.is_none() {
        for (entity, source_color, _) in row_q.iter() {
            let color = source_color.color;
            commands.entity(entity).insert(Glower::new(color, Srgba::new(0.0, 1.0, 0.0, 1.0), GLOWER_SPEED));
        }
    } else {
        for (entity, _, glower) in row_q.iter_mut() {
            if let Some(glower) = glower {
                glower.remove(&mut commands, entity);
            }
        }
    }

    for (mut color, state_text) in text_q.iter_mut() {
        if let PlayerStateText::Attribute(row, _) = state_text {
            *color = if let Some(kind) = event.kind {
                if *row == map_kind_to_index(kind) {
                    context.attr_pick = Some(kind);
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::GRAY
                }
            } else {
                bevy::color::palettes::basic::GRAY
            }
            .into()
        }
    }
}

fn on_remote_ui_update_roll(
    // bevy system
    _event: Trigger<RollTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut Text2d, &mut TextColor, &RemoteAttrText)>,
    icon_q: Query<Entity, With<RemoteAttrIcon>>,
) {
    for (mut attr_text, mut color, RemoteAttrText(_)) in text_q.iter_mut() {
        *attr_text = "-".into();
        *color = bevy::color::palettes::basic::GRAY.into();
    }
    if let Ok(entity) = icon_q.get_single() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn on_remote_ui_update_resources(
    // bevy system
    event: Trigger<ResourcesTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&mut Text2d, &mut TextColor, &RemoteAttrText)>,
    mut icon_q: Query<(Entity, &mut Sprite), With<RemoteAttrIcon>>,
    am: Res<AtlasManager>,
) {
    for (mut attr_text, mut color, RemoteAttrText(index)) in text_q.iter_mut() {
        *attr_text = event.remote_attr[*index].to_string().into();
        *color = bevy::color::palettes::basic::RED.into();
    }
    if let Ok((entity, mut sprite)) = icon_q.get_single_mut() {
        replace_kind_icon(&mut sprite, event.remote_kind, KindIconSize::Large, &am);
        commands.entity(entity).insert(Visibility::Visible);
    }
}

fn on_machine_ui_update_info(
    // bevy system
    event: Trigger<MachineInfoTrigger>,
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
) {
    for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
        if *machine_component == event.kind {
            match kind {
                MachineTextKind::Title => *text = event.info.name.to_string().into(),
                MachineTextKind::Id => *text = event.info.id.to_string().into(),
                MachineTextKind::Vitals(_) => {}
            }
        }
    }
}

fn on_machine_ui_update_state(
    // bevy system
    event: Trigger<MachineStateTrigger>,
    mut commands: Commands,
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
    mut sprite_q: Query<(&MachineKind, &mut Sprite, &MachineQueueItem)>,
    running_q: Query<(Entity, &MachineKind, &MachineRunning)>,
    dm: Res<DataManager>,
    mut context: ResMut<GameplayContext>,
) {
    context.cached_local = cache_game_machine(&event.local, &dm);
    context.cached_remote = cache_game_machine(&event.remote, &dm);

    for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
        if let MachineTextKind::Vitals(index) = kind {
            let machine = if *machine_component == MachineKind::Local {
                &event.local
            } else {
                &event.remote
            };
            *text = machine.vitals[*index].to_string().into();
        }
    }

    for (machine_component, mut sprite, queue_item) in sprite_q.iter_mut() {
        let (machine, player_owned) = if *machine_component == MachineKind::Local {
            (&event.local, true)
        } else {
            (&event.remote, false)
        };

        sprite.color = if let Some(process) = machine.queue.iter().find(|(_, delay)| *delay == queue_item.delay).map(|(item, _)| item) {
            if process.local == player_owned {
                bevy::color::palettes::basic::GREEN
            } else {
                bevy::color::palettes::basic::RED
            }
        } else {
            bevy::color::palettes::basic::WHITE
        }
        .into();
    }

    for (entity, machine_component, running) in running_q.iter() {
        let machine = if *machine_component == MachineKind::Local {
            &event.local
        } else {
            &event.remote
        };
        let card = machine.running.get(running.index).and_then(|process| dm.convert_card(&process.player_card));
        commands.entity(entity).trigger(CardPopulateEvent::new(card, Attributes::from_arrays(context.cached_state.attr)));
    }
}

fn on_mission_ui_update(
    // bevy system
    event: Trigger<MissionTrigger>,
    mut commands: Commands,
    display_q: Query<(Entity, &MissionNodeDisplay)>,
) {
    for (entity, display) in &display_q {
        let visibility = if event.mission.node.kind == display.kind {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        commands.entity(entity).insert(visibility);
    }
}

fn on_tty_update(
    // bevy system
    event: Trigger<TTYMessageTrigger>,
    mut context: ResMut<GameplayContext>,
    mut text_q: Query<(&mut Text2d, &mut TTYMessageText)>,
) {
    let message = format!("[{:03}] {}", context.tick, event.message);
    let queue = context.tty.entry(event.kind).or_default();
    queue.push_front(message);
    if queue.len() > TTY_MESSAGE_COUNT {
        queue.pop_back();
    }

    for (mut text, tty) in text_q.iter_mut() {
        let queue = context.tty.entry(tty.kind).or_default();
        if let Some(message) = queue.get(tty.slot) {
            *text = message.clone().into();
        } else {
            *text = String::default().into();
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
        Ok(GateCommand::GameStartTurn(gate_response)) => recv_start_turn(&mut commands, *gate_response),
        Ok(GateCommand::GameRoll(gate_response)) => recv_roll(&mut commands, *gate_response),
        Ok(GateCommand::GameChooseAttr(gate_response)) => recv_choose_attr(&mut commands, *gate_response),
        Ok(GateCommand::GameResources(gate_response)) => recv_resources(&mut commands, *gate_response),
        Ok(GateCommand::GamePlayCard(gate_response)) => recv_play_card(&mut commands, *gate_response),
        Ok(GateCommand::GameResolveCards(gate_response)) => recv_resolve_cards(&mut commands, *gate_response),
        Ok(GateCommand::GameEndTurn(gate_response)) => recv_end_turn(&mut commands, *gate_response),
        Ok(GateCommand::GameTick(gate_response)) => recv_tick(&mut commands, *gate_response, &mut context),
        Ok(GateCommand::GameEndGame(gate_response)) => recv_end_game(&mut commands, *gate_response),
        Ok(GateCommand::GameUpdateState(gate_response)) => recv_update_state(&mut commands, *gate_response),
        Ok(_) => None,
        Err(_) => None,
    };
    if let Some(phase) = new_phase {
        context.phase = phase;
        commands.trigger(GamePhaseTrigger::new(phase));
    }
}

fn recv_start_turn(commands: &mut Commands, response: GameStartTurnResponse) -> Option<VagabondGamePhase> {
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
