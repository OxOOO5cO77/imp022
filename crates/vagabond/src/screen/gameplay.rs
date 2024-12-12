use crate::manager::{AtlasManager, DataManager, ScreenLayoutManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::card_layout::{CardLayout, CardLayoutPiece};
use crate::screen::compose::ComposeHandoff;
use crate::system::ui_effects::{Blinker, Glower};
use crate::system::AppState;
use bevy::prelude::*;
use hall::data::game::GameMachinePlayerView;
use hall::data::player::PlayerStatePlayerView;
use hall::message::*;
use shared_data::attribute::AttributeKind;
use shared_data::build::BuildValueType;
use shared_data::card::{DelayType, ErgType};
use shared_data::mission::MissionNodeIdType;
use std::cmp::{Ordering, PartialEq};
use std::collections::HashMap;
use vagabond::data::VagabondCard;

const SCREEN_LAYOUT: &str = "gameplay";

const BLINKER_COUNT: f32 = 2.0;
const BLINKER_SPEED: f32 = 24.0;
const GLOWER_SPEED: f32 = 4.0;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::GameplayInit), gameplay_init_enter)
            .add_systems(Update, gameplay_init_update.run_if(in_state(AppState::GameplayInit)))
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, (gameplay_update, erg_ui_update, phase_ui_update, card_ui_update, indicator_ui_update, local_state_update, local_ui_update, roll_ui_update, remote_ui_update, machine_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(PostUpdate, cleanup_indicator_post_update.run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit);
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
    phase: VagabondGamePhase,
    attr_pick: Option<AttrKind>,
    card_picks: HashMap<CardIdxType, CardTarget>,
    current_remote: MissionNodeIdType,
    last_state: PlayerStatePlayerView,
    hand: Vec<VagabondCard>,
}

impl Default for GameplayContext {
    fn default() -> Self {
        Self {
            phase: Default::default(),
            attr_pick: None,
            card_picks: Default::default(),
            current_remote: 1,
            last_state: Default::default(),
            hand: Default::default(),
        }
    }
}

