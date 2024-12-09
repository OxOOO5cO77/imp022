use crate::manager::{AtlasManager, DataManager, ScreenLayout, ScreenLayoutManager, WarehouseManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::util;
use crate::system::ui_effects::Glower;
use crate::system::AppState;
use bevy::prelude::*;
use vagabond::data::{VagabondCard, VagabondPart};
use warehouse::data::player_bio::PlayerBio;

const SCREEN_LAYOUT: &str = "compose";

pub struct ComposePlugin;

impl Plugin for ComposePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<FinishPlayer>()
            .add_event::<PopulatePlayerUi>()
            .add_systems(OnEnter(AppState::ComposeInit), compose_init_enter)
            .add_systems(Update, compose_init_update.run_if(in_state(AppState::ComposeInit)))
            .add_systems(OnEnter(AppState::Compose), compose_enter)
            .add_systems(Update, (finish_player, populate_bio_ui, populate_deck_ui, commit_button_ui, compose_update).run_if(in_state(AppState::Compose)))
            .add_systems(PostUpdate, populate_part_layouts.run_if(in_state(AppState::Compose)))
            .add_systems(OnExit(AppState::Compose), compose_exit);
    }
}

#[derive(Resource)]
struct ComposeInitHandoff {
    parts: [VagabondPart; 8],
}

fn compose_init_enter(
    // bevy system
    gate: ResMut<GateIFace>,
) {
    gate.send_game_activate();
}

fn compose_init_update(
    // bevy system
    mut commands: Commands,
    mut gate: ResMut<GateIFace>,
    mut app_state: ResMut<NextState<AppState>>,
    dm: Res<DataManager>,
) {
    if let Ok(GateCommand::GameActivate(response)) = gate.grx.try_recv() {
        let init_handoff = ComposeInitHandoff {
            parts: response.parts.map(|part| dm.convert_part(&part).unwrap_or_default()),
        };
        gate.game_id = response.game_id;
        commands.insert_resource(init_handoff);
        app_state.set(AppState::Compose)
    }
}

#[derive(Resource, Default, PartialEq)]
enum ComposeState {
    #[default]
    Build,
    Ready,
    Committed,
}

#[derive(Event)]
struct FinishPlayer;

struct PopulatePlayerUiData {
    player_bio: PlayerBio,
    deck: Vec<VagabondCard>,
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
struct CardHeader {
    index: usize,
    title: Entity,
    cost: Entity,
    frame: Entity,
}

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
    fn insert_commit_button(self) -> Self;
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
    fn insert_commit_button(self) -> Self {
        self //
            .insert((CommitButton, PickingBehavior::default()))
            .observe(on_click_commit)
            .observe(on_over_commit)
            .observe(on_out_commit)
    }
}

