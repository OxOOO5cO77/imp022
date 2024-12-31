use std::mem::discriminant;

use bevy::prelude::*;
use tokio::sync::mpsc;

use crate::manager::{NetworkManager, ScreenLayoutManager};
use crate::network::client_gate::{GateClient, GateCommand, GateIFace};
use crate::screen::login_drawbridge::DrawbridgeHandoff;
use crate::screen::shared::AppScreenExt;
use crate::system::AppState;

const SCREEN_LAYOUT: &str = "login";

pub struct LoginGatePlugin;

impl Plugin for LoginGatePlugin {
    //noinspection Duplicates
    fn build(&self, app: &mut App) {
        app //
            .add_screen(AppState::LoginGate)
            .with_enter(gate_enter)
            .with_update(gate_update)
            .with_exit(login_gate_exit);
    }
}

#[derive(Component)]
struct ConnectedIcon;

fn gate_enter(
    // bevy system
    mut commands: Commands,
    handoff: Res<DrawbridgeHandoff>,
    mut net: ResMut<NetworkManager>,
) {
    let (gtx, to_gate_rx) = mpsc::unbounded_channel();
    let (from_gate_tx, grx) = mpsc::unbounded_channel();
    let gate = GateIFace {
        game_id: 0,
        auth: handoff.auth,
        gtx,
        grx,
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

fn login_gate_exit(
    // bevy system
    commands: Commands,
    mut slm: ResMut<ScreenLayoutManager>,
) {
    slm.destroy(commands, SCREEN_LAYOUT);
}
