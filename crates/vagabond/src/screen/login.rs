use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use shared_data::types::AuthType;
use std::mem::discriminant;
use tokio::sync::mpsc;

use crate::manager::NetworkManager;
use crate::network::client_drawbridge;
use crate::network::client_drawbridge::{AuthInfo, DrawbridgeClient, DrawbridgeIFace};
use crate::network::client_gate::{GateClient, GateCommand, GateIFace};
use crate::system::app_state::AppState;

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppState::LoginDrawbridge), drawbridge_enter)
            .add_systems(Update, drawbridge_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(Update, login_ui_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(OnExit(AppState::LoginGate), drawbridge_exit)
            .add_systems(OnEnter(AppState::LoginGate), gate_enter)
            .add_systems(Update, gate_update.run_if(in_state(AppState::LoginGate)));
    }
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

fn login_ui_update(
    // bevy system
    egui_context: EguiContexts,
    mut drawbridge: ResMut<DrawbridgeIFace>,
) {
    egui::Window::new("Login").show(egui_context.ctx(), |ui| {
        ui.label("User");
        let username = ui.add(egui::TextEdit::singleline(&mut drawbridge.username));
        let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
        let mut password_focus = false;
        if username.lost_focus() && enter_pressed {
            password_focus = true;
        }
        ui.label("Password");
        let password = ui.add(egui::TextEdit::singleline(&mut drawbridge.password).password(true));
        if password.lost_focus() && enter_pressed && !drawbridge.username.is_empty() && !drawbridge.password.is_empty() {
            client_drawbridge::send_authorize(&drawbridge.dtx, drawbridge.username.clone(), drawbridge.password.clone());
        }
        if password_focus {
            password.request_focus();
        }
    });
}

fn drawbridge_enter(
    // bevy system
    mut commands: Commands,
    mut net: ResMut<NetworkManager>,
) {
    let (to_drawbridge_tx, to_drawbridge_rx) = mpsc::unbounded_channel();
    let (from_drawbridge_tx, from_drawbridge_rx) = mpsc::unbounded_channel();
    let drawbridge = DrawbridgeIFace {
        username: "".to_string(),
        password: "".to_string(),
        dtx: to_drawbridge_tx,
        drx: from_drawbridge_rx,
    };

    commands.insert_resource(drawbridge);

    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = DrawbridgeClient::start("[::1]:23450".to_string(), from_drawbridge_tx, to_drawbridge_rx, &net.runtime);
}

fn drawbridge_update(
    // bevy system
    mut app_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    mut drawbridge: ResMut<DrawbridgeIFace>,
) {
    if let Ok(auth_info) = drawbridge.drx.try_recv() {
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
) {
    if let Ok(gate_command) = gate.grx.try_recv() {
        match gate_command {
            GateCommand::Hello => app_state.set(AppState::ComposeInit),
            _ => println!("[Login] Unexpected command received {:?}", discriminant(&gate_command)),
        }
    }
}
