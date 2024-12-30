use bevy::prelude::*;

use hall::data::core::{AttributeValues, Attributes};
use vagabond::data::{VagabondCard, VagabondPart};
use warehouse::data::player_bio::PlayerBio;

use crate::manager::{AtlasManager, DataManager, ScreenLayout, ScreenLayoutManager, ScreenLayoutManagerParams, WarehouseManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::compose_init::ComposeInitHandoff;
use crate::screen::shared::{on_out_reset_color, on_update_tooltip, CardLayout, CardPopulateEvent, CardTooltip, UpdateCardTooltipEvent};
use crate::system::ui_effects::{Glower, Hider, SetColorEvent, TextTip, UiFxTrackedColor, UiFxTrackedSize};
use crate::system::AppState;

const SCREEN_LAYOUT: &str = "compose";

const GLOWER_DROP_TARGET_SPEED: f32 = 4.0;
const GLOWER_COMMIT_SPEED: f32 = 8.0;

pub struct ComposeMainPlugin;

impl Plugin for ComposeMainPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppState::Compose), compose_enter)
            .add_systems(Update, compose_update.run_if(in_state(AppState::Compose)))
            .add_systems(PostUpdate, populate_part_layouts.run_if(in_state(AppState::Compose)))
            .add_systems(OnExit(AppState::Compose), compose_exit);
    }
}

#[derive(Default, PartialEq)]
enum ComposeState {
    #[default]
    Build,
    Ready,
    Committed,
}

#[derive(Resource, Default)]
struct ComposeContext {
    state: ComposeState,
    deck: Vec<VagabondCard>,
    attributes: Attributes,
}

#[derive(Event)]
struct FinishPlayerTrigger;

#[derive(Component)]
struct CardHeader {
    index: usize,
}

impl CardHeader {
    fn new(index: usize) -> Self {
        Self {
            index,
        }
    }
}

struct PopulatePlayerUiData {
    player_bio: PlayerBio,
}

#[derive(Event)]
enum PopulatePlayerUi {
    Hide,
    Show(PopulatePlayerUiData),
}

#[derive(Debug, Copy, Clone)]
enum StatRowKind {
    Analyze,
    Breach,
    Compute,
    Disrupt,
    Build,
    Detail,
}

#[derive(Debug, Component)]
enum Slot {
    StatRow(StatRowKind),
    Build,
    Detail,
    Card,
    Empty(Entity),
}

#[derive(Component, Default)]
struct PartHolder {
    part: Option<VagabondPart>,
}

impl PartHolder {
    fn new(part: VagabondPart) -> Self {
        Self {
            part: Some(part),
        }
    }
}

#[derive(Component)]
struct PlayerBioGroup;

#[derive(Component)]
struct DeckGutterGroup;

#[derive(Component)]
enum InfoKind {
    Name,
    ID,
    Birthplace,
    Age,
}

#[derive(Component)]
struct PartLayout {
    build: [Entity; 4],
    detail: [Entity; 4],
    values: [Entity; 4],
}

impl PartLayout {
    fn new() -> Self {
        Self {
            build: [Entity::PLACEHOLDER; 4],
            detail: [Entity::PLACEHOLDER; 4],
            values: [Entity::PLACEHOLDER; 4],
        }
    }
}

#[derive(Component)]
struct CommitButton;

#[derive(Resource)]
struct Draggable {
    drag: Entity,
    active: bool,
}

impl Draggable {
    fn new(drag: Entity) -> Self {
        Self {
            drag,
            active: false,
        }
    }
}

fn make_full_part_layout(commands: &mut Commands, layout: &ScreenLayout, name: &str) -> PartLayout {
    let mut part_layout = PartLayout::new();
    let ant = commands.entity(layout.entity(&format!("{}/ant", name))).id();
    let brd = commands.entity(layout.entity(&format!("{}/brd", name))).id();
    let cpu = commands.entity(layout.entity(&format!("{}/cpu", name))).id();
    let dsk = commands.entity(layout.entity(&format!("{}/dsk", name))).id();
    part_layout.build = [ant, brd, cpu, dsk];

    let ins = commands.entity(layout.entity(&format!("{}/ins", name))).id();
    let rol = commands.entity(layout.entity(&format!("{}/rol", name))).id();
    let loc = commands.entity(layout.entity(&format!("{}/loc", name))).id();
    let dis = commands.entity(layout.entity(&format!("{}/dis", name))).id();
    part_layout.detail = [ins, rol, loc, dis];

    let a = commands.entity(layout.entity(&format!("{}/a", name))).id();
    let b = commands.entity(layout.entity(&format!("{}/b", name))).id();
    let c = commands.entity(layout.entity(&format!("{}/c", name))).id();
    let d = commands.entity(layout.entity(&format!("{}/d", name))).id();
    part_layout.values = [a, b, c, d];

    part_layout
}

