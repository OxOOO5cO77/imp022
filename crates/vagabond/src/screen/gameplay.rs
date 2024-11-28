use crate::manager::{AtlasManager, DataManager, ScreenLayoutManager};
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::system::app_state::AppState;
use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use hall::data::game::GameMachinePlayerView;
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::{AttrKind, CardIdxType, CardTarget};
use shared_data::build::BuildValueType;
use shared_data::card::{DelayType, ErgType};
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<UiEvent>()
            .add_systems(OnEnter(AppState::Gameplay), gameplay_enter)
            .add_systems(Update, (gameplay_update, card_ui_update, erg_ui_update, phase_ui_update, local_ui_update, roll_ui_update, remote_ui_update, machine_ui_update).run_if(in_state(AppState::Gameplay)))
            .add_systems(OnExit(AppState::Gameplay), gameplay_exit);
    }
}

#[derive(Clone, Debug, PartialEq)]
enum WaitKind {
    One,
    All,
}

#[derive(Default, Clone, Debug, PartialEq)]
enum GameplayState {
    #[default]
    Start,
    Pick,
    Play,
    Draw,
    Wait(WaitKind),
}

#[derive(Resource, Default)]
struct GameplayContext {
    state: GameplayState,
    attr_pick: Option<AttrKind>,
    card_picks: HashMap<CardIdxType, CardTarget>,
}

impl GameplayContext {
    fn reset(&mut self) {
        self.attr_pick = None;
        self.card_picks.clear();
    }
}

#[derive(Component)]
struct PhaseText;

#[derive(Component)]
struct RemoteAttrText(usize);

#[derive(Component)]
struct ErgText(usize);

#[derive(Component)]
struct RollText(usize);

#[derive(Component)]
struct AttributeText(usize, usize);

#[derive(Component, Clone)]
struct CardLayout {
    slot: usize,
    title: Entity,
    cost: Entity,
    delay: Entity,
    launch: Entity,
    run: Entity,
}