impl GameplayContext {
    fn reset(&mut self) {
        self.attr_pick = None;
        self.card_picks.clear();
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
struct PhaseText;

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

#[derive(Component, PartialEq)]
enum MachineTextKind {
    Title,
    Id,
    Stat(usize),
    CurrentProgram,
    Process(usize),
}

#[derive(Component)]
struct MachineText(MachineTextKind);

#[derive(Component, Copy, Clone, PartialEq)]
enum MachineKind {
    Local,
    Remote,
}

struct MachineInfo {
    name: String,
    id: String,
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

trait PickableEntityCommandsExtension {
    fn observe_pickable_row(self, kind: AttrKind) -> Self;
    fn observe_next_button(self) -> Self;
    fn observe_card(self) -> Self;
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
    fn observe_card(self) -> Self {
        self //
            .insert(PickingBehavior::default())
            .observe(on_card_drag_start)
            .observe(on_card_drag)
            .observe(on_card_drag_end)
    }
}

fn gameplay_enter(
    // bevy system
    mut commands: Commands,
    mut handoff: ResMut<GameplayInitHandoff>,
    mut send: EventWriter<UiEvent>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    for_slm: (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
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
            commands.entity(layout.entity_or_default(name)).insert(PlayerStateText::Attribute(row_idx, col_idx));
        }
    }

    const ROLL: [&str; 4] = ["ea", "eb", "ec", "ed"];

    for (roll_idx, roll) in ROLL.iter().enumerate() {
        commands.entity(layout.entity_or_default(roll)).insert(RollText(roll_idx));
    }

    const REMOTE_ATTR: [&str; 4] = ["ra", "rb", "rc", "rd"];

    for (remote_idx, remote) in REMOTE_ATTR.iter().enumerate() {
        commands.entity(layout.entity_or_default(remote)).insert(RemoteAttrText(remote_idx));
    }

    const ERG: [&str; 4] = ["la", "lb", "lc", "ld"];

    for (erg_idx, erg) in ERG.iter().enumerate() {
        commands.entity(layout.entity_or_default(erg)).insert(PlayerStateText::Erg(erg_idx));
    }

    commands.entity(layout.entity_or_default("deck")).insert(PlayerStateText::Deck);
    commands.entity(layout.entity_or_default("heap")).insert(PlayerStateText::Heap);

    commands.entity(layout.entity_or_default("phase")).insert(PhaseText);
    commands.entity(layout.entity_or_default("next")).observe_next_button();

    commands.entity(layout.entity_or_default("attributes/row_a")).observe_pickable_row(AttrKind::Analyze);
    commands.entity(layout.entity_or_default("attributes/row_b")).observe_pickable_row(AttrKind::Breach);
    commands.entity(layout.entity_or_default("attributes/row_c")).observe_pickable_row(AttrKind::Compute);
    commands.entity(layout.entity_or_default("attributes/row_d")).observe_pickable_row(AttrKind::Disrupt);

    const MACHINES: [(&str, MachineKind); 2] = [("local", MachineKind::Local), ("remote", MachineKind::Remote)];

    for (machine_name, machine_kind) in &MACHINES {
        commands.entity(layout.entity_or_default(machine_name)).insert((*machine_kind, PickingBehavior::default())).observe(on_card_drop);

        commands.entity(layout.entity_or_default(&format!("{}/title", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Title)));
        commands.entity(layout.entity_or_default(&format!("{}/id", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Id)));

        commands.entity(layout.entity_or_default(&format!("{}/free_space", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Stat(0))));
        commands.entity(layout.entity_or_default(&format!("{}/thermal_capacity", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Stat(1))));
        commands.entity(layout.entity_or_default(&format!("{}/system_health", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Stat(2))));
        commands.entity(layout.entity_or_default(&format!("{}/open_ports", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Stat(3))));

        commands.entity(layout.entity_or_default(&format!("{}/current_program", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::CurrentProgram)));

        for queue_index in 0..10 {
            commands.entity(layout.entity_or_default(&format!("{}/queue{}", machine_name, queue_index))).insert((*machine_kind, MachineQueueItem(queue_index)));
        }

        commands.entity(layout.entity_or_default(&format!("{}/running1", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Process(0))));
        commands.entity(layout.entity_or_default(&format!("{}/running2", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Process(1))));
        commands.entity(layout.entity_or_default(&format!("{}/running3", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Process(2))));
        commands.entity(layout.entity_or_default(&format!("{}/running4", machine_name))).insert((*machine_kind, MachineText(MachineTextKind::Process(3))));
    }

    for card_index in 1..=5 {
        let built = CardLayout::build(&mut commands, layout, &format!("card{card_index}"), card_index - 1);
        commands.entity(built).observe_card();
    }

    commands.remove_resource::<GameplayInitHandoff>();
    commands.insert_resource(GameplayContext::default());

    let initial_response = handoff.initial_response.take().unwrap();
    recv_update_state(*initial_response, &mut send);
    send.send(UiEvent::GamePhase(VagabondGamePhase::Start));

    let info = MachineInfo {
        name: handoff.name.clone(),
        id: handoff.id.clone(),
    };
    send.send(UiEvent::MachineInfoUpdate(MachineKind::Local, info));
}

fn on_click_next(_: Trigger<Pointer<Click>>, mut context: ResMut<GameplayContext>, gate: Res<GateIFace>) {
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

#[allow(clippy::too_many_arguments)]
fn on_card_drag_start(
    //
    event: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    sprite_q: Query<(&CardLayout, &mut Sprite, &mut Transform, Option<&IndicatorTracker>), With<PickingBehavior>>,
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

    if let Ok((layout, sprite, transform, tracker)) = sprite_q.get(target) {
        let card = context.hand.get(layout.slot).cloned();
        if tracker.is_none() && card.as_ref().is_none_or(|card| card.cost > context.last_state.erg[kind_to_erg_index(card.kind)]) {
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
                    context.last_state.erg[kind_to_erg_index(card.kind)] += card.cost;
                    send.send(UiEvent::PlayerErg(context.last_state.erg));
                }
                context.card_picks.remove(&(layout.slot as CardIdxType));
                indicator.target = None;
                indicator.offset = offset;
                commands.entity(entity).insert(IndicatorActive);
            }
        }
    }
}

fn on_card_drag(
    //
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
    //
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
    //
    event: Trigger<Pointer<DragDrop>>,
    mut indicator_q: Query<&mut Indicator, With<IndicatorActive>>,
    mut machine_q: Query<&MachineKind>,
    layout_q: Query<&CardLayout>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    let dropped_on = event.target;

    if let Ok(mut indicator) = indicator_q.get_single_mut() {
        indicator.target = machine_q.get_mut(dropped_on).ok().copied();
        if let Some(target) = indicator.target {
            if let Ok(layout) = layout_q.get(indicator.parent) {
                context.add_card_pick(layout.slot, target);
                if let Some((kind, cost)) = context.hand.get_mut(layout.slot).map(|card| (card.kind, card.cost)) {
                    context.last_state.erg[kind_to_erg_index(kind)] -= cost;
                    send.send(UiEvent::PlayerErg(context.last_state.erg));
                }
            }
        }
    }
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

fn card_ui_update(
    // bevy system
    mut commands: Commands,
    mut receive: EventReader<UiEvent>,
    layout_q: Query<(Entity, &CardLayout)>,
    mut text_q: Query<&mut Text2d, With<CardLayoutPiece>>,
    mut sprite_q: Query<&mut Sprite>,
    dm: Res<DataManager>,
    am: Res<AtlasManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for (entity, layout) in &layout_q {
                let card = player_state.hand.get(layout.slot).and_then(|o| dm.convert_card(o));

                let visibility = if let Some(card) = card {
                    layout.populate(card, &mut text_q, &mut sprite_q, &am);
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                commands.entity(entity).insert(visibility);
            }
        }
    }
}

fn erg_ui_update(mut receive: EventReader<UiEvent>, mut erg_q: Query<(&mut Text2d, &PlayerStateText)>) {
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
    mut text_q: Query<&mut Text2d, With<PhaseText>>,
) {
    for ui_event in receive.read() {
        if let UiEvent::GamePhase(phase) = ui_event {
            let mut text = text_q.single_mut();
            *text = match phase {
                VagabondGamePhase::Start => "Start Turn".to_string(),
                VagabondGamePhase::Pick => "Pick Attribute".to_string(),
                VagabondGamePhase::Play => "Play Card".to_string(),
                VagabondGamePhase::Draw => "Draw Cards".to_string(),
                VagabondGamePhase::Wait(WaitKind::One) => "...".to_string(),
                VagabondGamePhase::Wait(WaitKind::All) => "(Waiting)".to_string(),
            }
            .into();
        }
    }
}

fn local_state_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut context: ResMut<GameplayContext>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            context.last_state = player_state.clone();
            context.hand = player_state.hand.iter().filter_map(|card| dm.convert_card(card)).collect::<Vec<_>>();
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
    mut text_q: Query<(&MachineKind, &mut Text2d, &MachineText)>,
    mut sprite_q: Query<(&MachineKind, &mut Sprite, &MachineQueueItem)>,
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
                    if let MachineTextKind::Stat(index) = kind {
                        let machine = if *machine_component == MachineKind::Local {
                            local
                        } else {
                            remote
                        };
                        *text = machine.stats[*index].to_string().into();
                    }
                }

                for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
                    if let MachineTextKind::CurrentProgram = kind {
                        let machine = if *machine_component == MachineKind::Local {
                            local
                        } else {
                            remote
                        };
                        let mut result = " v- <idle>".to_string();
                        if let Some(current) = machine.queue.iter().find(|(_, delay)| *delay == 0).map(|(item, _)| item) {
                            if let Some(card) = dm.convert_card(&current.player_card) {
                                result = format!(" v- {}", card.title);
                            }
                        }
                        *text = result.into();
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

                for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
                    if let MachineTextKind::Process(index) = kind {
                        let machine = if *machine_component == MachineKind::Local {
                            local
                        } else {
                            remote
                        };
                        let mut result = "?".to_string();
                        if let Some(process) = machine.running.get(*index) {
                            if let Some(card) = dm.convert_card(&process.player_card) {
                                result = card.title.clone();
                            }
                        }
                        *text = result.into();
                    }
                }
            }
            _ => {}
        }
    }
}

