use rand::prelude::{IteratorRandom, StdRng};
use rand::{RngExt, SeedableRng};
use std::collections::HashSet;
use tracing::info;

use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::{GameSubCommand, MissionNodeState, Stage};
use hall_lib::message::{GameActivateRequest, GameActivateResponse};
use shared_net::NodeType;

use crate::HallContext;
use crate::game::{GameMission, GameState, GameUser, GameUserMissionState};
use crate::manager::player_builder::PlayerBuilder;

pub(crate) fn recv_game_activate(context: &HallContext, request: GameActivateRequest, gate: NodeType, header: GateHeader) -> Option<GameActivateResponse> {
    // todo: check for existing user
    let mut user = GameUser::new(header.auth);

    let dm = context.data_manager.read().ok()?;
    let temp_builder = PlayerBuilder::new(&user.parts, &dm);
    user.parts.clear();

    let mut game_id_rng = rand::rng();
    let mut game_id = request.game_id;

    {
        let games = context.games.read().ok()?;
        while game_id == 0 {
            let new_id = game_id_rng.random();
            if !games.contains_key(&new_id) {
                game_id = new_id;
            }
        }
    }

    {
        let mut rng: StdRng = SeedableRng::seed_from_u64(game_id);
        let mut games = context.games.write().ok()?;
        let game = games.entry(game_id).or_insert_with(|| {
            let institution = dm.pick_institution(&mut rng).unwrap();
            let mission = GameMission::generate(institution, game_id);
            GameState::new(mission, &mut rng)
        });

        let known_nodes = game.mission.node.iter().filter(|n| matches!(n.initial_state, MissionNodeState::Known)).map(|n| n.id).collect::<HashSet<_>>();
        let initial_node = *known_nodes.iter().choose(&mut rng)?;

        user.mission_state = GameUserMissionState::new(initial_node, known_nodes);
        user.state.command.should_be(GameSubCommand::Build);
        game.user_add(header.user, user);
        game.set_stage(Stage::Building);
    }

    context.bx.write().ok()?.track(header.user, (gate, header.vagabond));

    info!(game_id, "Sending parts to G({})=>V({})", gate, header.vagabond);

    let parts = [
        //
        temp_builder.access.convert_to_player_part(),
        temp_builder.breach.convert_to_player_part(),
        temp_builder.compute.convert_to_player_part(),
        temp_builder.disrupt.convert_to_player_part(),
        temp_builder.build.convert_to_player_part(),
        temp_builder.build_values.convert_to_player_part(),
        temp_builder.detail.convert_to_player_part(),
        temp_builder.detail_values.convert_to_player_part(),
    ];

    let response = GameActivateResponse {
        game_id,
        parts,
    };
    Some(response)
}
