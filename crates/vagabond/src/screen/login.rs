use std::mem::discriminant;

use bevy::prelude::*;
use bevy_simple_text_input::{TextInputCursorPos, TextInputInactive, TextInputSubmitEvent, TextInputValue};
use tokio::sync::mpsc;

use shared_net::AuthType;

use crate::gfx::FrameMaterial;
use crate::manager::{AtlasManager, NetworkManager, ScreenLayoutManager};
use crate::network::client_drawbridge;
use crate::network::client_drawbridge::{AuthInfo, DrawbridgeClient, DrawbridgeIFace};
use crate::network::client_gate::{GateClient, GateCommand, GateIFace};
use crate::system::AppState;

const SCREEN_LAYOUT: &str = "login";

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppState::LoginDrawbridge), drawbridge_enter)
            .add_systems(OnEnter(AppState::LoginDrawbridge), login_ui_setup.after(drawbridge_enter))
            .add_systems(Update, drawbridge_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(Update, textedit_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(OnExit(AppState::LoginDrawbridge), drawbridge_exit)
            .add_systems(OnEnter(AppState::LoginGate), gate_enter)
            .add_systems(Update, gate_update.run_if(in_state(AppState::LoginGate)))
            .add_systems(OnExit(AppState::LoginGate), login_exit);
    }
}

#[derive(Component)]
struct LoginScreen;

#[derive(Component)]
struct ConnectedIcon;

#[derive(Resource)]
struct LoginContext {
    username: Entity,
    password: Entity,
}

#[derive(Resource)]
struct DrawbridgeHandoff {
    iface: String,
    auth: AuthType,
}

impl DrawbridgeHandoff {
    fn new(auth_info: AuthInfo) -> Self {
        Self {
            iface: format!("{}:{}", auth_info.ip, auth_info.port),
            auth: auth_info.auth,
        }
    }
}

fn drawbridge_enter(
    // bevy system
    mut commands: Commands,
    mut net: ResMut<NetworkManager>,
) {
    let (to_drawbridge_tx, to_drawbridge_rx) = mpsc::unbounded_channel();
    let (from_drawbridge_tx, from_drawbridge_rx) = mpsc::unbounded_channel();

    let username = std::env::var("VAGABOND_USERNAME").unwrap_or("".to_string());
    let password = std::env::var("VAGABOND_PASSWORD").unwrap_or("".to_string());

    let drawbridge = DrawbridgeIFace {
        username,
        password,
        dtx: to_drawbridge_tx,
        drx: from_drawbridge_rx,
    };

    commands.insert_resource(drawbridge);

    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = DrawbridgeClient::start("[::1]:23450".to_string(), from_drawbridge_tx, to_drawbridge_rx, &net.runtime);
}

trait TextInputExt {
    fn with_text(self, text: &str, active: bool) -> Self;
}

impl TextInputExt for &mut EntityCommands<'_> {
    fn with_text(self, text: &str, active: bool) -> Self {
        self //
            .insert((TextInputInactive(!active), TextInputCursorPos(text.len()), TextInputValue(text.into())))
    }
}

#[allow(clippy::type_complexity)]
fn login_ui_setup(
    // bevy system
    mut commands: Commands,
    drawbridge: Res<DrawbridgeIFace>,
    am: Res<AtlasManager>,
    mut slm: ResMut<ScreenLayoutManager>,
    for_slm: (Res<AssetServer>, ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>, ResMut<Assets<FrameMaterial>>),
) {
    let (layout, _) = slm.build(&mut commands, SCREEN_LAYOUT, &am, for_slm);

    commands.entity(layout.entity("connected_icon")).insert(ConnectedIcon);

    let username = commands.entity(layout.entity("username")).with_text(&drawbridge.username, true).id();
    let password = commands.entity(layout.entity("password")).with_text(&drawbridge.password, false).id();

    let tracker = LoginContext {
        username,
        password,
    };
    commands.insert_resource(tracker)
}

fn textedit_update(
    // bevy system
    mut commands: Commands,
    mut events: EventReader<TextInputSubmitEvent>,
    tracker: Res<LoginContext>,
    mut drawbridge: ResMut<DrawbridgeIFace>,
) {
    for event in events.read() {
        commands.entity(tracker.username).insert(TextInputInactive(true));
        commands.entity(tracker.password).insert(TextInputInactive(false));

        if event.entity == tracker.username {
            drawbridge.username = event.value.clone();
        } else if event.entity == tracker.password {
            drawbridge.password = event.value.clone();

            if !drawbridge.username.is_empty() && !drawbridge.password.is_empty() {
                client_drawbridge::send_authorize(&drawbridge.dtx, drawbridge.username.clone(), drawbridge.password.clone());
            }
        }
    }
}

fn drawbridge_update(
    // bevy system
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut drawbridge: ResMut<DrawbridgeIFace>,
    mut sprite_q: Query<&mut Sprite, With<ConnectedIcon>>,
) {
    if let Ok(auth_info) = drawbridge.drx.try_recv() {
        if let Ok(mut sprite) = sprite_q.get_single_mut() {
            sprite.color = bevy::color::palettes::css::YELLOW.into()
        }
        commands.insert_resource(DrawbridgeHandoff::new(auth_info));
        app_state.set(AppState::LoginGate);
    }
}

fn drawbridge_exit(
    // bevy system
    mut commands: Commands,
) {
    commands.remove_resource::<DrawbridgeIFace>();
}

fn gate_enter(
    // bevy system
    mut commands: Commands,
    handoff: Res<DrawbridgeHandoff>,
    mut net: ResMut<NetworkManager>,
) {
    let (gtx, to_gate_rx) = mpsc::unbounded_channel();
    let (from_gate_tx, from_gate_rx) = mpsc::unbounded_channel();
    let gate = GateIFace {
        game_id: 0,
        auth: handoff.auth,
        gtx,
        grx: from_gate_rx,
    };

    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = GateClient::start(handoff.iface.clone(), from_gate_tx, to_gate_rx, &net.runtime);

    commands.remove_resource::<DrawbridgeHandoff>();
    commands.insert_resource(gate);
}

fn gate_update(
    // bevy system
    mut app_state: ResMut<NextState<AppState>>,
    mut gate: ResMut<GateIFace>,
    mut sprite_q: Query<&mut Sprite, With<ConnectedIcon>>,
) {
    if let Ok(gate_command) = gate.grx.try_recv() {
        if let Ok(mut sprite) = sprite_q.get_single_mut() {
            sprite.color = bevy::color::palettes::css::GREEN.into()
        }
        match gate_command {
            GateCommand::Hello => app_state.set(AppState::ComposeInit),
            _ => println!("[Login] Unexpected command received {:?}", discriminant(&gate_command)),
        }
    }
}

fn login_exit(
    // bevy system
    mut commands: Commands,
    login_q: Query<Entity, With<LoginScreen>>,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    for e in &login_q {
        commands.entity(e).despawn_recursive();
    }
    commands.remove_resource::<LoginContext>();
    slm.destroy(commands, SCREEN_LAYOUT);
}
