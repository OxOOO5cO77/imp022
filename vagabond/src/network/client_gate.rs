use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use shared_net::{op, VClientMode, VRoute, VRoutedMessage, VSizedBuffer};

#[derive(PartialEq)]
pub(crate) enum GateCommand {
    Hello,
    PackContents(Vec<u64>),
}

#[derive(Resource)]
pub(crate) struct GateIFace {
    pub(crate) auth: u128,
    pub(crate) gtx: UnboundedSender<VRoutedMessage>,
    pub(crate) grx: UnboundedReceiver<GateCommand>,
}

#[derive(Clone)]
pub(crate) struct GateClient {
    game_id: Option<u128>,
    pub(crate) tx: UnboundedSender<GateCommand>,
}

impl GateClient {
    pub(crate) fn start(iface: String, tx: UnboundedSender<GateCommand>, rx: UnboundedReceiver<VRoutedMessage>, runtime: &Runtime) -> Option<JoinHandle<Result<(), ()>>> {
        let (dummy_tx, _) = mpsc::unbounded_channel();
        Some(runtime.spawn(shared_net::async_client(GateClient { game_id: None, tx }, op::Flavor::Vagabond, dummy_tx, rx, iface, process_gate)))
    }
}

fn process_gate(context: GateClient, _tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        op::Command::Hello => g_hello(context),
        op::Command::Chat => g_chat(&mut buf),
        op::Command::DM => g_dm(&mut buf),
        op::Command::InvList => g_invlist(&mut buf),
        op::Command::GameStart => g_gamestart(context, &mut buf),
        op::Command::GameEnd => g_gameend(&mut buf),
        _ => VClientMode::Continue
    }
}

fn g_hello(context: GateClient) -> VClientMode {
    let _ = context.tx.send(GateCommand::Hello);
    VClientMode::Continue
}

fn g_chat(buf: &mut VSizedBuffer) -> VClientMode {
    let name = buf.pull_string();
    if let Ok(msg) = String::from_utf8(buf.pull_bytes()) {
        println!("[Chat] {}: {}", name, msg.as_str());
    }

    VClientMode::Continue
}

fn g_dm(buf: &mut VSizedBuffer) -> VClientMode {
    let name = buf.pull_string();
    if let Ok(msg) = String::from_utf8(buf.pull_bytes()) {
        println!("[DM] {}: {}", name, msg.as_str());
    }

    VClientMode::Continue
}

fn g_invlist(buf: &mut VSizedBuffer) -> VClientMode {
    let count = buf.pull_u16();
    println!("[InvList] {} objects", count);
    for _idx in 0..count {
        println!("[InvList] * {:X}", buf.pull_u64());
    }

    VClientMode::Continue
}

fn g_gamestart(mut context: GateClient, buf: &mut VSizedBuffer) -> VClientMode {
    let game_id = buf.pull_u128();

    let count = buf.pull_u8();
    println!("[GameStart] {} objects", count);
    let parts = (0..count).map(|_| buf.pull_u64()).collect::<Vec<_>>();

    let _ = context.tx.send(GateCommand::PackContents(parts));
    context.game_id = Some(game_id);

    VClientMode::Continue
}

fn g_gameend(_buf: &mut VSizedBuffer) -> VClientMode {
    VClientMode::Continue
}

impl GateIFace {
    fn send(&self, out: VSizedBuffer) {
        let _ = self.gtx.send(VRoutedMessage { route: VRoute::Local, buf: out });
    }

    pub fn send_gamestart(&self) {
        let mut out = VSizedBuffer::new(64);
        out.push_command(op::Command::GameStart);
        out.push_u128(&self.auth);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn send_gameend(&self) {
        let mut out = VSizedBuffer::new(64);
        out.push_command(op::Command::GameEnd);
        out.push_u128(&self.auth);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_hack(&self) {
        let mut out = VSizedBuffer::new(32);
        out.push_command(op::Command::InvGen);
        out.push_u128(&self.auth);
        out.push_u8(&123_u8);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_inv(&self) {
        let mut out = VSizedBuffer::new(32);
        out.push_command(op::Command::InvList);
        out.push_u128(&self.auth);
        out.push_u8(&123_u8);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_chat(&self, msg: &str) {
        let mut out = VSizedBuffer::new(256);
        out.push_command(op::Command::Chat);
        out.push_u128(&self.auth);
        out.push_string(msg);

        self.send(out);
    }

    #[allow(dead_code)]
    pub fn g_send_dm(&self, who: &str, msg: &str) {
        let mut out = VSizedBuffer::new(256);
        out.push_command(op::Command::DM);
        out.push_u128(&self.auth);
        out.push_string(who);
        out.push_string(msg);

        self.send(out);
    }
}