trait PartEntityCommandsExtension {
    fn observe_part_drag(self) -> Self;
    fn observe_part_drop(self) -> Self;
    fn insert_empty_slot(self, slot: Slot, layout: PartLayout) -> Self;
    fn insert_filled_slot(self, slot: Slot, layout: PartLayout, part: VagabondPart) -> Self;
    fn observe_commit_button(self) -> Self;
    fn observe_card_header(self, index: usize) -> Self;
}

impl PartEntityCommandsExtension for &mut EntityCommands<'_> {
    fn observe_part_drag(self) -> Self {
        self //
            .observe(on_part_drag_start)
            .observe(on_part_drag)
            .observe(on_part_drag_end)
    }
    fn observe_part_drop(self) -> Self {
        self //
            .observe(on_part_drop)
    }
    fn insert_empty_slot(self, slot: Slot, layout: PartLayout) -> Self {
        self //
            .insert((slot, layout, PartHolder::default(), PickingBehavior::default()))
    }
    fn insert_filled_slot(self, slot: Slot, layout: PartLayout, part: VagabondPart) -> Self {
        self //
            .insert((slot, layout, PartHolder::new(part), PickingBehavior::default()))
    }
    fn observe_commit_button(self) -> Self {
        self //
            .insert((CommitButton, PickingBehavior::default()))
            .observe(on_click_commit)
            .observe(on_over_commit)
            .observe(on_out_reset_color)
    }
    fn observe_card_header(self, index: usize) -> Self {
        self //
            .insert((CardHeader::new(index), PickingBehavior::default()))
            .observe(on_over_header)
            .observe(on_out_header)
    }
}