fn gameplay_update(
    // bevy system
    mut gate: ResMut<GateIFace>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    let new_phase = match gate.grx.try_recv() {
        Ok(GateCommand::GameStartTurn(gate_response)) => recv_start_turn(*gate_response),
        Ok(GateCommand::GameRoll(gate_response)) => recv_roll(*gate_response, &mut send),
        Ok(GateCommand::GameChooseAttr(gate_response)) => recv_choose_attr(*gate_response, &mut send),
        Ok(GateCommand::GameResources(gate_response)) => recv_resources(*gate_response, &mut send),
        Ok(GateCommand::GamePlayCard(gate_response)) => recv_play_card(*gate_response),
        Ok(GateCommand::GameResolveCards(gate_response)) => recv_resolve_cards(*gate_response),
        Ok(GateCommand::GameEndTurn(gate_response)) => recv_end_turn(*gate_response),
        Ok(GateCommand::GameTick(gate_response)) => recv_tick(*gate_response, &mut context),
        Ok(GateCommand::GameEndGame(gate_response)) => recv_end_game(*gate_response),
        Ok(GateCommand::GameUpdateState(gate_response)) => recv_update_state(*gate_response, &mut send),
        Ok(_) => None,
        Err(_) => None,
    };
    if let Some(phase) = new_phase {
        context.phase = phase;
        send.send(UiEvent::GamePhase(phase));
    }
}

