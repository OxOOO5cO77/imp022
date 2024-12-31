use bevy::prelude::*;

use vagabond::data::VagabondPart;

use crate::manager::DataManager;
use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::shared::AppScreenExt;
use crate::system::AppState;

pub struct ComposeInitPlugin;

impl Plugin for ComposeInitPlugin {
    //noinspection Duplicates
    fn build(&self, app: &mut App) {
        app //
            .add_screen(AppState::ComposeInit)
            .with_enter(compose_init_enter)
            .with_update(compose_init_update);
    }
}

#[derive(Resource)]
pub(crate) struct ComposeInitHandoff {
    pub(crate) parts: [VagabondPart; 8],
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
