use std::net::IpAddr;

use bevy::prelude::Resource;
use fasthash::farm::fingerprint128;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;

use shared_net::{op, AuthType, RoutedMessage, VClientMode, VSizedBuffer};

pub(crate) struct AuthInfo {
    pub(crate) ip: IpAddr,
    pub(crate) port: u16,
    pub(crate) auth: AuthType,
}

#[derive(Resource)]
pub(crate) struct DrawbridgeIFace {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) dtx: UnboundedSender<RoutedMessage>,
    pub(crate) drx: UnboundedReceiver<AuthInfo>,
}

#[derive(Clone)]
pub(crate) struct DrawbridgeClient {
    pub(crate) auth_tx: UnboundedSender<AuthInfo>,
}

impl DrawbridgeClient {
    pub(crate) fn start(iface: String, auth_tx: UnboundedSender<AuthInfo>, rx: UnboundedReceiver<RoutedMessage>, runtime: &Runtime) -> Option<JoinHandle<Result<(), ()>>> {
        let (dummy_tx, _) = mpsc::unbounded_channel();
        Some(runtime.spawn(shared_net::async_client(
            DrawbridgeClient {
                auth_tx,
            },
            op::Flavor::Vagabond,
            dummy_tx,
            rx,
            iface,
            process_drawbridge,
        )))
    }
}

fn process_drawbridge(context: DrawbridgeClient, _tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        //op::Command::Hello => {},
        op::Command::Authorize => recv_authorize(context, buf),
        _ => VClientMode::Continue,
    }
}

fn recv_authorize(context: DrawbridgeClient, mut buf: VSizedBuffer) -> VClientMode {
    let mut ip_buf = [0; 16];
    ip_buf.copy_from_slice(&buf.pull_bytes_n(16));

    let ip = IpAddr::from(ip_buf);
    let port = buf.pull::<u16>();
    let auth = buf.pull::<AuthType>();

    let auth_info = AuthInfo {
        ip,
        port,
        auth,
    };
    let _ = context.auth_tx.send(auth_info);

    VClientMode::Shutdown
}

pub(crate) fn send_authorize(tx: &UnboundedSender<RoutedMessage>, user: String, pass: String) {
    let mut out = VSizedBuffer::new(64);
    out.push(&op::Command::Authorize);
    out.push(&fingerprint128(user.as_bytes()));
    out.push(&fingerprint128(pass.as_bytes()));

    let msg = RoutedMessage {
        route: op::Route::Local,
        buf: out,
    };

    let _ = tx.send(msg);
}
