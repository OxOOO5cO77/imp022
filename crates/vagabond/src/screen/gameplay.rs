use crate::gfx::FrameMaterial;
use crate::manager::{AtlasManager, DataManager, ScreenLayoutManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::card_layout::{CardLayout, CardPopulateEvent};
use crate::screen::card_tooltip::{on_update_tooltip, CardTooltip, UpdateCardTooltipEvent};
use crate::screen::compose::ComposeHandoff;
use crate::screen::util::GameMissionNodePlayerViewExt;
use crate::system::ui_effects::{Blinker, Glower};
use crate::system::AppState;
use bevy::prelude::*;
use hall::data::game::{GameMachinePlayerView, GameProcessPlayerView, TickType};
use hall::data::player::PlayerStatePlayerView;
use hall::message::*;
use shared_data::attribute::AttributeKind;
use shared_data::build::BuildValueType;
use shared_data::card::{DelayType, ErgType};
use shared_data::mission::MissionNodeIdType;
use std::cmp::{Ordering, PartialEq};
use std::collections::{HashMap, VecDeque};
use vagabond::data::{VagabondCard, VagabondMachine, VagabondProcess};

const SCREEN_LAYOUT: &str = "gameplay";

const BLINKER_COUNT: f32 = 2.0;
const BLINKER_SPEED: f32 = 24.0;
const GLOWER_SPEED: f32 = 4.0;

const PROCESS_QUEUE_SIZE: DelayType = 10;
const HAND_SIZE: usize = 5;
const RUNNING_PROGRAM_COUNT: usize = 6;
const TTY_MESSAGE_COUNT: usize = 9;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::GameplayInit), gameplay_init_enter)
            .add_systems(Update, gameplay_init_update.run_if(in_state(AppState::GameplayInit)))
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, (gameplay_update, erg_ui_update, phase_ui_update, hand_ui_update, indicator_ui_update, local_state_update, local_ui_update, roll_ui_update, remote_ui_update, machine_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(PostUpdate, cleanup_indicator_post_update.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit)
            .add_observer(tty_update);
    }
}

#[derive(Resource)]
struct GameplayInitHandoff {
    initial_response: Option<Box<GameUpdateStateResponse>>,
    name: String,
    id: String,
}

fn gameplay_init_enter(
    // bevy system
    gate: ResMut<GateIFace>,
) {
    gate.send_game_update_state();
}

