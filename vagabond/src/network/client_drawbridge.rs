use std::net::IpAddr;
use bevy::prelude::Resource;

use fasthash::farm::fingerprint128;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use shared_net::{op, VClientMode, VRoute, VRoutedMessage, VSizedBuffer};

pub(crate) struct AuthInfo {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
    pub(crate) auth: u128,
}

#[derive(Resource)]
pub(crate) struct DrawbridgeIFace {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) dtx: UnboundedSender<VRoutedMessage>,
    pub(crate) drx: UnboundedReceiver<AuthInfo>,
}

#[derive(Clone)]
pub(crate) struct DrawbridgeClient {
    pub(crate) auth_tx: UnboundedSender<AuthInfo>,
}

impl DrawbridgeClient {
    pub(crate) fn start(iface: String, auth_tx: UnboundedSender<AuthInfo>, rx: UnboundedReceiver<VRoutedMessage>, runtime: &Runtime) -> Option<JoinHandle<Result<(), ()>>> {
        let (dummy_tx, _) = mpsc::unbounded_channel();
        Some(runtime.spawn(shared_net::async_client(DrawbridgeClient { auth_tx }, op::Flavor::Vagabond, dummy_tx, rx, iface, process_drawbridge)))
    }
}

fn process_drawbridge(context: DrawbridgeClient, _tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        //op::Command::Hello => {},
        op::Command::Authorize => d_authorize(context, buf),
        _ => VClientMode::Continue
    }
}

fn d_authorize(context: DrawbridgeClient, mut buf: VSizedBuffer) -> VClientMode {
    let mut ip_buf = [0; 16];
    ip_buf.copy_from_slice(&buf.pull_bytes_n(16));
    let ip = IpAddr::from(ip_buf);

    let port = buf.pull_u16();
    let auth = buf.pull_u128();

    println!("IP: {} Port: {}", ip, port);

    let _ = context.auth_tx.send(AuthInfo {
        ip,
        port,
        auth,
    });

    VClientMode::Shutdown
}

pub(crate) fn send_authorize(tx: &UnboundedSender<VRoutedMessage>, user: String, pass: String) {
    let mut out = VSizedBuffer::new(64);
    out.push_command(op::Command::Authorize);
    out.push_u128(&fingerprint128(user.as_bytes()));
    out.push_u128(&fingerprint128(pass.as_bytes()));

    let msg = VRoutedMessage { route: VRoute::Local, buf: out };

    let _ = tx.send(msg);
}
