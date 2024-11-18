use crate::manager::data_manager::DataManager;
use crate::manager::player_builder::PlayerBuilder;
use gate::message::gate_header::GateHeader;
use hall::data::game::{GameMachinePlayerView, GamePhase, GameStage, GameState, GameUser};
use hall::data::player::player_state::PlayerStatePlayerView;
use hall::message::*;
use rand::prelude::*;
use shared_data::types::{AuthType, GameIdType, NodeType, UserIdType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use std::collections::HashMap;
use std::env;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) mod manager;

struct Broadcaster {
    local_tx: UnboundedSender<RoutedMessage>,
    gate_map: HashMap<UserIdType, (NodeType, NodeType)>,
}

impl Broadcaster {
    pub(crate) fn broadcast<T: CommandMessage>(&mut self, message: T) {
        let mut errors = vec![];
        for (id, (gate, vagabond)) in &self.gate_map {
            let route = op::Route::One(*gate);
            let command = T::COMMAND;

            let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + message.size_in_buffer());

            out.push(&route);
            out.push(&command);
            out.push(vagabond);
            out.push(&message);

            let result = self.local_tx.send(RoutedMessage {
                route: op::Route::None,
                buf: out,
            });
            if result.is_err() {
                errors.push(*id);
            }
        }

        for id in &errors {
            self.gate_map.remove(id);
        }
    }

    pub(crate) fn track(&mut self, id: UserIdType, target: (NodeType, NodeType)) {
        self.gate_map.insert(id, target);
    }
}

struct Hall {
    games: HashMap<GameIdType, GameState>,
    data_manager: DataManager,
    bx: Broadcaster,
}

impl Hall {
    fn split_borrow(&mut self) -> (&mut HashMap<GameIdType, GameState>, &mut Broadcaster) {
        (&mut self.games, &mut self.bx)
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Hall] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (local_tx, local_rx) = mpsc::unbounded_channel();

    let context = Hall {
        games: HashMap::new(),
        data_manager: DataManager::new().expect("[Hall] Unable to initialize DataManager"),
        bx: Broadcaster {
            local_tx: local_tx.clone(),
            gate_map: HashMap::new(),
        },
    };
    let context = Arc::new(Mutex::new(context));

    let courtyard_client = shared_net::async_client(context, op::Flavor::Hall, local_tx, local_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Hall] END");

    result
}

fn process_courtyard(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::GameActivate => recv_game_start(context, tx, buf),
        op::Command::GameBuild => recv_game_build(context, tx, buf),
        op::Command::GameStartTurn => recv_game_start_turn(context, tx, buf),
        op::Command::GameChooseAttr => recv_game_choose_attr(context, tx, buf),
        op::Command::GamePlayCard => recv_game_play_card(context, tx, buf),
        op::Command::GameEndTurn => recv_game_end_turn(context, tx, buf),
        op::Command::GameEndGame => recv_game_end(context, tx, buf),
        _ => {}
    };
    VClientMode::Continue
}

fn send_routed_message<T: CommandMessage>(message: T, gate: NodeType, vagabond: NodeType, tx: &UnboundedSender<RoutedMessage>) -> Result<(), SendError<RoutedMessage>> {
    let route = op::Route::One(gate);
    let command = T::COMMAND;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + message.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&vagabond);
    out.push(&message);

    tx.send(RoutedMessage {
        route: op::Route::None,
        buf: out,
    })
}

fn recv_game_start(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameActivateRequest>();

    let mut user = GameUser::new(header.auth);

    let temp_builder = PlayerBuilder::new(&user.parts, &context.data_manager);
    user.parts.clear();

    let mut rng = thread_rng();
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

    let parts = [temp_builder.access.convert_to_player_part(), temp_builder.breach.convert_to_player_part(), temp_builder.compute.convert_to_player_part(), temp_builder.disrupt.convert_to_player_part(), temp_builder.build.convert_to_player_part(), temp_builder.build_values.convert_to_player_part(), temp_builder.detail.convert_to_player_part(), temp_builder.detail_values.convert_to_player_part()];

    let response = GameActivateResponse {
        game_id,
        parts,
    };
    let _ = send_routed_message(response, gate, header.vagabond, &tx);
}

