use crate::manager::data_manager::DataManager;
use crate::manager::player_builder::PlayerBuilder;
use gate::message::gate_header::GateHeader;
use hall::data::game::GameState;
use hall::data::game::GameUser;
use hall::message::game_build::{GameBuildRequest, GameBuildResponse};
use hall::message::game_choose_attr::{GameChooseAttrRequest, GameChooseAttrResponse};
use hall::message::game_end_turn::{GameEndTurnRequest, GameEndTurnResponse};
use hall::message::game_play_card::{GamePlayCardRequest, GamePlayCardResponse};
use hall::message::game_start::{GameStartRequest, GameStartResponse};
use hall::message::game_start_turn::{GameStartTurnRequest, GameStartTurnResponse};
use rand::prelude::*;
use shared_data::game::card::CostType;
use shared_data::types::{AuthType, GameIdType, NodeType, UserIdType};
use shared_net::sizedbuffers::Bufferable;
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use std::collections::HashMap;
use std::env;
use std::ops::{DerefMut};
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) mod manager;

struct Broadcaster {
    local_tx: UnboundedSender<RoutedMessage>,
    gate_map: HashMap<UserIdType, (NodeType, NodeType)>,
}

impl Broadcaster {
    pub(crate) fn broadcast(&mut self, command: op::Command, buf: VSizedBuffer) {
        let mut errors = vec![];
        for (id, (gate, vagabond)) in &self.gate_map {
            let route = op::Route::One(*gate);
            let mut msg = buf.clone();

            let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + msg.size());

            out.push(&route);
            out.push(&command);
            out.push(vagabond);
            out.xfer_bytes(&mut msg);

            let result = self.local_tx.send(RoutedMessage { route: op::Route::None, buf: out });
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
        op::Command::GameActivate => c_game_start(context, tx, buf),
        op::Command::GameBuild => c_game_build(context, tx, buf),
        op::Command::GameCommitDeck => c_game_commit_deck(context, tx, buf),
        op::Command::GameStartTurn => c_game_start_turn(context, tx, buf),
        op::Command::GameChooseAttr => c_game_choose_attr(context, tx, buf),
        op::Command::GamePlayCard => c_game_play_card(context, tx, buf),
        op::Command::GameEndTurn => c_game_end_turn(context, tx, buf),
        op::Command::GameEnd => c_game_end(context, tx, buf),
        _ => {}
    };
    VClientMode::Continue
}

fn c_game_start(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameStartRequest>();

    let mut user = GameUser::new(header.auth);

    let temp_builder = PlayerBuilder::new(&user.parts, &context.data_manager);
    user.parts.clear();

    let mut game_id = request.game_id;
    while game_id == 0 {
        let new_id = thread_rng().random::<GameIdType>();
        if !context.games.contains_key(&new_id) {
            game_id = new_id;
        }
    }

    let game = context.games.entry(game_id).or_default();
    game.user_add(header.user, user);
    game.set_stage_build();

    context.bx.track(header.user, (gate, header.vagabond));

    println!("[Hall] [{:X}] Sending parts to G({})=>V({})", game_id, gate, header.vagabond);

    let parts = [
        temp_builder.access.convert_to_player_part(),
        temp_builder.breach.convert_to_player_part(),
        temp_builder.compute.convert_to_player_part(),
        temp_builder.disrupt.convert_to_player_part(),
        temp_builder.build.convert_to_player_part(),
        temp_builder.build_values.convert_to_player_part(),
        temp_builder.category.convert_to_player_part(),
        temp_builder.category_values.convert_to_player_part(),
    ];

    let route = op::Route::One(gate);
    let command = op::Command::GameActivate;
    let response = GameStartResponse {
        game_id,
        parts,
    };

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn c_game_build(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameBuildRequest>();

    let builder = PlayerBuilder::new(&request.parts, &context.data_manager);
    let response = if let Some(player) = builder.create_player(&context.data_manager) {
        GameBuildResponse {
            seed: player.seed,
            deck: Vec::<_>::from(player.deck),
        }
    } else {
        GameBuildResponse {
            seed: 0,
            deck: Vec::default(),
        }
    };

    println!("[Hall] Sending build to G({})=>V({})", gate, header.vagabond);

    let route = op::Route::One(gate);
    let command = op::Command::GameBuild;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn c_game_commit_deck(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameBuildRequest>();

    let builder = PlayerBuilder::new(&request.parts, &context.data_manager);
    let player = builder.create_player(&context.data_manager);

    {
        let context = context.deref_mut();
        let (games, dm) = (&mut context.games, &context.data_manager);

        if let Some(game) = games.get_mut(&request.game_id) {
            let mut rng = game.rng.clone();
            if let Some(user) = game.get_user_auth(header.user, header.auth) {
                user.player = player;
                if let Some(valid_player) = user.player.as_ref() {
                    user.state.setup(valid_player.deck.iter().filter_map(|card| dm.lookup_player_card(card)).collect(), &mut rng);

                    let route = op::Route::One(gate);
                    let command = op::Command::GameCommitDeck;
                    let player_view = user.state.to_player_view();

                    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + player_view.size_in_buffer());

                    out.push(&route);
                    out.push(&command);
                    out.push(&header.vagabond);
                    out.push(&player_view);

                    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
                }
            }
        }
    }


    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_have_players() {
            game.set_stage_gameplay();
            let stage = game.get_stage();
            let mut broadcast_buf = VSizedBuffer::new(stage.size_in_buffer());
            broadcast_buf.push(&stage);
            context.bx.broadcast(op::Command::GameStartGame, broadcast_buf);
        }
    }
}

