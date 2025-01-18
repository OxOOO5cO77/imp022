use bevy::prelude::Resource;
use std::collections::HashMap;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use hall::core::{AttributeKind, MissionNodeIntent};
use hall::message::*;
use shared_net::{op, RoutedMessage, SizedBuffer, SizedBufferError, VClientMode};
use shared_net::{AuthType, Bufferable, GameIdType, PartType};

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

fn process_gate(context: GateClient, _tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    if let Ok(command) = buf.pull::<op::Command>() {
        match command {
            op::Command::Hello => recv_hello(context),
            op::Command::Chat => recv_chat(&mut buf),
            op::Command::DM => recv_dm(&mut buf),
            op::Command::InvList => recv_inv_list(&mut buf),
            op::Command::GameActivate => recv_response(context, &mut buf, GateCommand::GameActivate),
            op::Command::GameBuild => recv_response(context, &mut buf, GateCommand::GameBuild),
            op::Command::GameStartGame => recv_response(context, &mut buf, GateCommand::GameStartGame),
            op::Command::GameChooseIntent => recv_response(context, &mut buf, GateCommand::GameChooseIntent),
            op::Command::GameRoll => recv_response(context, &mut buf, GateCommand::GameRoll),
            op::Command::GameChooseAttr => recv_response(context, &mut buf, GateCommand::GameChooseAttr),
            op::Command::GameResources => recv_response(context, &mut buf, GateCommand::GameResources),
            op::Command::GamePlayCard => recv_response(context, &mut buf, GateCommand::GamePlayCard),
            op::Command::GameResolveCards => recv_response(context, &mut buf, GateCommand::GameResolveCards),
            op::Command::GameEndTurn => recv_response(context, &mut buf, GateCommand::GameEndTurn),
            op::Command::GameTick => recv_response(context, &mut buf, GateCommand::GameTick),
            op::Command::GameEndGame => recv_response(context, &mut buf, GateCommand::GameEndGame),
            op::Command::GameUpdateMission => recv_response(context, &mut buf, GateCommand::GameUpdateMission),
            op::Command::GameUpdateTokens => recv_response(context, &mut buf, GateCommand::GameUpdateTokens),
            op::Command::GameUpdateState => recv_response(context, &mut buf, GateCommand::GameUpdateState),
            op::Command::NoOp | op::Command::Register | op::Command::Authorize | op::Command::UserAttr | op::Command::InvGen => Ok(VClientMode::Continue),
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

    pub fn send_game_play_cards(&self, picks_map: &HashMap<CardIdxType, CardTarget>) -> bool {
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
        let _ = out.push(&op::Command::InvGen);
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
        let _ = out.push(&op::Command::InvList);
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
        let _ = out.push(&op::Command::Chat);
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
        let _ = out.push(&op::Command::DM);
        let _ = out.push(&self.auth);
        let _ = out.push(&who.to_string());
        let _ = out.push(&msg.to_string());

        let _ = self.gtx.send(RoutedMessage {
            route: op::Route::Local,
            buf: out,
        });
    }
}
