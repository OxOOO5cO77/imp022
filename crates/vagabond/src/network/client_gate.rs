use std::collections::HashMap;

use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use archive_lib::core::ArchiveSubCommand;
use forum_lib::core::ForumSubCommand;
use hall_lib::core::{AttributeKind, GameSubCommand, MissionNodeIntent, PickedCardTarget};
use hall_lib::message::*;
use shared_net::op::SubCommandType;
use shared_net::{AuthType, Bufferable, GameIdType, PartType};
use shared_net::{RoutedMessage, SizedBuffer, SizedBufferError, VClientMode, op};

pub(crate) enum GateCommand {
    Hello,
    GameActivate(Box<GameActivateResponse>),
    GameBuild(Box<GameBuildResponse>),
    GameStartGame(Box<GameStartGameMessage>),
    GameChooseIntent(Box<GameChooseIntentResponse>),
    GameRoll(Box<GameRollMessage>),
    GameChooseAttr(Box<GameChooseAttrResponse>),
    GameResources(Box<GameResourcesMessage>),
    GamePlayCard(Box<GamePlayCardResponse>),
    GameResolveCards(Box<GameResolveCardsMessage>),
    GameEndTurn(Box<GameEndTurnResponse>),
    GameTick(Box<GameTickMessage>),
    GameEndGame(Box<GameEndGameResponse>),
    GameUpdateMission(Box<GameUpdateMissionMessage>),
    GameUpdateTokens(Box<GameUpdateTokensMessage>),
    GameUpdateState(Box<GameUpdateStateResponse>),
}

#[derive(Resource)]
pub(crate) struct GateIFace {
    pub(crate) auth: AuthType,
    pub(crate) game_id: GameIdType,
    pub(crate) gtx: UnboundedSender<RoutedMessage>,
    pub(crate) grx: UnboundedReceiver<GateCommand>,
}

#[derive(Clone)]
pub(crate) struct GateClient {
    pub(crate) tx: UnboundedSender<GateCommand>,
}

impl GateClient {
    pub(crate) fn start(iface: String, tx: UnboundedSender<GateCommand>, rx: UnboundedReceiver<RoutedMessage>, runtime: &Runtime) -> Option<JoinHandle<Result<(), ()>>> {
        let (dummy_tx, _) = mpsc::unbounded_channel();
        let gate_client = GateClient {
            tx,
        };
        Some(runtime.spawn(shared_net::async_client(gate_client, op::Flavor::Vagabond, dummy_tx, rx, iface, process_gate)))
    }
}

fn subprocess_message(subcommand: SubCommandType, mut buf: SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    match subcommand.into() {
        ForumSubCommand::Chat => recv_chat(&mut buf),
        ForumSubCommand::DM => recv_dm(&mut buf),
    }
}

fn subprocess_inventory(subcommand: SubCommandType, mut buf: SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    match subcommand.into() {
        ArchiveSubCommand::InvList => recv_inv_list(&mut buf),
        ArchiveSubCommand::InvGen => Ok(VClientMode::Continue),
    }
}

fn subprocess_game(subcommand: SubCommandType, context: GateClient, mut buf: SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    match subcommand.into() {
        GameSubCommand::Activate => recv_response(context, &mut buf, GateCommand::GameActivate),
        GameSubCommand::Build => recv_response(context, &mut buf, GateCommand::GameBuild),
        GameSubCommand::StartGame => recv_response(context, &mut buf, GateCommand::GameStartGame),
        GameSubCommand::ChooseIntent => recv_response(context, &mut buf, GateCommand::GameChooseIntent),
        GameSubCommand::Roll => recv_response(context, &mut buf, GateCommand::GameRoll),
        GameSubCommand::ChooseAttr => recv_response(context, &mut buf, GateCommand::GameChooseAttr),
        GameSubCommand::Resources => recv_response(context, &mut buf, GateCommand::GameResources),
        GameSubCommand::PlayCard => recv_response(context, &mut buf, GateCommand::GamePlayCard),
        GameSubCommand::ResolveCards => recv_response(context, &mut buf, GateCommand::GameResolveCards),
        GameSubCommand::EndTurn => recv_response(context, &mut buf, GateCommand::GameEndTurn),
        GameSubCommand::Tick => recv_response(context, &mut buf, GateCommand::GameTick),
        GameSubCommand::EndGame => recv_response(context, &mut buf, GateCommand::GameEndGame),
        GameSubCommand::UpdateMission => recv_response(context, &mut buf, GateCommand::GameUpdateMission),
        GameSubCommand::UpdateTokens => recv_response(context, &mut buf, GateCommand::GameUpdateTokens),
        GameSubCommand::UpdateState => recv_response(context, &mut buf, GateCommand::GameUpdateState),
    }
}

fn process_gate(context: GateClient, _tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    if let Ok(command) = buf.pull::<op::Command>() {
        match command {
            op::Command::Hello => recv_hello(context),
            op::Command::Message(sub) => subprocess_message(sub, buf),
            op::Command::Inventory(sub) => subprocess_inventory(sub, buf),
            op::Command::Game(sub) => subprocess_game(sub, context, buf),
            op::Command::NoOp | op::Command::Register | op::Command::Authorize | op::Command::UserAttr => Ok(VClientMode::Continue),
        }
        .unwrap_or(VClientMode::Continue)
    } else {
        VClientMode::Continue
    }
}