fn recv_start_turn(response: GameStartTurnResponse) -> Option<VagabondGamePhase> {
    println!(
        "[RECV] GameStartTurn {}",
        if response.success {
            "OK"
        } else {
            "ERROR"
        }
    );
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_roll(response: GameRollMessage, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    println!("[RECV] GameRoll => Pick");
    send.send(UiEvent::Roll(response.roll));
    send.send(UiEvent::ChooseAttr(None));
    Some(VagabondGamePhase::Pick)
}

fn recv_choose_attr(response: GameChooseAttrResponse, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    println!(
        "[RECV] GameChooseAttr {}",
        if response.success {
            "OK"
        } else {
            "ERROR"
        }
    );
    if !response.success {
        send.send(UiEvent::ChooseAttr(None));
    }

    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resources(response: GameResourcesMessage, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    println!("[RECV] GameResources => Play");
    send.send(UiEvent::PlayerErg(response.player_state_view.erg));
    send.send(UiEvent::PlayerState(response.player_state_view));
    send.send(UiEvent::Resources(response.local_erg, response.remote_erg, response.remote_attr));
    Some(VagabondGamePhase::Play)
}

fn recv_play_card(response: GamePlayCardResponse) -> Option<VagabondGamePhase> {
    let success = response.success.iter().all(|&success| success);
    println!(
        "[RECV] GamePlayCard {}",
        if success {
            "OK"
        } else {
            "ERROR"
        }
    );
    success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_resolve_cards(_response: GameResolveCardsMessage) -> Option<VagabondGamePhase> {
    println!("[RECV] GameResolveCards => Draw");
    Some(VagabondGamePhase::Draw)
}

fn recv_end_turn(response: GameEndTurnResponse) -> Option<VagabondGamePhase> {
    println!(
        "[RECV] GameEndTurn {}",
        if response.success {
            "OK"
        } else {
            "ERROR"
        }
    );
    response.success.then_some(VagabondGamePhase::Wait(WaitKind::All))
}

fn recv_tick(_response: GameTickMessage, context: &mut GameplayContext) -> Option<VagabondGamePhase> {
    println!("[RECV] GameTick");
    context.reset();
    Some(VagabondGamePhase::Start)
}

fn recv_end_game(response: GameEndGameResponse) -> Option<VagabondGamePhase> {
    println!(
        "[RECV] GameEndGame {}",
        if response.success {
            "OK"
        } else {
            "ERROR"
        }
    );
    None
}

fn recv_update_state(response: GameUpdateStateResponse, send: &mut EventWriter<UiEvent>) -> Option<VagabondGamePhase> {
    println!("[RECV] GameUpdateState");

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
    commands.remove_resource::<GameplayContext>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