fn gameplay_init_update(
    //
    mut commands: Commands,
    mut gate: ResMut<GateIFace>,
    mut app_state: ResMut<NextState<AppState>>,
    gameplay_handoff: Res<ComposeHandoff>,
) {
    if let Ok(GateCommand::GameUpdateState(gate_response)) = gate.grx.try_recv() {
        let handoff = GameplayInitHandoff {
            initial_response: Some(gate_response),
            name: gameplay_handoff.local_name.clone(),
            id: gameplay_handoff.local_id.clone(),
        };
        commands.insert_resource(handoff);
        commands.remove_resource::<ComposeHandoff>();
        app_state.set(AppState::Gameplay)
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
    attr_pick: Option<AttrKind>,
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
struct PhaseIcon(VagabondGamePhase);

#[derive(Component)]
struct RemoteAttrText(usize);

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
struct MachineQueueItem(DelayType);

#[derive(Component)]
enum MachineTextKind {
    Title,
    Id,
    Vitals(usize),
}

#[derive(Component)]
struct MachineText(MachineTextKind);

#[derive(Component)]
struct MachineRunning(usize);

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

#[derive(Event)]
struct TTYMessageEvent {
    kind: MachineKind,
    message: String,
}

impl TTYMessageEvent {
    fn new(kind: MachineKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}

#[derive(Event)]
enum UiEvent {
    GamePhase(VagabondGamePhase),
    PlayerState(PlayerStatePlayerView),
    ChooseAttr(Option<AttrKind>),
    Roll([ErgType; 4]),
    PlayerErg([ErgType; 4]),
    Resources([ErgType; 4], [ErgType; 4], [BuildValueType; 4]),
    MachineInfoUpdate(MachineKind, MachineInfo),
    MachineStateUpdate(GameMachinePlayerView, GameMachinePlayerView),
}

#[derive(Component)]
struct AttributeRow(AttrKind);

#[derive(Component)]
struct HandCard(usize);

trait PickableEntityCommandsExtension {
    fn observe_pickable_row(self, kind: AttrKind) -> Self;
    fn observe_next_button(self) -> Self;
    fn observe_hand_card(self, hand_index: usize) -> Self;
    fn observe_process(self, kind: MachineKind, queue_index: DelayType) -> Self;
    fn observe_running(self, kind: MachineKind, running_index: usize) -> Self;
}

impl PickableEntityCommandsExtension for &mut EntityCommands<'_> {
    fn observe_pickable_row(self, kind: AttrKind) -> Self {
        self //
            .insert((AttributeRow(kind), PickingBehavior::default()))
            .observe(on_click_attr)
            .observe(on_over_attr)
            .observe(on_out_attr)
    }
    fn observe_next_button(self) -> Self {
        self //
            .insert(PickingBehavior::default())
            .observe(on_click_next)
            .observe(on_over_next)
            .observe(on_out_next)
    }
    fn observe_hand_card(self, hand_index: usize) -> Self {
        self //
            .insert((HandCard(hand_index), PickingBehavior::default()))
            .observe(on_card_drag_start)
            .observe(on_card_drag)
            .observe(on_card_drag_end)
    }
    fn observe_process(self, kind: MachineKind, queue_index: DelayType) -> Self {
        self //
            .insert((kind, MachineQueueItem(queue_index), PickingBehavior::default()))
            .observe(on_over_process)
            .observe(on_out_process)
    }
    fn observe_running(self, kind: MachineKind, running_index: usize) -> Self {
        self //
            .insert((kind, MachineRunning(running_index), PickingBehavior::default()))
            .observe(on_over_process)
            .observe(on_out_process)
    }
}

#[allow(clippy::type_complexity)]
fn gameplay_enter(
    // bevy system
    mut commands: Commands,
    mut handoff: ResMut<GameplayInitHandoff>,
    mut send: EventWriter<UiEvent>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    for_slm: (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>, ResMut<Assets<FrameMaterial>>),
) {
    let layout = slm.build(&mut commands, SCREEN_LAYOUT, &am, for_slm);

    const LOCAL_ATTR: [[&str; 4]; 4] = [
        //
        ["attributes/aa", "attributes/ab", "attributes/ac", "attributes/ad"],
        ["attributes/ba", "attributes/bb", "attributes/bc", "attributes/bd"],
        ["attributes/ca", "attributes/cb", "attributes/cc", "attributes/cd"],
        ["attributes/da", "attributes/db", "attributes/dc", "attributes/dd"],
    ];

    for (row_idx, row) in LOCAL_ATTR.iter().enumerate() {
        for (col_idx, name) in row.iter().enumerate() {
            commands.entity(layout.entity(name)).insert(PlayerStateText::Attribute(row_idx, col_idx));
        }
    }

    const ROLL: [&str; 4] = ["ea", "eb", "ec", "ed"];

    for (roll_idx, roll) in ROLL.iter().enumerate() {
        commands.entity(layout.entity(roll)).insert(RollText(roll_idx));
    }

    const REMOTE_ATTR: [&str; 4] = ["ra", "rb", "rc", "rd"];

    for (remote_idx, remote) in REMOTE_ATTR.iter().enumerate() {
        commands.entity(layout.entity(remote)).insert(RemoteAttrText(remote_idx));
    }

    const ERG: [&str; 4] = ["la", "lb", "lc", "ld"];

    for (erg_idx, erg) in ERG.iter().enumerate() {
        commands.entity(layout.entity(erg)).insert(PlayerStateText::Erg(erg_idx));
    }

    commands.entity(layout.entity("deck")).insert(PlayerStateText::Deck);
    commands.entity(layout.entity("heap")).insert(PlayerStateText::Heap);

    commands.entity(layout.entity("phase_start")).insert(PhaseIcon(VagabondGamePhase::Start));
    commands.entity(layout.entity("phase_pick")).insert(PhaseIcon(VagabondGamePhase::Pick));
    commands.entity(layout.entity("phase_play")).insert(PhaseIcon(VagabondGamePhase::Play));
    commands.entity(layout.entity("phase_draw")).insert(PhaseIcon(VagabondGamePhase::Draw));

    commands.entity(layout.entity("phase_bg")).observe_next_button();

    commands.entity(layout.entity("attributes/row_a")).observe_pickable_row(AttrKind::Analyze);
    commands.entity(layout.entity("attributes/row_b")).observe_pickable_row(AttrKind::Breach);
    commands.entity(layout.entity("attributes/row_c")).observe_pickable_row(AttrKind::Compute);
    commands.entity(layout.entity("attributes/row_d")).observe_pickable_row(AttrKind::Disrupt);

    const MACHINES: [(&str, MachineKind); 2] = [("local", MachineKind::Local), ("remote", MachineKind::Remote)];

    for (machine_name, machine_kind) in &MACHINES {
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

    let tooltip = CardLayout::build(&mut commands, layout, "tooltip");
    let tooltip_id = commands.entity(tooltip).insert(Visibility::Hidden).observe(on_update_tooltip).id();
    commands.insert_resource(CardTooltip(tooltip_id));

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
    recv_update_state(*initial_response, &mut send);

    commands.trigger(TTYMessageEvent::new(MachineKind::Local, &format!("Connected to {}", remote_info.id)));
    commands.trigger(TTYMessageEvent::new(MachineKind::Remote, &format!("Connection from {}", handoff.id)));

    send.send(UiEvent::MachineInfoUpdate(MachineKind::Local, local_info));
    send.send(UiEvent::MachineInfoUpdate(MachineKind::Remote, remote_info));

    send.send(UiEvent::GamePhase(VagabondGamePhase::Start));
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

fn on_over_next(event: Trigger<Pointer<Over>>, context: Res<GameplayContext>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = match context.phase {
            VagabondGamePhase::Pick => {
                if context.attr_pick.is_some() {
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::RED
                }
            }
            VagabondGamePhase::Wait(WaitKind::One) => bevy::color::palettes::basic::RED,
            VagabondGamePhase::Wait(WaitKind::All) => bevy::color::palettes::basic::YELLOW,
            _ => bevy::color::palettes::basic::GREEN,
        }
        .into();
    }
}

fn on_out_next(event: Trigger<Pointer<Out>>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = bevy::color::palettes::css::DARK_GRAY.into();
    }
}

fn on_click_attr(event: Trigger<Pointer<Click>>, mut send: EventWriter<UiEvent>, attr_q: Query<&AttributeRow>) {
    if let Ok(AttributeRow(kind)) = attr_q.get(event.target) {
        send.send(UiEvent::ChooseAttr(Some(*kind)));
    }
}

fn on_over_attr(event: Trigger<Pointer<Over>>, context: Res<GameplayContext>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = if VagabondGamePhase::Pick == context.phase {
            bevy::color::palettes::basic::GREEN
        } else {
            bevy::color::palettes::css::SILVER
        }
        .into();
    }
}

fn on_out_attr(event: Trigger<Pointer<Out>>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = bevy::color::palettes::css::DARK_GRAY.into();
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
    )
}