fn recv_hello(context: GateClient) -> Result<VClientMode, SizedBufferError> {
    let _ = context.tx.send(GateCommand::Hello);
    Ok(VClientMode::Continue)
}

fn recv_chat(buf: &mut SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    let name = buf.pull::<String>()?;
    if let Ok(msg) = String::from_utf8(buf.pull_remaining()?) {
        println!("[Chat] {}: {}", name, msg.as_str());
    }

    Ok(VClientMode::Continue)
}

fn recv_dm(buf: &mut SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    let name = buf.pull::<String>()?;
    if let Ok(msg) = String::from_utf8(buf.pull_remaining()?) {
        println!("[DM] {}: {}", name, msg.as_str());
    }

    Ok(VClientMode::Continue)
}

fn recv_inv_list(buf: &mut SizedBuffer) -> Result<VClientMode, SizedBufferError> {
    let count = buf.pull::<u16>()?;
    println!("[InvList] {} objects", count);
    for _idx in 0..count {
        println!("[InvList] * {:X}", buf.pull::<u64>()?);
    }

    Ok(VClientMode::Continue)
}

fn recv_response<T: Bufferable>(context: GateClient, buf: &mut SizedBuffer, as_enum: impl FnOnce(Box<T>) -> GateCommand) -> Result<VClientMode, SizedBufferError> {
    let response = buf.pull::<T>()?;
    let _ = context.tx.send(as_enum(Box::new(response)));
    Ok(VClientMode::Continue)
}

impl GateIFace {
    fn send_request<T: CommandMessage>(&self, request: T) -> bool {
        let mut out = SizedBuffer::new(T::COMMAND.size_in_buffer() + self.auth.size_in_buffer() + request.size_in_buffer());
        let _ = out.push(&T::COMMAND);
        let _ = out.push(&self.auth);
        let _ = out.push(&request);

        let result = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });

        result.is_ok()
    }

    pub fn send_game_activate(&self) -> bool {
        let request = GameActivateRequest {
            game_id: self.game_id,
        };

        self.send_request(request)
    }

    pub fn send_game_build(&self, parts: [PartType; 8], commit: bool) -> bool {
        let request = GameBuildRequest {
            game_id: self.game_id,
            parts,
            commit,
        };

        self.send_request(request)
    }

    pub fn send_game_choose_intent(&self, intent: Option<MissionNodeIntent>) -> bool {
        let request = GameChooseIntentRequest {
            game_id: self.game_id,
            intent: intent.unwrap_or_default(),
        };

        self.send_request(request)
    }

    pub fn send_game_choose_attr(&self, kind: Option<AttributeKind>) -> bool {
        if let Some(attr) = kind {
            let request = GameChooseAttrRequest {
                game_id: self.game_id,
                attr,
            };
            self.send_request(request)
        } else {
            false
        }
    }

    pub fn send_game_play_cards(&self, picks_map: &HashMap<CardIdxType, PickedCardTarget>) -> bool {
        let request = GamePlayCardRequest {
            game_id: self.game_id,
            picks: picks_map.iter().map(|(&idx, &target)| (idx, target)).collect(),
        };

        self.send_request(request)
    }

    pub fn send_game_end_turn(&self) -> bool {
        let request = GameEndTurnRequest {
            game_id: self.game_id,
        };

        self.send_request(request)
    }

    pub fn send_game_update_state(&self) -> bool {
        let request = GameUpdateStateRequest {
            game_id: self.game_id,
        };

        self.send_request(request)
    }

    #[allow(dead_code)]
    pub fn g_send_hack(&self) {
        let mut out = SizedBuffer::new(32);
        let _ = out.push(&op::Command::Inventory(ArchiveSubCommand::InvGen as SubCommandType));
        let _ = out.push(&self.auth);
        let _ = out.push(&123_u8);

        let _ = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });
    }

    #[allow(dead_code)]
    pub fn g_send_inv(&self) {
        let mut out = SizedBuffer::new(32);
        let _ = out.push(&op::Command::Inventory(ArchiveSubCommand::InvList as SubCommandType));
        let _ = out.push(&self.auth);
        let _ = out.push(&123_u8);

        let _ = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });
    }

    #[allow(dead_code)]
    pub fn g_send_chat(&self, msg: &str) {
        let mut out = SizedBuffer::new(256);
        let _ = out.push(&op::Command::Message(ForumSubCommand::Chat as SubCommandType));
        let _ = out.push(&self.auth);
        let _ = out.push(&msg.to_string());

        let _ = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });
    }

    #[allow(dead_code)]
    pub fn g_send_dm(&self, who: &str, msg: &str) {
        let mut out = SizedBuffer::new(256);
        let _ = out.push(&op::Command::Message(ForumSubCommand::DM as SubCommandType));
        let _ = out.push(&self.auth);
        let _ = out.push(&who.to_string());
        let _ = out.push(&msg.to_string());

        let _ = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });
    }
}
