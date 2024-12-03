use crate::logic::shared::update_user;
use crate::manager::player_builder::PlayerBuilder;
use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GamePhase, GameState};
use hall::message::{GameBuildRequest, GameBuildResponse, GameStartGameMessage};
use shared_net::types::NodeType;
use shared_net::{op, RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_build(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameBuildRequest>();

    let (games, dm, _) = context.split_borrow();

    let player = update_user(games, request.game_id, header.user, header.auth, op::Command::GameBuild, |_user| {
        let builder = PlayerBuilder::new(&request.parts, &dm);
        builder.create_player(&dm)
    });

    let response = if let Some(player) = player.as_ref() {
        GameBuildResponse {
            seed: player.seed,
            deck: player.deck.iter().cloned().collect(),
        }
    } else {
        GameBuildResponse::default()
    };

    println!("[Hall] Sending build to G({})=>V({})", gate, header.vagabond);

    let _ = send_routed_message(&response, gate, header.vagabond, &tx);

    if request.commit && response.seed != 0 {
        if let Some(game) = games.get_mut(&request.game_id) {
            let mut rng = game.rng.clone();
            if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, header.user, header.auth) {
                user.player = player;
                if let Some(valid_player) = user.player.as_ref() {
                    user.state.set_attr(valid_player.attributes);
                    user.state.setup_deck(valid_player.deck.iter().filter_map(|card| dm.lookup_player_card(card)).collect(), &mut rng);
                }
            }
        }

        if let Some(game) = context.games.get_mut(&request.game_id) {
            if game.all_users_have_players() {
                game.set_phase(GamePhase::TurnStart, op::Command::GameStartTurn);
                let message = GameStartGameMessage {
                    success: true,
                };
                context.bx.broadcast(message);
            }
        }
    }
}