fn recv_game_build(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameBuildRequest>();

    let builder = PlayerBuilder::new(&request.parts, &context.data_manager);
    let player = builder.create_player(&context.data_manager);
    let response = if let Some(player) = player.as_ref() {
        GameBuildResponse {
            seed: player.seed,
            deck: player.deck.iter().cloned().collect(),
        }
    } else {
        GameBuildResponse {
            seed: 0,
            deck: Vec::default(),
        }
    };

    println!("[Hall] Sending build to G({})=>V({})", gate, header.vagabond);

    let _ = send_routed_message(response, gate, header.vagabond, &tx);

    if request.commit {
        let context = context.deref_mut();
        let (games, dm) = (&mut context.games, &context.data_manager);

        if let Some(game) = games.get_mut(&request.game_id) {
            let mut rng = game.rng.clone();
            if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, header.user, header.auth) {
                user.player = player;
                if let Some(valid_player) = user.player.as_ref() {
                    user.state.set_attr(valid_player.attributes);
                    user.state.setup_deck(valid_player.deck.iter().filter_map(|card| dm.lookup_player_card(card)).collect(), &mut rng);
                    let remote = GameState::split_get_remote(&mut game.remotes, user.remote.unwrap_or_default()).unwrap();
                    let message = GameUpdateStateMessage {
                        player_state: PlayerStatePlayerView::from(&user.state),
                        local_machine: GameMachinePlayerView::from(&user.machine),
                        remote_machine: GameMachinePlayerView::from(&remote.machine),
                    };
                    let _ = send_routed_message(message, gate, header.vagabond, &tx);
                }
            }
        }
    }

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_have_players() {
            game.set_phase(GamePhase::TurnStart);
            let message = GameStartGameMessage {
                success: true,
            };
            context.bx.broadcast(message);
        }
    }
}

fn recv_game_end(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let game_id = buf.pull::<GameIdType>();

    if let Some(game) = context.games.get_mut(&game_id) {
        if game.is_empty() {
            context.games.remove(&game_id);
        }
    }

    let response = GameEndGameResponse {
        success: true,
    };

    let _ = send_routed_message(response, gate, header.vagabond, &tx);
}

fn update_user(context: &mut Hall, game_id: GameIdType, user: UserIdType, auth: AuthType, command: op::Command, update: impl Fn(&mut GameUser) -> bool) -> bool {
    if let Some(game) = context.games.get_mut(&game_id) {
        if let Some(user) = GameState::split_get_user_auth_mut(&mut game.users, user, auth) {
            user.state.last_command = Some(command);
            return update(user);
        }
    }
    false
}

fn recv_game_start_turn(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameStartTurnRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameStartTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameStartTurnResponse {
        success,
    };

    let _ = send_routed_message(response, gate, header.vagabond, &tx);

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameStartTurn) {
            game.roll();
            let message = GameRollMessage {
                roll: game.erg_roll,
            };
            game.set_phase(GamePhase::ChooseAttr);
            context.bx.broadcast(message);
        }
    }
}

fn recv_game_choose_attr(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameChooseAttrRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameChooseAttr, |user| {
        user.state.resolve_kind = Some(request.attr.into());
        true
    });

    let response = GameChooseAttrResponse {
        success,
    };

    let _ = send_routed_message(response, gate, header.vagabond, &tx);

    let (games, bx) = context.split_borrow();

    if let Some(game) = games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameChooseAttr) {
            let mut rng = thread_rng();
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
                        if let Some((user_gate, user_vagabond)) = bx.gate_map.get(id) {
                            let _ = send_routed_message(message, *user_gate, *user_vagabond, &bx.local_tx);
                        }
                    }
                }
            }
            game.set_phase(GamePhase::CardPlay);
        }
    }
}

fn recv_game_play_card(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GamePlayCardRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GamePlayCard, |user| user.state.play_card(request.card_idx as usize));

    let response = GamePlayCardResponse {
        success,
    };

    let _ = send_routed_message(response, gate, header.vagabond, &tx);

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GamePlayCard) {
            let message = GameResolveCardsMessage {
                success: true,
            };
            game.set_phase(GamePhase::TurnEnd);
            context.bx.broadcast(message);
        }
    }
}

fn recv_game_end_turn(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameEndTurnRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameEndTurn, |_user| {
        /*TODO:*/
        true
    });

    let response = GameEndTurnResponse {
        success,
    };

    let _ = send_routed_message(response, gate, header.vagabond, &tx);

    // game.set_phase(GamePhase::TurnStart);
    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameEndTurn) {
            let message = GameTickMessage {
                success: true,
            };

            for remote in game.remotes.values_mut() {
                remote.reset();
            }

            game.set_phase(GamePhase::TurnStart);
            context.bx.broadcast(message);
        }
    }
}