fn indicator_ui_update(mut commands: Commands, mut receive: EventReader<UiEvent>, indicator_q: Query<(Entity, &Indicator)>) {
    for ui_event in receive.read() {
        if let UiEvent::GamePhase(phase) = ui_event {
            match phase {
                VagabondGamePhase::Start => {}
                VagabondGamePhase::Play => {}
                VagabondGamePhase::Draw => indicator_q.iter().for_each(|(e, i)| cleanup_indicator(&mut commands, e, i.parent)),
                _ => {}
            }
        }
    }
}

fn kind_to_erg_index(kind: AttributeKind) -> usize {
    match kind {
        AttributeKind::Analyze => 0,
        AttributeKind::Breach => 1,
        AttributeKind::Compute => 2,
        AttributeKind::Disrupt => 3,
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_card_drag_start(
    // bevy system
    event: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    sprite_q: Query<(&CardLayout, &mut Sprite, &mut Transform, &HandCard, Option<&IndicatorTracker>), With<PickingBehavior>>,
    mut bg_q: Query<(&mut Sprite, Option<&Blinker>), Without<CardLayout>>,
    mut indicator_q: Query<(Entity, &mut Indicator)>,
    mut context: ResMut<GameplayContext>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut send: EventWriter<UiEvent>,
) {
    if context.phase != VagabondGamePhase::Play {
        return;
    }

    let target = event.target;

    if let Ok((layout, sprite, transform, hand, tracker)) = sprite_q.get(target) {
        let card = context.hand.get(hand.0).cloned();
        if tracker.is_none() && card.as_ref().is_none_or(|card| card.cost > context.cached_state.erg[kind_to_erg_index(card.kind)]) {
            if let Some(frame) = layout.frame {
                if let Ok((mut bg_sprite, blink)) = bg_q.get_mut(frame) {
                    if let Some(blink) = blink {
                        blink.remove(&mut commands, &mut bg_sprite, frame);
                    }
                    commands.entity(frame).insert(Blinker::new(bg_sprite.color, bevy::color::palettes::basic::RED.into(), BLINKER_COUNT, BLINKER_SPEED));
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
                    context.cached_state.erg[kind_to_erg_index(card.kind)] += card.cost;
                    send.send(UiEvent::PlayerErg(context.cached_state.erg));
                }
                context.card_picks.remove(&(hand.0 as CardIdxType));
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
    indicator_q: Query<Entity, With<IndicatorActive>>,
    mut commands: Commands,
) {
    commands.entity(event.target).insert(PickingBehavior::default());
    if let Ok(entity) = indicator_q.get_single() {
        commands.entity(entity).remove::<IndicatorActive>();
    }
}

fn on_card_drop(
    // bevy system
    event: Trigger<Pointer<DragDrop>>,
    mut indicator_q: Query<&mut Indicator, With<IndicatorActive>>,
    mut machine_q: Query<&MachineKind>,
    hand_q: Query<&HandCard>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    let dropped_on = event.target;

    if let Ok(mut indicator) = indicator_q.get_single_mut() {
        indicator.target = machine_q.get_mut(dropped_on).ok().copied();
        if let Some(target) = indicator.target {
            if let Ok(hand) = hand_q.get(indicator.parent) {
                context.add_card_pick(hand.0, target);
                if let Some((kind, cost)) = context.hand.get_mut(hand.0).map(|card| (card.kind, card.cost)) {
                    context.cached_state.erg[kind_to_erg_index(kind)] -= cost;
                    send.send(UiEvent::PlayerErg(context.cached_state.erg));
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
        let card = cached.queue.iter().find(|(_, d)| queue_item.0 == *d).map(|(c, _)| c.card.clone());
        commands.trigger_targets(UpdateCardTooltipEvent::new(event.pointer_location.position, card), tooltip.0);
    }
}

fn on_out_process(
    // bevy system
    _event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    tooltip: Res<CardTooltip>,
) {
    commands.entity(tooltip.0).insert(Visibility::Hidden);
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

fn map_kind_to_row(kind: AttrKind) -> usize {
    match kind {
        AttrKind::Analyze => 0,
        AttrKind::Breach => 1,
        AttrKind::Compute => 2,
        AttrKind::Disrupt => 3,
    }
}

fn roll_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut roll_q: Query<(&mut Text2d, &mut TextColor, &RollText)>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::Roll(roll) => {
                for (mut text, mut color, RollText(index)) in roll_q.iter_mut() {
                    *text = format!("{}", roll[*index]).into();
                    *color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(local_erg, remote_erg, _) => {
                for (_, mut color, RollText(index)) in roll_q.iter_mut() {
                    *color = match local_erg[*index].cmp(&remote_erg[*index]) {
                        Ordering::Less => bevy::color::palettes::basic::RED,
                        Ordering::Equal => bevy::color::palettes::basic::YELLOW,
                        Ordering::Greater => bevy::color::palettes::basic::GREEN,
                    }
                    .into();
                }
            }
            _ => {}
        }
    }
}

fn hand_ui_update(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<UiEvent>,
    hand_q: Query<(Entity, &HandCard)>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for (entity, hand) in &hand_q {
                let card = player_state.hand.get(hand.0).and_then(|o| dm.convert_card(o));
                commands.entity(entity).trigger(CardPopulateEvent::from(card));
            }
        }
    }
}

fn erg_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut erg_q: Query<(&mut Text2d, &PlayerStateText)>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerErg(erg) = ui_event {
            for (mut erg_text, state_text) in erg_q.iter_mut() {
                if let PlayerStateText::Erg(index) = state_text {
                    *erg_text = format!("{}", erg[*index]).into();
                }
            }
        }
    }
}

fn phase_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut sprite_q: Query<(&mut Sprite, &PhaseIcon)>,
) {
    for ui_event in receive.read() {
        if let UiEvent::GamePhase(phase) = ui_event {
            for (mut sprite, icon) in sprite_q.iter_mut() {
                let color = if *phase == icon.0 {
                    bevy::color::palettes::css::CHARTREUSE
                } else {
                    Srgba::new(0.2, 0.2, 0.2, 1.0)
                };
                sprite.color = color.into();
            }
        }
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

fn local_state_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut context: ResMut<GameplayContext>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::PlayerState(player_state) => {
                context.cached_state = player_state.clone();
                context.hand = player_state.hand.iter().filter_map(|card| dm.convert_card(card)).collect();
            }
            UiEvent::MachineStateUpdate(local, remote) => {
                context.cached_local = cache_game_machine(local, &dm);
                context.cached_remote = cache_game_machine(remote, &dm);
            }
            _ => {}
        }
    }
}

fn local_ui_update(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<(&mut Text2d, &mut TextColor, &PlayerStateText)>,
    mut row_q: Query<(Entity, &mut Sprite, Option<&Glower>), With<AttributeRow>>,
    mut context: ResMut<GameplayContext>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::PlayerState(player_state) => {
                for (mut text, _, state_text) in text_q.iter_mut() {
                    match state_text {
                        PlayerStateText::Attribute(row, col) => *text = format!("{}", player_state.attr[*row][*col]).into(),
                        PlayerStateText::Deck => *text = player_state.deck.to_string().into(),
                        PlayerStateText::Heap => *text = player_state.heap.len().to_string().into(),
                        _ => {}
                    }
                }
            }
            UiEvent::ChooseAttr(kind) => {
                if context.phase != VagabondGamePhase::Pick {
                    continue;
                }

                if kind.is_none() {
                    for (entity, sprite, _) in row_q.iter() {
                        commands.entity(entity).insert(Glower::new(sprite.color, Srgba::new(0.0, 1.0, 0.0, 1.0).into(), GLOWER_SPEED));
                    }
                } else {
                    for (entity, mut sprite, glower) in row_q.iter_mut() {
                        if let Some(glower) = glower {
                            glower.remove(&mut commands, &mut sprite, entity);
                        }
                    }
                }

                for (_, mut color, state_text) in text_q.iter_mut() {
                    if let PlayerStateText::Attribute(row, _) = state_text {
                        *color = if let Some(kind) = kind {
                            if *row == map_kind_to_row(*kind) {
                                context.attr_pick = Some(*kind);
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
            _ => {}
        }
    }
}

fn remote_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<(&mut Text2d, &mut TextColor, &RemoteAttrText)>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::Roll(_) => {
                for (mut attr_text, mut color, RemoteAttrText(_)) in text_q.iter_mut() {
                    *attr_text = "?".into();
                    *color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(_, _, remote_attr) => {
                for (mut attr_text, mut color, RemoteAttrText(index)) in text_q.iter_mut() {
                    *attr_text = remote_attr[*index].to_string().into();
                    *color = bevy::color::palettes::basic::RED.into();
                }
            }
            _ => {}
        }
    }
}

fn machine_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut commands: Commands,
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
    mut sprite_q: Query<(&MachineKind, &mut Sprite, &MachineQueueItem)>,
    running_q: Query<(Entity, &MachineKind, &MachineRunning)>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::MachineInfoUpdate(machine, info) => {
                for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
                    if machine_component == machine {
                        match kind {
                            MachineTextKind::Title => *text = info.name.to_string().into(),
                            MachineTextKind::Id => *text = info.id.to_string().into(),
                            _ => {}
                        }
                    }
                }
            }
            UiEvent::MachineStateUpdate(local, remote) => {
                for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
                    if let MachineTextKind::Vitals(index) = kind {
                        let machine = if *machine_component == MachineKind::Local {
                            local
                        } else {
                            remote
                        };
                        *text = machine.vitals[*index].to_string().into();
                    }
                }

                for (machine_component, mut sprite, MachineQueueItem(index)) in sprite_q.iter_mut() {
                    let (machine, player_owned) = if *machine_component == MachineKind::Local {
                        (local, true)
                    } else {
                        (remote, false)
                    };

                    sprite.color = if let Some(process) = machine.queue.iter().find(|(_, delay)| delay == index).map(|(item, _)| item) {
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
                        local
                    } else {
                        remote
                    };
                    let card = machine.running.get(running.0).and_then(|process| dm.convert_card(&process.player_card));
                    commands.entity(entity).trigger(CardPopulateEvent::from(card));
                }
            }
            _ => {}
        }
    }
}

