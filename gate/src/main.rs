use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VIdMessage, VRoute, VRoutedMessage, VSizedBuffer};
use shared_net::op;

struct GateUser {
    name: String,
    user: u128,
    id: u8,
}

struct Gate {
    myface: SocketAddr,
    reply: UnboundedSender<VRoutedMessage>,
    map: HashMap<u128, GateUser>,
}

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let iface_to_vagabond = args.next().unwrap_or("[::]:23451".to_string());

    let (g2c_tx, g2c_rx) = mpsc::unbounded_channel();
    let (g2v_tx, g2v_rx) = mpsc::unbounded_channel();

    let gate_context = Arc::new(Mutex::new(Gate {
        myface: iface_to_vagabond.parse().unwrap(),
        reply: g2v_tx.clone(),
        map: HashMap::new(),
    }));

    let gate = shared_net::async_server(gate_context.clone(), g2v_tx, g2c_rx, iface_to_vagabond, process_vagabond);
    let courtyard_client = shared_net::async_client(gate_context.clone(), op::Flavor::Gate, g2c_tx, g2v_rx, iface_to_courtyard, process_courtyard);

    tokio::spawn(gate);
    tokio::spawn(courtyard_client);

    let _ = signal::ctrl_c().await;
}

fn process_vagabond(context: Arc<Mutex<Gate>>, tx: UnboundedSender<VRoutedMessage>, msg: VIdMessage) -> bool {
    let id = msg.id;
    let mut buf = msg.buf;
    let command = buf.pull_command();

    match command {
        op::Command::Hello => v_hello(context, id, &mut buf),
        op::Command::Chat |
        op::Command::DM => v_marshall_username(context, op::Flavor::Forum, command, &tx, &mut buf),
        op::Command::InvGen |
        op::Command::InvList => v_marshall(context, op::Flavor::Archive, command, &tx, id, &mut buf),
        op::Command::GameStart |
        op::Command::GameBuild |
        op::Command::GameEnd => v_marshall(context, op::Flavor::Hall, command, &tx, id, &mut buf),
        _ => false
    }
}

fn v_hello(context: Arc<Mutex<Gate>>, id: u8, buf: &mut VSizedBuffer) -> bool {
    let auth = buf.pull_u128();
    if let Some(user) = context.lock().unwrap().map.get_mut(&auth) {
        user.id = id;
        true
    } else { false }
}

fn v_marshall_username(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) -> bool {
    if let Some(user) = context.lock().unwrap().map.get(&buf.pull_u128()) {
        let mut out = VSizedBuffer::new(256);
        out.push_route(op::Route::Any);
        out.push_flavor(flavor);
        out.push_command(command);
        out.push_string(&user.name);
        out.xfer_bytes(buf);

        tx.send(VRoutedMessage { route: VRoute::Local, buf: out }).is_ok()
    } else {
        false
    }
}

fn v_marshall(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<VRoutedMessage>, id: u8, buf: &mut VSizedBuffer) -> bool {
    let auth = buf.pull_u128();
    if let Some(user) = context.lock().unwrap().map.get(&auth) {
        let mut out = VSizedBuffer::new(256);
        out.push_route(op::Route::Any);
        out.push_flavor(flavor);
        out.push_command(command);
        out.push_u8(&id);
        out.push_u128(&user.user);
        out.push_u128(&auth);
        out.xfer_bytes(buf);

        tx.send(VRoutedMessage { route: VRoute::Local, buf: out }).is_ok()
    } else {
        false
    }
}

fn process_courtyard(context: Arc<Mutex<Gate>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    let command = buf.pull_command();

    match command {
        op::Command::Authorize => c_authorize(context, &mut buf),
        op::Command::DM => c_marshall_name(command, context, &tx, &mut buf),
        op::Command::Chat => c_marshall_all(command, &tx, &mut buf),
        op::Command::InvList |
        op::Command::GameStart => c_marshall_one(command, &tx, &mut buf),
        _ => VClientMode::Continue
    }
}

fn convert_to_v6(iface: SocketAddr) -> Ipv6Addr {
    match iface.ip() {
        IpAddr::V4(ipv4) => ipv4.to_ipv6_compatible(),
        IpAddr::V6(ipv6) => ipv6
    }
}

fn c_authorize(context: Arc<Mutex<Gate>>, buf: &mut VSizedBuffer) -> VClientMode {
    println!("c_authorize");
    let mut out = VSizedBuffer::new(256);
    out.push_route(op::Route::One);

    let _ = buf.pull_u8(); // auth (discard)
    out.xfer_u8(buf);
    out.push_command(op::Command::Authorize);
    out.xfer_u8(buf);

    let authorization = buf.pull_u128();
    let user = buf.pull_u128();
    let name = buf.pull_string();

    let mut context = context.lock().unwrap();

    context.map.insert(authorization, GateUser { name, user, id: 0 });

    let ipv6 = convert_to_v6(context.myface);
    out.push_bytes(&ipv6.octets());
    out.push_u16(&context.myface.port());
    out.push_u128(&authorization);
    out.xfer_bytes(buf);

    if context.reply.send(VRoutedMessage { route: VRoute::Local, buf: out }).is_err() {
        return VClientMode::Disconnect;
    }

    let mut update = VSizedBuffer::new(128);
    update.push_route(op::Route::Any);
    update.push_flavor(op::Flavor::Jail);
    update.push_command(op::Command::UserAttr);
    update.push_u128(&user);
    update.push_string("login");

    let now = Utc::now().timestamp() as u128;
    update.push_u128(&now);

    if context.reply.send(VRoutedMessage { route: VRoute::Local, buf: update }).is_err() {
        VClientMode::Disconnect
    } else {
        VClientMode::Continue
    }
}

fn c_marshall_name(command: op::Command, context: Arc<Mutex<Gate>>, tx: &UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull_u8(); // forum (discard)

    let sendee = buf.pull_string();

    for (_, user) in context.lock().unwrap().map.iter_mut() {
        if user.name == sendee {
            return send_to_client(VRoute::One(user.id), command, tx, buf);
        }
    }

    VClientMode::Continue
}

fn c_marshall_one(command: op::Command, tx: &UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull_u8(); // sender (discard)
    let vagabond = buf.pull_u8();

    send_to_client(VRoute::One(vagabond), command, tx, buf)
}

fn c_marshall_all(command: op::Command, tx: &UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull_u8(); // sender (discard)

    send_to_client(VRoute::All(op::Flavor::Vagabond as u8), command, tx, buf)
}

fn send_to_client(route: VRoute, command: op::Command, tx: &UnboundedSender<VRoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let mut out = VSizedBuffer::new(buf.remaining() + 1);
    out.push_command(command);
    out.xfer_bytes(buf);

    if tx.send(VRoutedMessage { route, buf: out }).is_err() {
        VClientMode::Disconnect
    } else {
        VClientMode::Continue
    }
}