impl CardLayout {
    fn new(slot: usize) -> Self {
        Self {
            slot,
            title: Entity::PLACEHOLDER,
            cost: Entity::PLACEHOLDER,
            delay: Entity::PLACEHOLDER,
            launch: Entity::PLACEHOLDER,
            run: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Component)]
struct CardText;

#[derive(Component)]
struct MachineQueueItem(DelayType);

#[derive(Component)]
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

#[derive(Event)]
enum UiEvent {
    GamePhase(GameplayState),
    PlayerState(PlayerStatePlayerView),
    ChooseAttr(Option<AttrKind>),
    Roll([ErgType; 4]),
    Resources([ErgType; 4], [ErgType; 4], [BuildValueType; 4]),
    MachineUpdate(GameMachinePlayerView, GameMachinePlayerView),
}

#[derive(Component)]
struct AttributeRow(AttrKind);

fn button_events<A, B, C>(click: impl IntoSystem<(), (), A>, over: impl IntoSystem<(), (), B>, out: impl IntoSystem<(), (), C>) -> impl Bundle {
    (
        //
        On::<Pointer<Click>>::run(click),
        On::<Pointer<Over>>::run(over),
        On::<Pointer<Out>>::run(out),
    )
}

fn gameplay_enter(
    // bevy system
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut am: ResMut<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    am.load_atlas("atlas/gameplay", &asset_server, &mut texture_atlas_layouts).unwrap_or_default();

    let layout = slm.build(&mut commands, "gameplay", &am, &asset_server, &mut meshes, &mut materials);

    const LOCAL_ATTR: [[&str; 4]; 4] = [["aa", "ab", "ac", "ad"], ["ba", "bb", "bc", "bd"], ["ca", "cb", "cc", "cd"], ["da", "db", "dc", "dd"]];

    for (row_idx, row) in LOCAL_ATTR.iter().enumerate() {
        for (col_idx, name) in row.iter().enumerate() {
            layout.decorate(&mut commands, name, AttributeText(row_idx, col_idx));
        }
    }

    const ROLL: [&str; 4] = ["ea", "eb", "ec", "ed"];

    for (roll_idx, roll) in ROLL.iter().enumerate() {
        layout.decorate(&mut commands, roll, RollText(roll_idx));
    }

    const REMOTE_ATTR: [&str; 4] = ["ra", "rb", "rc", "rd"];

    for (remote_idx, remote) in REMOTE_ATTR.iter().enumerate() {
        layout.decorate(&mut commands, remote, RemoteAttrText(remote_idx));
    }

    const ERG: [&str; 4] = ["la", "lb", "lc", "ld"];

    for (erg_idx, erg) in ERG.iter().enumerate() {
        layout.decorate(&mut commands, erg, ErgText(erg_idx));
    }

    layout.decorate(&mut commands, "phase", PhaseText);

    layout.decorate(&mut commands, "next", button_events(on_click_next, on_over_next, on_out_next));

    layout.decorate(&mut commands, "row_a", (AttributeRow(AttrKind::Analyze), button_events(on_click_attr, on_over_attr, on_out_attr)));
    layout.decorate(&mut commands, "row_b", (AttributeRow(AttrKind::Breach), button_events(on_click_attr, on_over_attr, on_out_attr)));
    layout.decorate(&mut commands, "row_c", (AttributeRow(AttrKind::Compute), button_events(on_click_attr, on_over_attr, on_out_attr)));
    layout.decorate(&mut commands, "row_d", (AttributeRow(AttrKind::Disrupt), button_events(on_click_attr, on_over_attr, on_out_attr)));

    const MACHINES: [(MachineKind, &str); 2] = [(MachineKind::Local, "local"), (MachineKind::Remote, "remote")];

    for machine in &MACHINES {
        layout.decorate(&mut commands, &format!("{}/title", machine.1), (machine.0, MachineText(MachineTextKind::Title)));
        layout.decorate(&mut commands, &format!("{}/id", machine.1), (machine.0, MachineText(MachineTextKind::Id)));

        layout.decorate(&mut commands, &format!("{}/free_space", machine.1), (machine.0, MachineText(MachineTextKind::Stat(0))));
        layout.decorate(&mut commands, &format!("{}/thermal_capacity", machine.1), (machine.0, MachineText(MachineTextKind::Stat(1))));
        layout.decorate(&mut commands, &format!("{}/system_health", machine.1), (machine.0, MachineText(MachineTextKind::Stat(2))));
        layout.decorate(&mut commands, &format!("{}/open_ports", machine.1), (machine.0, MachineText(MachineTextKind::Stat(3))));

        layout.decorate(&mut commands, &format!("{}/current_program", machine.1), (machine.0, MachineText(MachineTextKind::CurrentProgram)));

        layout.decorate(&mut commands, &format!("{}/running1", machine.1), (machine.0, MachineText(MachineTextKind::Process(0))));
        layout.decorate(&mut commands, &format!("{}/running2", machine.1), (machine.0, MachineText(MachineTextKind::Process(1))));
        layout.decorate(&mut commands, &format!("{}/running3", machine.1), (machine.0, MachineText(MachineTextKind::Process(2))));
        layout.decorate(&mut commands, &format!("{}/running4", machine.1), (machine.0, MachineText(MachineTextKind::Process(3))));
    }

    for card_index in 1..=5 {
        let mut card_layout = CardLayout::new(card_index);
        card_layout.title = layout.decorate(&mut commands, &format!("card{}/title", card_index), CardText);
        card_layout.cost = layout.decorate(&mut commands, &format!("card{}/cost", card_index), CardText);
        card_layout.delay = layout.decorate(&mut commands, &format!("card{}/delay", card_index), CardText);
        card_layout.launch = layout.decorate(&mut commands, &format!("card{}/launch", card_index), CardText);
        card_layout.run = layout.decorate(&mut commands, &format!("card{}/run", card_index), CardText);
        layout.decorate(&mut commands, &format!("card{}", card_index), card_layout);
    }

    commands.insert_resource(GameplayContext::default());
}

fn on_click_next(mut context: ResMut<GameplayContext>, gate: Res<GateIFace>) {
    match context.state {
        GameplayState::Start => gate.send_game_start_turn(),
        GameplayState::Pick => {
            if let Some(kind) = context.attr_pick {
                gate.send_game_choose_attr(kind);
            } else {
                return;
            }
        }
        GameplayState::Play => gate.send_game_play_cards(&context.card_picks),
        GameplayState::Draw => gate.send_game_end_turn(),
        GameplayState::Wait(_) => return,
    };
    context.state = GameplayState::Wait(WaitKind::One);
}

fn on_over_next(event: Res<ListenerInput<Pointer<Over>>>, context: Res<GameplayContext>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target()) {
        sprite.color = match context.state {
            GameplayState::Pick => {
                if context.attr_pick.is_some() {
                    bevy::color::palettes::basic::GREEN
                } else {
                    bevy::color::palettes::basic::RED
                }
            }
            GameplayState::Wait(WaitKind::One) => bevy::color::palettes::basic::RED,
            GameplayState::Wait(WaitKind::All) => bevy::color::palettes::basic::YELLOW,
            _ => bevy::color::palettes::basic::GREEN,
        }
        .into();
    }
}

fn on_out_next(event: Res<ListenerInput<Pointer<Out>>>, mut sprite_q: Query<&mut Sprite>) {
    if let Ok(mut sprite) = sprite_q.get_mut(event.target()) {
        sprite.color = bevy::color::palettes::css::DARK_GRAY.into();
    }
}