fn tty_update(
    // bevy system
    event: Trigger<TTYMessageEvent>,
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
    commands: Commands,
    mut gate: ResMut<GateIFace>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    let new_phase = match gate.grx.try_recv() {
        Ok(GateCommand::GameStartTurn(gate_response)) => recv_start_turn(commands, *gate_response),
        Ok(GateCommand::GameRoll(gate_response)) => recv_roll(commands, *gate_response, &mut send),
        Ok(GateCommand::GameChooseAttr(gate_response)) => recv_choose_attr(commands, *gate_response, &mut send),
        Ok(GateCommand::GameResources(gate_response)) => recv_resources(commands, *gate_response, &mut send),
        Ok(GateCommand::GamePlayCard(gate_response)) => recv_play_card(commands, *gate_response),
        Ok(GateCommand::GameResolveCards(gate_response)) => recv_resolve_cards(commands, *gate_response),
        Ok(GateCommand::GameEndTurn(gate_response)) => recv_end_turn(commands, *gate_response),
        Ok(GateCommand::GameTick(gate_response)) => recv_tick(commands, *gate_response, &mut context),
        Ok(GateCommand::GameEndGame(gate_response)) => recv_end_game(commands, *gate_response),
        Ok(GateCommand::GameUpdateState(gate_response)) => recv_update_state(*gate_response, &mut send),
        Ok(_) => None,
        Err(_) => None,
    };
    if let Some(phase) = new_phase {
        context.phase = phase;
        send.send(UiEvent::GamePhase(phase));
    }
}

