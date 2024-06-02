use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use shared_net::{op, VClientMode, VRoute, VRoutedMessage, VSizedBuffer};

#[derive(PartialEq)]
pub(crate) enum Command {
    Hello,
}

#[derive(Clone)]
pub struct GateClient {
    auth: u128,
    pub(crate) tx: UnboundedSender<Command>,
}

impl GateClient {
    pub(crate) fn start(auth: u128, iface: String, tx: UnboundedSender<Command>, rx: UnboundedReceiver<VRoutedMessage>, runtime: &Runtime) -> Option<JoinHandle<Result<(), ()>>> {
        let (dummy_tx, _) = mpsc::unbounded_channel();
        Some(runtime.spawn(shared_net::async_client(GateClient { auth, tx }, op::Flavor::Vagabond, dummy_tx, rx, iface, process_gate)))
    }
}

fn process_gate(context: GateClient, _tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        op::Command::Hello => g_hello(context),
        op::Command::Chat => g_chat(&mut buf),
        op::Command::DM => g_dm(&mut buf),
        op::Command::InvList => g_invlist(&mut buf),
        op::Command::DraftList => g_draftlist(&mut buf),
        op::Command::DraftJoin => g_draftjoin(&mut buf),
        _ => VClientMode::Continue
    }
}

fn g_hello(context: GateClient) -> VClientMode {
    let _ = context.tx.send(Command::Hello);
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
        println!("[InvList] * {:X}", buf.pull_u128());
    }

    VClientMode::Continue
}

fn g_draftlist(buf: &mut VSizedBuffer) -> VClientMode {
    let count = buf.pull_u16();
    println!("[DraftList] {} objects", count);
    for _idx in 0..count {
        println!("[DraftList] * {:X}", buf.pull_u128());
    }

    VClientMode::Continue
}

fn g_draftjoin(buf: &mut VSizedBuffer) -> VClientMode {
    let success = buf.pull_u128();
    println!("[DraftJoin] {}", if success == 0 { "Failure" } else { "Success" });

    VClientMode::Continue
}

fn send(tx: UnboundedSender<VRoutedMessage>, out: VSizedBuffer) {
    let _ = tx.send(VRoutedMessage { route: VRoute::Local, buf: out });
}

#[allow(dead_code)]
pub fn g_send_hack(tx: UnboundedSender<VRoutedMessage>, auth: u128) {
    let mut out = VSizedBuffer::new(32);
    out.push_command(op::Command::InvGen);
    out.push_u128(&auth);
    out.push_u8(&123_u8);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_inv(tx: UnboundedSender<VRoutedMessage>, auth: u128) {
    let mut out = VSizedBuffer::new(32);
    out.push_command(op::Command::InvList);
    out.push_u128(&auth);
    out.push_u8(&123_u8);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_chat(tx: UnboundedSender<VRoutedMessage>, auth: u128, msg: &str) {
    let mut out = VSizedBuffer::new(256);
    out.push_command(op::Command::Chat);
    out.push_u128(&auth);
    out.push_string(msg);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_dm(tx: UnboundedSender<VRoutedMessage>, auth: u128, who: &str, msg: &str) {
    let mut out = VSizedBuffer::new(256);
    out.push_command(op::Command::DM);
    out.push_u128(&auth);
    out.push_string(who);
    out.push_string(msg);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_draftlist(tx: UnboundedSender<VRoutedMessage>, auth: u128) {
    let mut out = VSizedBuffer::new(32);
    out.push_command(op::Command::DraftList);
    out.push_u128(&auth);
    out.push_u8(&0);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_draftjoin(tx: UnboundedSender<VRoutedMessage>, auth: u128, id: u128) {
    let mut out = VSizedBuffer::new(64);
    out.push_command(op::Command::DraftJoin);
    out.push_u128(&auth);
    out.push_u128(&id);

    send(tx, out);
}

#[allow(dead_code)]
pub fn g_send_draftpick(tx: UnboundedSender<VRoutedMessage>, auth: u128, draft: u128, pick: u8) {
    let mut out = VSizedBuffer::new(64);
    out.push_command(op::Command::DraftPick);
    out.push_u128(&auth);
    out.push_u128(&draft);
    out.push_u8(&pick);

    send(tx, out);
}
