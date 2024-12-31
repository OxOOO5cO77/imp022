use bevy::prelude::*;

use hall::message::GameUpdateStateResponse;

use crate::network::client_gate::{GateCommand, GateIFace};
use crate::screen::compose_main::ComposeHandoff;
use crate::screen::shared::AppScreenExt;
use crate::system::AppState;

pub struct GameplayInitPlugin;

impl Plugin for GameplayInitPlugin {
    //noinspection Duplicates
    fn build(&self, app: &mut App) {
        app //
            .add_screen(AppState::GameplayInit)
            .with_enter(gameplay_init_enter)
            .with_update(gameplay_init_update);
    }
}

#[derive(Resource)]
pub(crate) struct GameplayInitHandoff {
    pub(crate) initial_response: Option<Box<GameUpdateStateResponse>>,
    pub(crate) name: String,
    pub(crate) id: String,
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
