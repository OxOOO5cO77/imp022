use crate::manager::player_builder::PlayerBuilder;
use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GameStage, GameState, GameUser};
use hall::message::{GameActivateRequest, GameActivateResponse};
use rand::Rng;
use shared_net::types::{GameIdType, NodeType};
use shared_net::{RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_activate(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameActivateRequest>();

    // todo: check for existing user
    
    let mut user = GameUser::new(header.auth);

    let temp_builder = PlayerBuilder::new(&user.parts, &context.data_manager);
    user.parts.clear();

    let mut rng = rand::rng();
    let mut game_id = request.game_id;
    while game_id == 0 {
        let new_id = rng.random::<GameIdType>();
        if !context.games.contains_key(&new_id) {
            game_id = new_id;
        }
    }

    let game = context.games.entry(game_id).or_insert(GameState::new(4, &mut rng));
    user.remote = game.pick_remote();
    game.user_add(header.user, user);
    game.set_stage(GameStage::Building);

    context.bx.track(header.user, (gate, header.vagabond));

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
    let _ = send_routed_message(&response, gate, header.vagabond, &tx);
}
