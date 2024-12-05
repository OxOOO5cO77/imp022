use crate::manager::player_builder::PlayerBuilder;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GameStage, GameState, GameUser};
use hall::message::{GameActivateRequest, GameActivateResponse};
use rand::Rng;
use shared_net::op;
use shared_net::types::{GameIdType, NodeType};

pub(crate) fn recv_game_activate(context: &HallContext, request: GameActivateRequest, gate: NodeType, header: GateHeader) -> Option<GameActivateResponse> {
    // todo: check for existing user

    let mut user = GameUser::new(header.auth);

    let temp_builder = PlayerBuilder::new(&user.parts, &context.data_manager.read().unwrap());
    user.parts.clear();

    let mut rng = rand::rng();
    let mut game_id = request.game_id;
    {
        let games = context.games.read().unwrap();
        while game_id == 0 {
            let new_id = rng.random::<GameIdType>();
            if !games.contains_key(&new_id) {
                game_id = new_id;
            }
        }
    }

    {
        let mut games = context.games.write().unwrap();
        let game = games.entry(game_id).or_insert(GameState::new(4, &mut rng));
        user.remote = game.pick_remote(&mut rng);
        user.state.command.should_be(op::Command::GameBuild);
        game.user_add(header.user, user);
        game.set_stage(GameStage::Building);
    }

    context.bx.write().unwrap().track(header.user, (gate, header.vagabond));

    println!("[Hall] [{:X}] Sending parts to G({})=>V({})", game_id, gate, header.vagabond);

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