fn compose_enter(
    // bevy system
    mut commands: Commands,
    init_handoff: Res<ComposeInitHandoff>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    for_slm: (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
) {
    let parts = init_handoff.parts.clone();
    commands.remove_resource::<ComposeInitHandoff>();

    let layout = slm.build(&mut commands, SCREEN_LAYOUT, &am, for_slm);

    const ATTR: [(&str, StatRowKind, [&str; 4]); 4] = [
        //
        ("row_a", StatRowKind::Analyze, ["aa", "ab", "ac", "ad"]),
        ("row_b", StatRowKind::Breach, ["ba", "bb", "bc", "bd"]),
        ("row_c", StatRowKind::Compute, ["ca", "cb", "cc", "cd"]),
        ("row_d", StatRowKind::Disrupt, ["da", "db", "dc", "dd"]),
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
        let slot_index = index + 1;

        let name = format!("part{}", slot_index);
        let part_layout = make_full_part_layout(&mut commands, layout, &name);

        let part_entity = commands //
            .entity(layout.entity(&name))
            .insert_filled_slot(Slot::Card, part_layout, part.clone())
            .observe_part_drag()
            .observe_part_drop()
            .id();

        let slot_name = format!("part{}_slot", slot_index);
        commands //
            .entity(layout.entity(&slot_name))
            .insert_empty_slot(Slot::Empty(part_entity), PartLayout::new())
            .observe_part_drop();
    }

    commands.entity(layout.entity("gutter")).insert((DeckGutterGroup, Visibility::Hidden));
    for index in 0..40 {
        let slot_index = index + 1;
        let title_string = format!("gutter/card{:02}/title", slot_index);
        let title = commands.entity(layout.entity(&title_string)).id();
        let cost_string = format!("gutter/card{:02}/cost", slot_index);
        let cost = commands.entity(layout.entity(&cost_string)).id();
        let frame_string = format!("gutter/card{:02}/frame", slot_index);
        let frame = commands.entity(layout.entity(&frame_string)).id();
        let header = CardHeader {
            index,
            title,
            cost,
            frame,
        };
        commands.entity(layout.entity(&format!("gutter/card{:02}", slot_index))).insert(header);
    }

    commands.entity(layout.entity("commit")).insert_commit_button();

    let draggable_layout = make_full_part_layout(&mut commands, layout, "draggable");
    let draggable = commands //
        .entity(layout.entity("draggable"))
        .insert((Slot::Card, draggable_layout, PartHolder::default())) // no PickableBehavior::default()!
        .insert(Visibility::Hidden)
        .id();
    commands.insert_resource(Draggable::new(draggable));

    commands.insert_resource(ComposeState::default());
}

fn on_part_drag_start(
    //
    event: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    mut holder_q: Query<(Entity, &Sprite, &Slot, &mut PartHolder)>,
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

    for (entity, sprite, slot, holder) in &holder_q {
        if entity != draggable.drag {
            let glow = match slot {
                Slot::Card => continue,
                Slot::Empty(_) => Srgba::new(0.7, 0.7, 0.0, 1.0),
                _ if holder.part.is_none() => Srgba::new(0.0, 1.0, 0.0, 1.0),
                _ => Srgba::new(0.8, 0.8, 0.8, 1.0),
            }
            .into();
            let glower = Glower::new(sprite.color, glow);
            commands.entity(entity).insert(glower);
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
    mut glower_q: Query<(Entity, &mut Sprite, &Glower), Without<CommitButton>>,
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

    for (entity, mut sprite, glower) in glower_q.iter_mut() {
        glower.remove(&mut commands, &mut sprite, entity);
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
    mut holder_q: Query<(&mut PartHolder, &Slot)>,
    mut send: EventWriter<FinishPlayer>,
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

    send.send(FinishPlayer);
}

fn on_click_commit(
    //
    _event: Trigger<Pointer<Click>>,
    mut state: ResMut<ComposeState>,
    mut send: EventWriter<FinishPlayer>,
) {
    if *state != ComposeState::Ready {
        return;
    }
    *state = ComposeState::Committed;
    send.send(FinishPlayer);
}

fn on_over_commit(event: Trigger<Pointer<Over>>, mut sprite_q: Query<&mut Sprite, With<CommitButton>>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = bevy::color::palettes::basic::RED.into();
    }
}

fn on_out_commit(event: Trigger<Pointer<Out>>, mut sprite_q: Query<&mut Sprite, With<CommitButton>>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target) {
        sprite.color = bevy::color::palettes::css::DARK_GRAY.into();
    }
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

fn commit_button_ui(
    // bevy system
    mut commands: Commands,
    mut read: EventReader<PopulatePlayerUi>,
    mut glower_q: Query<(Entity, &mut Sprite, Option<&Glower>), With<CommitButton>>,
) {
    if let Some(event) = read.read().last() {
        if let Ok((entity, mut sprite, glower)) = glower_q.get_single_mut() {
            match event {
                PopulatePlayerUi::Hide => {
                    if let Some(glower) = glower {
                        glower.remove(&mut commands, &mut sprite, entity);
                    }
                }
                PopulatePlayerUi::Show(_) => {
                    commands.entity(entity).insert(Glower::new(sprite.color, bevy::color::palettes::basic::GREEN.into()).with_speed(8.0));
                }
            };
        }
    }
}

fn populate_deck_ui(
    // bevy system
    mut commands: Commands,
    mut read: EventReader<PopulatePlayerUi>,
    header_q: Query<&CardHeader>,
    mut text_q: Query<&mut Text2d>,
    mut sprite_q: Query<&mut Sprite>,
    gutter_q: Query<Entity, With<DeckGutterGroup>>,
) {
    if let Some(event) = read.read().last() {
        let visibility = match event {
            PopulatePlayerUi::Hide => Visibility::Hidden,
            PopulatePlayerUi::Show(data) => {
                for (idx, card) in data.deck.iter().enumerate() {
                    if let Some(header) = header_q.iter().find(|h| h.index == idx) {
                        if let Ok(mut title) = text_q.get_mut(header.title) {
                            *title = card.title.clone().into();
                        }
                        if let Ok(mut cost) = text_q.get_mut(header.cost) {
                            *cost = util::map_kind_to_cost(card.kind, card.cost).into();
                        }
                        if let Ok(mut frame) = sprite_q.get_mut(header.frame) {
                            frame.color = util::map_kind_to_color(card.kind);
                        }
                    }
                }
                Visibility::Visible
            }
        };
        if let Ok(gutter) = gutter_q.get_single() {
            commands.entity(gutter).insert(visibility);
        }
    }
}

fn populate_bio_ui(
    // bevy system
    mut commands: Commands,
    mut read: EventReader<PopulatePlayerUi>,
    mut info_q: Query<(&mut Text2d, &InfoKind), Without<CardHeader>>,
    bio_q: Query<Entity, With<PlayerBioGroup>>,
) {
    if let Some(event) = read.read().last() {
        let visibility = match event {
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
}

fn seed_from_holder(holder: &PartHolder) -> u64 {
    holder.part.as_ref().map(|o| o.seed).unwrap_or_default()
}

fn finish_player(
    // bevy system
    mut read: EventReader<FinishPlayer>,
    mut send: EventWriter<PopulatePlayerUi>,
    holder_q: Query<(&PartHolder, &Slot)>,
    gate: Res<GateIFace>,
    mut state: ResMut<ComposeState>,
) {
    if read.read().last().is_some() {
        let mut parts = [0, 0, 0, 0, 0, 0, 0, 0];

        for (holder, holder_kind) in holder_q.iter() {
            if let Some(idx) = match holder_kind {
                Slot::StatRow(row) => match row {
                    StatRowKind::Analyze => Some(0),
                    StatRowKind::Breach => Some(1),
                    StatRowKind::Compute => Some(2),
                    StatRowKind::Disrupt => Some(3),
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
            if *state == ComposeState::Build {
                *state = ComposeState::Ready;
            }
            gate.send_game_build(parts, *state == ComposeState::Committed);
        } else {
            *state = ComposeState::Build;
            send.send(PopulatePlayerUi::Hide);
        }
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
    mut send: EventWriter<PopulatePlayerUi>,
    wm: Res<WarehouseManager>,
    dm: Res<DataManager>,
    mut app_state: ResMut<NextState<AppState>>,
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

                    let mut deck = dm.convert_deck(gate_response.deck);
                    deck.sort_by_key(|c| (std::cmp::Reverse(c.rarity), c.set, c.number));

                    let data = PopulatePlayerUiData {
                        player_bio,
                        deck,
                    };
                    send.send(PopulatePlayerUi::Show(data));
                } else {
                    send.send(PopulatePlayerUi::Hide);
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
    commands.remove_resource::<ComposeState>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