fn on_click_attr(event: Res<ListenerInput<Pointer<Click>>>, mut send: EventWriter<UiEvent>, attr_q: Query<&AttributeRow>) {
    if let Ok(attr) = attr_q.get(event.target()) {
        send.send(UiEvent::ChooseAttr(Some(attr.0)));
    }
}

fn on_over_attr(event: Res<ListenerInput<Pointer<Over>>>, context: Res<GameplayContext>, mut sprite_q: Query<(&mut Sprite, &mut Transform)>) {
    if let Ok((mut sprite, mut transform)) = sprite_q.get_mut(event.target()) {
        if transform.translation.z < 100.0 {
            transform.translation.z += 100.0;
        }
        sprite.color = if GameplayState::Pick == context.state {
            bevy::color::palettes::basic::GREEN
        } else {
            bevy::color::palettes::css::SILVER
        }
        .into();
    }
}

fn on_out_attr(event: Res<ListenerInput<Pointer<Out>>>, mut sprite_q: Query<(&mut Sprite, &mut Transform)>) {
    if let Ok((mut sprite, mut transform)) = sprite_q.get_mut(event.target()) {
        if transform.translation.z > 100.0 {
            transform.translation.z -= 100.0;
        }
        sprite.color = bevy::color::palettes::css::DARK_GRAY.into();
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
    mut roll_q: Query<(&mut Text, &RollText)>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::Roll(roll) => {
                for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                    roll_text.sections[0].value = format!("{}", roll[*index]);
                    roll_text.sections[0].style.color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(local_erg, remote_erg, _) => {
                for (mut roll_text, RollText(index)) in roll_q.iter_mut() {
                    roll_text.sections[0].style.color = match local_erg[*index].cmp(&remote_erg[*index]) {
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
    mut receive: EventReader<UiEvent>,
    layout_q: Query<&CardLayout>,
    mut text_q: Query<&mut Text, With<CardText>>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for layout in &layout_q {
                let card = player_state.hand.get(layout.slot).and_then(|o| dm.convert_card(o));

                if let Ok(mut title_text) = text_q.get_mut(layout.title) {
                    title_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.title.clone());
                }
                if let Ok(mut cost_text) = text_q.get_mut(layout.cost) {
                    cost_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.cost.to_string());
                }
                if let Ok(mut launch_text) = text_q.get_mut(layout.launch) {
                    launch_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.launch_rules.clone());
                }
                if let Ok(mut run_text) = text_q.get_mut(layout.run) {
                    run_text.sections[0].value = card.as_ref().map_or("<Empty>".to_string(), |o| o.run_rules.clone());
                }
            }
        }
    }
}

fn erg_ui_update(mut receive: EventReader<UiEvent>, mut erg_q: Query<(&mut Text, &ErgText)>) {
    for ui_event in receive.read() {
        if let UiEvent::PlayerState(player_state) = ui_event {
            for (mut erg_text, ErgText(index)) in erg_q.iter_mut() {
                erg_text.sections[0].value = format!("{:02}", player_state.erg[*index])
            }
        }
    }
}

fn phase_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<&mut Text, With<PhaseText>>,
) {
    for ui_event in receive.read() {
        if let UiEvent::GamePhase(phase) = ui_event {
            let mut text = text_q.single_mut();
            text.sections[0].value = match phase {
                GameplayState::Start => "Start Turn".to_string(),
                GameplayState::Pick => "Pick Attribute".to_string(),
                GameplayState::Play => "Play Card".to_string(),
                GameplayState::Draw => "Draw Cards".to_string(),
                GameplayState::Wait(WaitKind::One) => "...".to_string(),
                GameplayState::Wait(WaitKind::All) => "(Waiting)".to_string(),
            };
        }
    }
}

fn local_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<(&mut Text, &AttributeText)>,
    mut context: ResMut<GameplayContext>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::PlayerState(player_state) => {
                for (mut attr_text, AttributeText(row, col)) in text_q.iter_mut() {
                    attr_text.sections[0].value = format!("{}", player_state.attr[*row][*col]);
                }
            }
            UiEvent::ChooseAttr(kind) => {
                if context.state != GameplayState::Pick {
                    continue;
                }

                for (mut attr_text, AttributeText(row, _)) in text_q.iter_mut() {
                    attr_text.sections[0].style.color = if let Some(kind) = kind {
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
            _ => {}
        }
    }
}

fn remote_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<(&mut Text, &RemoteAttrText)>,
) {
    for ui_event in receive.read() {
        match ui_event {
            UiEvent::Roll(_) => {
                for (mut attr_text, RemoteAttrText(_)) in text_q.iter_mut() {
                    attr_text.sections[0].value = "?".to_string();
                    attr_text.sections[0].style.color = bevy::color::palettes::basic::GRAY.into();
                }
            }
            UiEvent::Resources(_, _, remote_attr) => {
                for (mut attr_text, RemoteAttrText(index)) in text_q.iter_mut() {
                    attr_text.sections[0].value = remote_attr[*index].to_string();
                    attr_text.sections[0].style.color = bevy::color::palettes::basic::RED.into();
                }
            }
            _ => {}
        }
    }
}

