use crate::logic::shared::update_user;
use crate::network::util::send_routed_message;
use crate::HallContext;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GamePhase, GameState};
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::{GameChooseAttrRequest, GameChooseAttrResponse, GameResourcesMessage};
use shared_net::types::NodeType;
use shared_net::{op, RoutedMessage, VSizedBuffer};
use tokio::sync::mpsc::UnboundedSender;

pub(crate) fn recv_game_choose_attr(context: HallContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameChooseAttrRequest>();

    let (games, _, bx) = context.split_borrow();

    let success = update_user(games, request.game_id, header.user, header.auth, op::Command::GameChooseAttr, |user| {
        user.state.resolve_kind = Some(request.attr.into());
        true
    });

    let response = GameChooseAttrResponse {
        success,
    };

    let _ = send_routed_message(&response, gate, header.vagabond, &tx);

    if let Some(game) = games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameChooseAttr) {
            let mut rng = rand::rng();
            let (erg_roll, users, remotes) = game.split_borrow_for_resolve();
            for (id, user) in users.iter_mut() {
                if let Some(player) = &user.player {
                    if let Some(kind) = user.state.resolve_kind {
                        let remote = remotes.get(&user.remote.unwrap()).unwrap();
                        let remote_attr = remote.choose_attr(&mut rng);
                        let (local_erg, remote_erg) = GameState::resolve_matchups(erg_roll, &player.attributes.get(kind), &remote_attr);
                        user.state.add_erg(kind, local_erg);

                        let player_state_view = PlayerStatePlayerView::from(&user.state);
                        let message = GameResourcesMessage {
                            player_state_view,
                            remote_attr,
                            local_erg,
                            remote_erg,
                        };

                        bx.send_to_user(id, &message);
                    }
                }
            }
            game.set_phase(GamePhase::CardPlay, op::Command::GamePlayCard);
        }
    }
}