fn compose_enter(
    // bevy system
    mut commands: Commands,
    init_handoff: Res<ComposeInitHandoff>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    mut slm_params: ScreenLayoutManagerParams,
) {
    let parts = init_handoff.parts.clone();
    commands.remove_resource::<ComposeInitHandoff>();

    let (layout, base_id) = slm.build(&mut commands, SCREEN_LAYOUT, &am, &mut slm_params);

    commands.entity(base_id).with_children(|parent| {
        parent.spawn(Observer::new(on_finish_player));
        parent.spawn(Observer::new(on_populate_bio_ui));
        parent.spawn(Observer::new(on_populate_deck_ui));
        parent.spawn(Observer::new(on_commit_button_ui));
    });

    let container = commands.entity(layout.entity("text_tip")).insert_text_tip_container(layout.entity("text_tip/text")).id();
    commands.entity(layout.entity("attributes/a")).insert_text_tip(container, "Analyze");
    commands.entity(layout.entity("attributes/b")).insert_text_tip(container, "Breach");
    commands.entity(layout.entity("attributes/c")).insert_text_tip(container, "Compute");
    commands.entity(layout.entity("attributes/d")).insert_text_tip(container, "Disrupt");

    const ATTR: [(&str, StatRowKind, [&str; 4]); 4] = [
        //
        ("attributes/row_a", StatRowKind::Analyze, ["attributes/aa", "attributes/ab", "attributes/ac", "attributes/ad"]),
        ("attributes/row_b", StatRowKind::Breach, ["attributes/ba", "attributes/bb", "attributes/bc", "attributes/bd"]),
        ("attributes/row_c", StatRowKind::Compute, ["attributes/ca", "attributes/cb", "attributes/cc", "attributes/cd"]),
        ("attributes/row_d", StatRowKind::Disrupt, ["attributes/da", "attributes/db", "attributes/dc", "attributes/dd"]),
    ];

    for (row_name, kind, row) in ATTR {
        let mut row_part_layout = PartLayout::new();
        row_part_layout.values = row.map(|item| commands.entity(layout.entity(item)).id());
        commands //
            .entity(layout.entity(row_name))
            .insert_empty_slot(Slot::StatRow(kind), row_part_layout)
            .observe_part_drag()
            .observe_part_drop();
    }

    const BUILD: [&str; 4] = ["ant", "brd", "cpu", "dsk"];
    let mut build_part_layout = PartLayout::new();
    build_part_layout.build = BUILD.map(|item| commands.entity(layout.entity(item)).id());
    commands //
        .entity(layout.entity("build"))
        .insert_empty_slot(Slot::Build, build_part_layout)
        .observe_part_drag()
        .observe_part_drop();

    const BUILD_VALUES: [&str; 4] = ["build_a", "build_b", "build_c", "build_d"];
    let mut build_values_part_layout = PartLayout::new();
    build_values_part_layout.values = BUILD_VALUES.map(|item| commands.entity(layout.entity(item)).id());
    commands //
        .entity(layout.entity("build_values"))
        .insert_empty_slot(Slot::StatRow(StatRowKind::Build), build_values_part_layout)
        .observe_part_drag()
        .observe_part_drop();

    const DETAIL: [&str; 4] = ["ins", "rol", "loc", "dis"];
    let mut detail_part_layout = PartLayout::new();
    detail_part_layout.detail = DETAIL.map(|item| commands.entity(layout.entity(item)).id());
    commands //
        .entity(layout.entity("detail"))
        .insert_empty_slot(Slot::Detail, detail_part_layout)
        .observe_part_drag()
        .observe_part_drop();

    const DETAIL_VALUES: [&str; 4] = ["detail_a", "detail_b", "detail_c", "detail_d"];
    let mut detail_values_part_layout = PartLayout::new();
    detail_values_part_layout.values = DETAIL_VALUES.map(|item| commands.entity(layout.entity(item)).id());
    commands //
        .entity(layout.entity("detail_values"))
        .insert_empty_slot(Slot::StatRow(StatRowKind::Detail), detail_values_part_layout)
        .observe_part_drag()
        .observe_part_drop();

    commands.entity(layout.entity("bio")).insert((PlayerBioGroup, Visibility::Hidden));
    commands.entity(layout.entity("bio/id")).insert(InfoKind::ID);
    commands.entity(layout.entity("bio/name")).insert(InfoKind::Name);
    commands.entity(layout.entity("bio/place")).insert(InfoKind::Birthplace);
    commands.entity(layout.entity("bio/age")).insert(InfoKind::Age);

    for (index, part) in parts.iter().enumerate() {
        let name = format!("part{}", index);
        let part_layout = make_full_part_layout(&mut commands, layout, &name);

        let part_entity = commands //
            .entity(layout.entity(&name))
            .insert_filled_slot(Slot::Card, part_layout, part.clone())
            .observe_part_drag()
            .observe_part_drop()
            .id();

        let slot_name = format!("part{}_slot", index);
        commands //
            .entity(layout.entity(&slot_name))
            .insert_empty_slot(Slot::Empty(part_entity), PartLayout::new())
            .observe_part_drop();
    }

    let tooltip = CardLayout::build(&mut commands, layout, "tooltip");
    let tooltip_id = commands.entity(tooltip).insert(Visibility::Hidden).observe(on_update_tooltip).id();
    commands.insert_resource(CardTooltip::new(tooltip_id));

    commands.entity(layout.entity("gutter")).insert((DeckGutterGroup, Visibility::Hidden));
    for index in 0..40 {
        let base_name = format!("gutter/card{:02}", index);
        let header = CardLayout::build(&mut commands, layout, &base_name);
        commands.entity(header).observe_card_header(index);
    }

    commands.entity(layout.entity("commit")).observe_commit_button();

    let draggable_layout = make_full_part_layout(&mut commands, layout, "draggable");
    let draggable = commands //
        .entity(layout.entity("draggable"))
        .insert((Slot::Card, draggable_layout, PartHolder::default())) // no PickableBehavior::default()!
        .insert(Visibility::Hidden)
        .id();
    commands.insert_resource(Draggable::new(draggable));

    commands.insert_resource(ComposeContext::default());
}