fn machine_ui_update(
    // bevy system
    mut receive: EventReader<UiEvent>,
    mut text_q: Query<(&MachineKind, &mut Text, &MachineText)>,
    mut sprite_q: Query<(&MachineKind, &mut Sprite, &MachineQueueItem)>,
    dm: Res<DataManager>,
) {
    for ui_event in receive.read() {
        if let UiEvent::MachineUpdate(local, remote) = ui_event {
            for (machine_component, mut text, MachineText(kind)) in text_q.iter_mut() {
                if let MachineTextKind::Stat(index) = kind {
                    let machine = if *machine_component == MachineKind::Local {
                        local
                    } else {
                        remote
                    };
                    text.sections[0].value = machine.stats[*index].to_string();
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
                    if let Some(current) = machine.queue.iter().find(|item| item.delay == 0) {
                        if let Some(card) = dm.convert_card(&current.player_card) {
                            result = format!(" v- {}", card.title);
                        }
                    }
                    text.sections[0].value = result;
                }
            }

            for (machine_component, mut sprite, MachineQueueItem(index)) in sprite_q.iter_mut() {
                let machine = if *machine_component == MachineKind::Local {
                    local
                } else {
                    remote
                };
                sprite.color = if let Some(process) = machine.queue.iter().find(|item| item.delay == *index) {
                    if process.local {
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
                    text.sections[0].value = result;
                }
            }
        }
    }
}

fn gameplay_update(
    // bevy system
    mut gate: ResMut<GateIFace>,
    mut context: ResMut<GameplayContext>,
    mut send: EventWriter<UiEvent>,
) {
    match gate.grx.try_recv() {
        Ok(GateCommand::GameStartTurn(gate_response)) => {
            println!(
                "[RECV] GameStartTurn {}",
                if gate_response.success {
                    "OK"
                } else {
                    "ERROR"
                }
            );
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameRoll(gate_response)) => {
            println!("[RECV] GameRoll => Pick");
            context.state = GameplayState::Pick;
            send.send(UiEvent::Roll(gate_response.roll));
            send.send(UiEvent::ChooseAttr(None));
        }
        Ok(GateCommand::GameChooseAttr(gate_response)) => {
            println!(
                "[RECV] GameChooseAttr {}",
                if gate_response.success {
                    "OK"
                } else {
                    "ERROR"
                }
            );
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResources(gate_response)) => {
            println!("[RECV] GameResources => Play");
            send.send(UiEvent::PlayerState(gate_response.player_state_view));
            send.send(UiEvent::Resources(gate_response.local_erg, gate_response.remote_erg, gate_response.remote_attr));
            context.state = GameplayState::Play;
        }
        Ok(GateCommand::GamePlayCard(gate_response)) => {
            let success = gate_response.success.iter().all(|&success| success);
            println!(
                "[RECV] GamePlayCard {}",
                if success {
                    "OK"
                } else {
                    "ERROR"
                }
            );
            if success {
                context.card_picks.clear();
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameResolveCards(_gate_response)) => {
            println!("[RECV] GameResolveCards => Draw");
            context.state = GameplayState::Draw;
        }
        Ok(GateCommand::GameEndTurn(gate_response)) => {
            println!(
                "[RECV] GameEndTurn {}",
                if gate_response.success {
                    "OK"
                } else {
                    "ERROR"
                }
            );
            if gate_response.success {
                context.state = GameplayState::Wait(WaitKind::All);
            }
        }
        Ok(GateCommand::GameTick(_gate_response)) => {
            println!("[RECV] GameTick");
            context.reset();
            context.state = GameplayState::Start;
        }
        Ok(GateCommand::GameEndGame(gate_response)) => {
            println!(
                "[RECV] GameEndGame {}",
                if gate_response.success {
                    "OK"
                } else {
                    "ERROR"
                }
            );
        }
        Ok(GateCommand::GameUpdateState(gate_response)) => {
            println!("[RECV] GameUpdateState");
            send.send(UiEvent::PlayerState(gate_response.player_state));
            send.send(UiEvent::MachineUpdate(gate_response.local_machine, gate_response.remote_machine));
        }
        Ok(_) => return,
        Err(_) => return,
    }
    send.send(UiEvent::GamePhase(context.state.clone()));
}

pub fn gameplay_exit(
    // bevy system
    mut commands: Commands,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    commands.remove_resource::<GameplayContext>();
    slm.destroy(commands, "gameplay");
}
