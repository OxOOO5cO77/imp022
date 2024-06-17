use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use hall::message::gamebuild::{GameBuildRequest, GameBuildResponse};
use hall::message::gamestart::{GameStartRequest, GameStartResponse};
use shared_data::types::{AuthType, GameIdType, PartType};
use shared_net::{op, RoutedMessage, VClientMode, VSizedBuffer};
use shared_net::sizedbuffers::Bufferable;

pub(crate) enum GateCommand {
    Hello,
    GameStart(Box<GameStartResponse>),
    PlayerBuild(Box<GameBuildResponse>),
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
        Some(runtime.spawn(shared_net::async_client(GateClient { tx }, op::Flavor::Vagabond, dummy_tx, rx, iface, process_gate)))
    }
}

fn process_gate(context: GateClient, _tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::Hello => g_hello(context),
        op::Command::Chat => g_chat(&mut buf),
        op::Command::DM => g_dm(&mut buf),
        op::Command::InvList => g_invlist(&mut buf),
        op::Command::GameStart => g_gamestart(context, &mut buf),
        op::Command::GameBuild => g_gamebuild(context, &mut buf),
        op::Command::GameEnd => g_gameend(&mut buf),
        _ => VClientMode::Continue
    }
}

fn g_hello(context: GateClient) -> VClientMode {
    let _ = context.tx.send(GateCommand::Hello);
    VClientMode::Continue
}

fn g_chat(buf: &mut VSizedBuffer) -> VClientMode {
    let name = buf.pull::<String>();
    if let Ok(msg) = String::from_utf8(buf.drain_bytes()) {
        println!("[Chat] {}: {}", name, msg.as_str());
    }

    VClientMode::Continue
}

fn g_dm(buf: &mut VSizedBuffer) -> VClientMode {
    let name = buf.pull::<String>();
    if let Ok(msg) = String::from_utf8(buf.drain_bytes()) {
        println!("[DM] {}: {}", name, msg.as_str());
    }

    VClientMode::Continue
}

fn g_invlist(buf: &mut VSizedBuffer) -> VClientMode {
    let count = buf.pull::<u16>();
    println!("[InvList] {} objects", count);
    for _idx in 0..count {
        println!("[InvList] * {:X}", buf.pull::<u64>());
    }

    VClientMode::Continue
}

fn g_gamestart(context: GateClient, buf: &mut VSizedBuffer) -> VClientMode {
    let response = buf.pull::<GameStartResponse>();

    let _ = context.tx.send(GateCommand::GameStart(Box::new(response)));

    VClientMode::Continue
}

fn g_gamebuild(context: GateClient, buf: &mut VSizedBuffer) -> VClientMode {
    let response = buf.pull::<GameBuildResponse>();

    let _ = context.tx.send(GateCommand::PlayerBuild(Box::new(response)));

    VClientMode::Continue
}

fn g_gameend(_buf: &mut VSizedBuffer) -> VClientMode {
    VClientMode::Continue
}

impl GateIFace {
    fn send(&self, out: VSizedBuffer) {
        let _ = self.gtx.send(RoutedMessage { route: op::Route::Local, buf: out });
    }

    pub fn send_gamestart(&self) {
        let command = op::Command::GameStart;
        let request = GameStartRequest {
          game_id: 0
        };

        let mut out = VSizedBuffer::new(command.size_in_buffer() + self.auth.size_in_buffer() + request.size_in_buffer());
        out.push(&command);
        out.push(&self.auth);
        out.push(&request);

        self.send(out);
    }

    pub fn send_gamebuild(&self, game_id: GameIdType, parts: [PartType; 8]) {
        let command = op::Command::GameBuild;
        let request = GameBuildRequest {
            game_id,
            parts,
        };

        let mut out = VSizedBuffer::new(command.size_in_buffer() + self.auth.size_in_buffer() + request.size_in_buffer());
        out.push(&command);
        out.push(&self.auth);
        out.push(&request);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn send_gameend(&self) {
        let command = op::Command::GameEnd;

        let mut out = VSizedBuffer::new(command.size_in_buffer() + self.auth.size_in_buffer());
        out.push(&command);
        out.push(&self.auth);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_hack(&self) {
        let mut out = VSizedBuffer::new(32);
        out.push(&op::Command::InvGen);
        out.push(&self.auth);
        out.push(&123_u8);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_inv(&self) {
        let mut out = VSizedBuffer::new(32);
        out.push(&op::Command::InvList);
        out.push(&self.auth);
        out.push(&123_u8);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_chat(&self, msg: &str) {
        let mut out = VSizedBuffer::new(256);
        out.push(&op::Command::Chat);
        out.push(&self.auth);
        out.push(&msg.to_string());

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_dm(&self, who: &str, msg: &str) {
        let mut out = VSizedBuffer::new(256);
        out.push(&op::Command::DM);
        out.push(&self.auth);
        out.push(&who.to_string());
        out.push(&msg.to_string());

        self.send(out);
    }
}