fn on_part_drag_start(
    //
    event: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut holder_q: Query<(Entity, Option<&UiFxTrackedColor>, &Slot, &mut PartHolder)>,
    mut draggable: ResMut<Draggable>,
) {
    if let Ok([(_, _, _, mut holder), (_, _, _, mut drag_holder)]) = holder_q.get_many_mut([event.target, draggable.drag]) {
        if holder.part.is_none() {
            draggable.active = false;
            return;
        }

        drag_holder.part = holder.part.clone();
        holder.part = None;
        draggable.active = true;

        commands.entity(event.target).insert(PickingBehavior::IGNORE);
    }

    for (entity, source_color, slot, holder) in &holder_q {
        if entity != draggable.drag {
            let glow = match slot {
                Slot::Card => continue,
                Slot::Empty(_) => Srgba::new(0.7, 0.7, 0.0, 1.0),
                _ if holder.part.is_none() => Srgba::new(0.0, 1.0, 0.0, 1.0),
                _ => Srgba::new(0.8, 0.8, 0.8, 1.0),
            };
            if let Some(color) = source_color {
                let glower = Glower::new(color.color, glow, GLOWER_DROP_TARGET_SPEED);
                commands.entity(entity).insert(glower);
            }
        }
    }
}

fn on_part_drag(
    //
    event: Trigger<Pointer<Drag>>,
    mut draggable_q: Query<&mut Transform, With<PartHolder>>,
    draggable: Res<Draggable>,
) {
    if !draggable.active {
        return;
    }

    if let Ok(mut transform) = draggable_q.get_mut(draggable.drag) {
        transform.translation = event.pointer_location.position.extend(100.0);
        transform.translation.y = -transform.translation.y;
    }
}

fn on_part_drag_end(
    //
    event: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    mut holder_q: Query<(&mut PartHolder, &Slot)>,
    mut draggable: ResMut<Draggable>,
    mut glower_q: Query<(Entity, &Glower), Without<CommitButton>>,
) {
    if !draggable.active {
        return;
    }

    let mut original = Entity::PLACEHOLDER;
    if let Ok([mut target, mut drag]) = holder_q.get_many_mut([event.target, draggable.drag]) {
        original = handle_swap(None, &mut target.0, &mut drag.0, target.1);
    }

    handle_empty(event.target, original, holder_q);

    draggable.active = false;
    commands.entity(event.target).insert(PickingBehavior::default());

    for (entity, glower) in glower_q.iter_mut() {
        glower.remove(&mut commands, entity);
    }
}

fn handle_swap(source: Option<&mut PartHolder>, target: &mut PartHolder, drag: &mut PartHolder, target_slot: &Slot) -> Entity {
    if let Some(source) = source {
        source.part = target.part.clone();
    }

    if drag.part.is_some() {
        target.part = drag.part.clone();
        drag.part = None;
    }

    match target_slot {
        Slot::Empty(original) => *original,
        _ => Entity::PLACEHOLDER,
    }
}

fn handle_empty(empty: Entity, original: Entity, mut holder_q: Query<(&mut PartHolder, &Slot)>) {
    if original == Entity::PLACEHOLDER {
        return;
    }

    if let Ok([(mut empty, _), (mut card, _)]) = holder_q.get_many_mut([empty, original]) {
        card.part = empty.part.clone();
        empty.part = None;
    }
}

fn on_part_drop(
    //
    event: Trigger<Pointer<DragDrop>>,
    mut commands: Commands,
    mut holder_q: Query<(&mut PartHolder, &Slot)>,
    draggable: Res<Draggable>,
) {
    if !draggable.active {
        return;
    }

    let mut original = Entity::PLACEHOLDER;
    if let Ok([(mut source, _), (mut target, slot), (mut drag, _)]) = holder_q.get_many_mut([event.dropped, event.target, draggable.drag]) {
        original = handle_swap(Some(&mut source), &mut target, &mut drag, slot);
    }

    handle_empty(event.target, original, holder_q);

    commands.trigger(FinishPlayerTrigger);
}

fn on_click_commit(
    //
    _event: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut context: ResMut<ComposeContext>,
) {
    if context.state != ComposeState::Ready {
        return;
    }
    context.state = ComposeState::Committed;
    commands.trigger(FinishPlayerTrigger);
}

fn on_over_commit(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
) {
    commands.entity(event.target).trigger(SetColorEvent::new(event.target, bevy::color::palettes::basic::RED));
}

