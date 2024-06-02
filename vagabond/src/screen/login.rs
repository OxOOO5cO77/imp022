use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use shared_net::VRoutedMessage;

use crate::manager::NetworkManager;
use crate::network::client_drawbridge;
use crate::network::client_drawbridge::{AuthInfo, DrawbridgeClient};
use crate::network::client_gate::{Command, GateClient};
use crate::system::app_state::AppState;

pub struct LoginPlugin;

impl Plugin for LoginPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::LoginDrawbridge), drawbridge_enter)
            .add_systems(Update, drawbridge_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(Update, login_ui_update.run_if(in_state(AppState::LoginDrawbridge)))
            .add_systems(OnExit(AppState::LoginGate), drawbridge_exit)
            .add_systems(OnEnter(AppState::LoginGate), gate_enter)
            .add_systems(Update, gate_update.run_if(in_state(AppState::LoginGate)))
        ;
    }
}

#[derive(Resource)]
struct DrawbridgeIFace {
    username: String,
    password: String,
    dtx: UnboundedSender<VRoutedMessage>,
    drx: UnboundedReceiver<AuthInfo>,
}

fn login_ui_update(egui_context: EguiContexts, mut login: ResMut<DrawbridgeIFace>) {
    egui::Window::new("Login").show(egui_context.ctx(), |ui| {
        ui.label("User");
        let username = ui.add(egui::TextEdit::singleline(&mut login.username));
        let enter_pressed = ui.input(|i| i.key_pressed(egui::Key::Enter));
        let mut password_focus = false;
        if username.lost_focus() && enter_pressed {
            password_focus = true;
        }
        ui.label("Password");
        let password = ui.add(egui::TextEdit::singleline(&mut login.password).password(true));
        if password.lost_focus() && enter_pressed && !login.username.is_empty() && !login.password.is_empty() {
            client_drawbridge::send_authorize(login.dtx.clone(), login.username.clone(), login.password.clone());
        }
        if password_focus {
            password.request_focus();
        }
    });
}

fn drawbridge_enter(mut commands: Commands, mut net: ResMut<NetworkManager>) {
    let (to_drawbridge_tx, to_drawbridge_rx) = mpsc::unbounded_channel();
    let (from_drawbridge_tx, from_drawbridge_rx) = mpsc::unbounded_channel();
    let login = DrawbridgeIFace {
        username: "".to_string(),
        password: "".to_string(),
        dtx: to_drawbridge_tx,
        drx: from_drawbridge_rx,
    };

    commands.insert_resource(login);

    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = DrawbridgeClient::start("[::1]:23450".to_string(), from_drawbridge_tx, to_drawbridge_rx, &net.runtime);
}

fn drawbridge_update(mut app_state: ResMut<NextState<AppState>>, mut commands: Commands, mut login1: ResMut<DrawbridgeIFace>) {
    if let Ok(res) = login1.drx.try_recv() {
        commands.insert_resource(res);
        app_state.set(AppState::LoginGate);
    }
}

fn drawbridge_exit(mut commands: Commands) {
    commands.remove_resource::<DrawbridgeIFace>();
}

#[derive(Resource)]
struct GateIFace {
    grx: UnboundedReceiver<Command>,
}

fn gate_enter(mut commands: Commands, authinfo: Res<AuthInfo>, mut net: ResMut<NetworkManager>) {
    let (_to_gate_tx, to_gate_rx) = mpsc::unbounded_channel();
    let (from_gate_tx, from_gate_rx) = mpsc::unbounded_channel();
    let login2 = GateIFace {
        grx: from_gate_rx,
    };

    commands.insert_resource(login2);

    let iface = format!("{}:{}", authinfo.0, authinfo.1);
    if let Some(task) = &net.current_task {
        task.abort();
    }
    net.current_task = GateClient::start(authinfo.2, iface, from_gate_tx, to_gate_rx, &net.runtime);

    commands.remove_resource::<AuthInfo>();
}

fn gate_update(mut app_state: ResMut<NextState<AppState>>, mut login2: ResMut<GateIFace>) {
    if let Ok(cmd) = login2.grx.try_recv() {
        if cmd == Command::Hello {
            app_state.set(AppState::Compose);
        }
    }
}
