use crate::logic::shared::update_user;
use crate::manager::player_builder::PlayerBuilder;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::GameState;
use hall::message::{GameBuildRequest, GameBuildResponse};
use shared_net::op;
use shared_net::types::NodeType;

pub(crate) fn recv_game_build(context: &HallContext, request: GameBuildRequest, gate: NodeType, header: GateHeader) -> Option<GameBuildResponse> {
    let mut games = context.games.write().unwrap();
    let dm = context.data_manager.read().unwrap();
    let created_player = update_user(&mut games, request.game_id, header.user, header.auth, op::Command::GameBuild, |_user| {
        let builder = PlayerBuilder::new(&request.parts, &dm);
        builder.create_player(&dm)
    });

    let response = created_player.as_ref().map(|player| GameBuildResponse {
        seed: player.seed,
        deck: player.deck.iter().cloned().collect(),
    });

    if request.commit && response.is_some() {
        if let Some(game) = games.get_mut(&request.game_id) {
            let mut rng = game.rng.clone();
            if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, header.user, header.auth) {
                user.player = created_player;
                if let Some(valid_player) = user.player.as_ref() {
                    user.state.set_attr(valid_player.attributes);
                    user.state.setup_deck(valid_player.deck.iter().filter_map(|card| dm.lookup_player_card(card)).collect(), &mut rng);
                }
            }
        }
    }

    println!("[Hall] Sending build to G({})=>V({})", gate, header.vagabond);

    response
}