fn on_over_header(
    //
    event: Trigger<Pointer<Over>>,
    mut commands: Commands,
    header_q: Query<(&Transform, &CardHeader)>,
    tooltip_q: Query<(&Transform, &UiFxTrackedSize)>,
    tooltip: Res<CardTooltip>,
    context: Res<ComposeContext>,
    window_q: Query<&Window>,
) {
    let window = window_q.single();
    if let Ok((header_transform, header)) = header_q.get(event.target) {
        if let Ok((tooltip_transform, tooltip_size)) = tooltip_q.get(tooltip.entity) {
            let new_y = (header_transform.translation.y + (tooltip_size.y / 2.0)).clamp(-window.height() + tooltip_size.y, 0.0);
            let position = Vec2::new(tooltip_transform.translation.x, -new_y);
            let card = context.deck.get(header.index).cloned();
            commands.entity(tooltip.entity).remove::<Hider>().trigger(UpdateCardTooltipEvent::new(position, card, context.attributes));
        }
    }
}

fn on_out_header(
    //
    _event: Trigger<Pointer<Out>>,
    mut commands: Commands,
    tooltip: Res<CardTooltip>,
) {
    commands.entity(tooltip.entity).insert(Hider::new(0.25, Visibility::Hidden));
}

fn populate_part_layouts(
    //
    layout_q: Query<(Entity, &PartLayout, &PartHolder, &Slot), Changed<PartHolder>>,
    mut text_q: Query<&mut Text2d>,
    mut commands: Commands,
) {
    for (entity, layout, holder, slot) in &layout_q {
        for (idx, e) in layout.build.iter().enumerate() {
            if let Ok(mut text) = text_q.get_mut(*e) {
                *text = if let Some(part) = &holder.part {
                    part.build[idx].title.clone().into()
                } else {
                    "-".into()
                }
            }
        }
        for (idx, e) in layout.detail.iter().enumerate() {
            if let Ok(mut text) = text_q.get_mut(*e) {
                *text = if let Some(part) = &holder.part {
                    part.detail[idx].title.clone().into()
                } else {
                    "-".into()
                }
            }
        }
        for (idx, e) in layout.values.iter().enumerate() {
            if let Ok(mut text) = text_q.get_mut(*e) {
                *text = if let Some(part) = &holder.part {
                    part.values[idx].to_string().into()
                } else {
                    "-".into()
                }
            }
        }

        if let Slot::Card = slot {
            let visibility = if holder.part.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
            commands.entity(entity).insert(visibility);
        }
    }
}

fn on_commit_button_ui(
    // bevy system
    event: Trigger<PopulatePlayerUi>,
    mut commands: Commands,
    mut glower_q: Query<(Entity, &UiFxTrackedColor, Option<&Glower>), With<CommitButton>>,
) {
    if let Ok((entity, source_color, glower)) = glower_q.get_single_mut() {
        match *event {
            PopulatePlayerUi::Hide => {
                if let Some(glower) = glower {
                    glower.remove(&mut commands, entity);
                }
            }
            PopulatePlayerUi::Show(_) => {
                commands.entity(entity).insert(Glower::new(source_color.color, bevy::color::palettes::basic::GREEN, GLOWER_COMMIT_SPEED));
            }
        };
    }
}

fn on_populate_deck_ui(
    // bevy system
    event: Trigger<PopulatePlayerUi>,
    mut commands: Commands,
    header_q: Query<(Entity, &CardHeader)>,
    gutter_q: Query<Entity, With<DeckGutterGroup>>,
    context: Res<ComposeContext>,
) {
    let visibility = match *event {
        PopulatePlayerUi::Hide => {
            commands.trigger_targets(CardPopulateEvent::default(), header_q.iter().map(|(e, _)| e).collect::<Vec<_>>());
            Visibility::Hidden
        }
        PopulatePlayerUi::Show(_) => {
            for (idx, card) in context.deck.iter().enumerate() {
                if let Some((entity, _)) = header_q.iter().find(|(_, h)| h.index == idx) {
                    commands.entity(entity).trigger(CardPopulateEvent::new(Some(card.clone()), context.attributes));
                }
            }
            Visibility::Visible
        }
    };
    if let Ok(gutter) = gutter_q.get_single() {
        commands.entity(gutter).insert(visibility);
    }
}