fn c_game_end(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let game_id = buf.pull::<GameIdType>();

    if let Some(game) = context.games.get_mut(&game_id) {
        if game.is_empty() {
            context.games.remove(&game_id);
        }
    }

    let route = op::Route::One(gate);
    let command = op::Command::GameEnd;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn update_user(context: &mut Hall, game_id: GameIdType, user: UserIdType, auth: AuthType, command: op::Command, update: impl Fn(&mut GameUser) -> bool) -> bool {
    if let Some(game) = context.games.get_mut(&game_id) {
        if let Some(user) = game.get_user_auth(user, auth) {
            user.state.last_command = Some(command);
            return update(user);
        }
    }
    false
}

fn c_game_start_turn(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameStartTurnRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameStartTurn, |_user| { /*TODO:*/true });

    let response = GameStartTurnResponse {
        success
    };

    let route = op::Route::One(gate);
    let command = op::Command::GameStartTurn;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameStartTurn) {
            game.roll();
            let mut broadcast_buf = VSizedBuffer::new(size_of::<CostType>() * 4);
            broadcast_buf.push(&game.erg_roll[0]);
            broadcast_buf.push(&game.erg_roll[1]);
            broadcast_buf.push(&game.erg_roll[2]);
            broadcast_buf.push(&game.erg_roll[3]);
            context.bx.broadcast(op::Command::GameStartGame, broadcast_buf);
        }
    }
}

fn c_game_choose_attr(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameChooseAttrRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameChooseAttr, |_user| { /*TODO:*/true });

    let response = GameChooseAttrResponse {
        success
    };

    let route = op::Route::One(gate);
    let command = op::Command::GameChooseAttr;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });

    if let Some(game) = context.games.get_mut(&request.game_id) {
        if game.all_users_last_command(op::Command::GameChooseAttr) {
            let (erg_roll, users) = game.split_borrow_for_resolve();
            for user in users.values_mut() {
                if let Some(player) = &user.player {
                    if let Some(kind) = user.state.resolve_kind {
                        let [erg, _] = GameState::resolve_matchups(erg_roll, &player.get_attr(kind), &[5, 5, 5, 5]);
                        user.state.add_erg(kind, erg);
                    }
                }
            }
            // TODO: set phase
            broadcast_player_state(request.game_id, context);
        }
    }
}

fn broadcast_player_state(game_id: GameIdType, mut context: MutexGuard<Hall>) {
    let (games, bx) = context.split_borrow();

    if let Some(game) = games.get_mut(&game_id) {
        if game.all_users_last_command(op::Command::GameChooseAttr) {
            let mut errors = vec![];
            for (id, (gate, vagabond)) in &bx.gate_map {
                let route = op::Route::One(*gate);
                let command = op::Command::GameUpdateState;
                let state = game.get_user(*id).unwrap().state.to_player_view();

                let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + vagabond.size_in_buffer() + state.size_in_buffer());

                out.push(&route);
                out.push(&command);
                out.push(vagabond);
                out.push(&state);

                let result = bx.local_tx.send(RoutedMessage { route: op::Route::None, buf: out });
                if result.is_err() {
                    errors.push(*id);
                }
            }

            for id in &errors {
                context.bx.gate_map.remove(id);
            }
        }
    }
}

fn c_game_play_card(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GamePlayCardRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GamePlayCard, |user| user.state.play_card(request.card_idx as usize));

    let response = GamePlayCardResponse {
        success
    };

    let route = op::Route::One(gate);
    let command = op::Command::GamePlayCard;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn c_game_end_turn(context: Arc<Mutex<Hall>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let mut context = context.lock().unwrap();

    let gate = buf.pull::<NodeType>();
    let header = buf.pull::<GateHeader>();
    let request = buf.pull::<GameEndTurnRequest>();

    let success = update_user(&mut context, request.game_id, header.user, header.auth, op::Command::GameEndTurn, |_user| { /*TODO:*/true });

    let response = GameEndTurnResponse {
        success
    };

    let route = op::Route::One(gate);
    let command = op::Command::GameEndTurn;

    let mut out = VSizedBuffer::new(route.size_in_buffer() + command.size_in_buffer() + header.vagabond.size_in_buffer() + response.size_in_buffer());

    out.push(&route);
    out.push(&command);
    out.push(&header.vagabond);
    out.push(&response);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}