fn recv_start_turn(mut commands: Commands, response: GameStartTurnResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Remote, "TURN STARTED"));
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_roll(mut commands: Commands, response: GameRollMessage, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Local, "CHOOSE ATTR"));
    send.send(UiEvent::Roll(response.roll));
    send.send(UiEvent::ChooseAttr(None));
    Some(VagabondGamePhase::Pick)
}

fn recv_choose_attr(mut commands: Commands, response: GameChooseAttrResponse, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Remote, "ATTR CHOSEN"));
    if !response.success {
        send.send(UiEvent::ChooseAttr(None));
    }

    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resources(mut commands: Commands, response: GameResourcesMessage, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Local, "PLAY CARDS"));
    send.send(UiEvent::PlayerErg(response.player_state_view.erg));
    send.send(UiEvent::PlayerState(response.player_state_view));
    send.send(UiEvent::Resources(response.local_erg, response.remote_erg, response.remote_attr));
    Some(VagabondGamePhase::Play)
}

fn recv_play_card(mut commands: Commands, response: GamePlayCardResponse) -> Option<VagabondGamePhase> {
    let success = response.success.iter().all(|&success| success);
    commands.trigger(TTYMessageEvent::new(MachineKind::Remote, "CARDS PLAYED"));
    success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resolve_cards(mut commands: Commands, _response: GameResolveCardsMessage) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Local, "DRAW CARDS"));
    Some(VagabondGamePhase::Draw)
}

fn recv_end_turn(mut commands: Commands, response: GameEndTurnResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Remote, "END TURN"));
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_tick(mut commands: Commands, response: GameTickMessage, context: &mut GameplayContext) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Local, "START TURN"));
    context.reset(response.tick);
    Some(VagabondGamePhase::Start)
}

fn recv_end_game(mut commands: Commands, _response: GameEndGameResponse) -> Option<VagabondGamePhase> {
    commands.trigger(TTYMessageEvent::new(MachineKind::Local, "END GAME"));
    None
}

fn recv_update_state(response: GameUpdateStateResponse, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    send.send(UiEvent::PlayerErg(response.player_state.erg));
    send.send(UiEvent::PlayerState(response.player_state));
    send.send(UiEvent::MachineStateUpdate(response.local_machine, response.remote_machine));
    None
}

pub fn gameplay_exit(
    // bevy system
    mut commands: Commands,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    commands.remove_resource::<CardTooltip>();
    commands.remove_resource::<GameplayContext>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