fn on_populate_bio_ui(
    // bevy system
    event: Trigger<PopulatePlayerUi>,
    mut commands: Commands,
    mut info_q: Query<(&mut Text2d, &InfoKind), Without<CardLayout>>,
    bio_q: Query<Entity, With<PlayerBioGroup>>,
) {
    let visibility = match &*event {
        PopulatePlayerUi::Hide => Visibility::Hidden,
        PopulatePlayerUi::Show(data) => {
            for (mut info, info_kind) in info_q.iter_mut() {
                match info_kind {
                    InfoKind::Name => *info = data.player_bio.name.clone().into(),
                    InfoKind::ID => *info = data.player_bio.id.clone().into(),
                    InfoKind::Birthplace => *info = data.player_bio.birthplace().clone().into(),
                    InfoKind::Age => *info = data.player_bio.age().to_string().into(),
                }
            }
            Visibility::Visible
        }
    };
    if let Ok(bio) = bio_q.get_single() {
        commands.entity(bio).insert(visibility);
    }
}

fn seed_from_holder(holder: &PartHolder) -> u64 {
    holder.part.as_ref().map(|o| o.seed).unwrap_or_default()
}

fn attributes_from_holder(holder: &PartHolder) -> AttributeValues {
    AttributeValues::from_array(holder.part.as_ref().map(|o| o.values).unwrap_or_default())
}

fn on_finish_player(
    // bevy system
    _event: Trigger<FinishPlayerTrigger>,
    mut commands: Commands,
    holder_q: Query<(&PartHolder, &Slot)>,
    gate: Res<GateIFace>,
    mut context: ResMut<ComposeContext>,
) {
    let mut parts = [0, 0, 0, 0, 0, 0, 0, 0];

    for (holder, holder_kind) in holder_q.iter() {
        if let Some(idx) = match holder_kind {
            Slot::StatRow(row) => match row {
                StatRowKind::Analyze => {
                    context.attributes.analyze = attributes_from_holder(holder);
                    Some(0)
                }
                StatRowKind::Breach => {
                    context.attributes.breach = attributes_from_holder(holder);
                    Some(1)
                }
                StatRowKind::Compute => {
                    context.attributes.compute = attributes_from_holder(holder);
                    Some(2)
                }
                StatRowKind::Disrupt => {
                    context.attributes.disrupt = attributes_from_holder(holder);
                    Some(3)
                }
                StatRowKind::Build => Some(5),
                StatRowKind::Detail => Some(7),
            },
            Slot::Build => Some(4),
            Slot::Detail => Some(6),
            Slot::Card => None,
            Slot::Empty(_) => None,
        } {
            parts[idx] = seed_from_holder(holder);
        }
    }

    if parts.iter().all(|&o| o != 0) {
        if context.state == ComposeState::Build {
            context.state = ComposeState::Ready;
        }
        gate.send_game_build(parts, context.state == ComposeState::Committed);
    } else {
        context.state = ComposeState::Build;
        commands.trigger(PopulatePlayerUi::Hide);
    }
}

#[derive(Resource)]
pub(crate) struct ComposeHandoff {
    pub(crate) local_name: String,
    pub(crate) local_id: String,
}

fn compose_update(
    // bevy system
    mut commands: Commands,
    mut gate: ResMut<GateIFace>,
    wm: Res<WarehouseManager>,
    dm: Res<DataManager>,
    mut app_state: ResMut<NextState<AppState>>,
    mut context: ResMut<ComposeContext>,
) {
    match gate.grx.try_recv() {
        Ok(GateCommand::GameBuild(gate_response)) => match wm.fetch_player(gate_response.seed) {
            Ok(warehouse_response) => {
                if let Some(player_bio) = warehouse_response.player_bio {
                    let handoff = ComposeHandoff {
                        local_name: player_bio.name.clone(),
                        local_id: player_bio.id.clone(),
                    };
                    commands.insert_resource(handoff);

                    context.deck = dm.convert_deck(gate_response.deck);
                    context.deck.sort_by_key(|c| (std::cmp::Reverse(c.rarity), c.set, c.number));

                    let data = PopulatePlayerUiData {
                        player_bio,
                    };
                    commands.trigger(PopulatePlayerUi::Show(data));
                } else {
                    commands.trigger(PopulatePlayerUi::Hide);
                }
            }
            Err(err) => println!("Error: {err}"),
        },
        Ok(GateCommand::GameStartGame(gate_response)) => {
            if gate_response.success {
                app_state.set(AppState::GameplayInit);
            }
        }
        Ok(_) => {}
        Err(_) => {}
    }
}

pub fn compose_exit(
    // bevy system
    mut commands: Commands,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    commands.remove_resource::<CardTooltip>();
    commands.remove_resource::<ComposeContext>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
