use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use gate::message::gate_header::GateHeader;
use shared_net::types::{AuthType, NodeType, TimestampType, UserIdType};
use shared_net::{op, IdMessage, RoutedMessage, VClientMode, VSizedBuffer};

struct GateUser {
    name: String,
    user: UserIdType,
    vagabond: NodeType,
}

struct Gate {
    myface: SocketAddr,
    reply: UnboundedSender<RoutedMessage>,
    map: HashMap<u128, GateUser>,
}

#[tokio::main]
async fn main() {
    println!("[Gate] START");

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

    println!("[Gate] END");
}

fn process_vagabond(context: Arc<Mutex<Gate>>, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let id = msg.id;
    let mut buf = msg.buf;
    let command = buf.pull::<op::Command>();

    match command {
        op::Command::Hello => v_hello(context, id, &mut buf),
        op::Command::Chat |
        op::Command::DM => v_marshal_username(context, op::Flavor::Forum, command, &tx, &mut buf),
        op::Command::InvGen |
        op::Command::InvList => v_marshal(context, op::Flavor::Archive, command, &tx, id, &mut buf),
        op::Command::GameActivate |
        op::Command::GameBuild |
        op::Command::GameStartTurn |
        op::Command::GameChooseAttr |
        op::Command::GamePlayCard |
        op::Command::GameUpdateState |
        op::Command::GameEndTurn => v_marshal(context, op::Flavor::Hall, command, &tx, id, &mut buf),
        op::Command::NoOp |
        op::Command::Register |
        op::Command::Authorize |
        op::Command::GameStartGame |
        op::Command::GameRoll |
        op::Command::GameResources |
        op::Command::GameResolveCards |
        op::Command::GameEndGame |
        op::Command::GameTick |
        op::Command::UserAttr => false,
    }
}

fn v_hello(context: Arc<Mutex<Gate>>, id: u8, buf: &mut VSizedBuffer) -> bool {
    let auth = buf.pull::<AuthType>();
    if let Some(user) = context.lock().unwrap().map.get_mut(&auth) {
        user.vagabond = id;
        true
    } else {
        false
    }
}

fn v_marshal_username(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> bool {
    if let Some(user) = context.lock().unwrap().map.get(&buf.pull::<u128>()) {
        let mut out = VSizedBuffer::new(256);
        out.push(&op::Route::Any(flavor));
        out.push(&command);
        out.push(&user.name);
        out.xfer_bytes(buf);

        tx.send(RoutedMessage { route: op::Route::Local, buf: out }).is_ok()
    } else {
        false
    }
}

fn v_marshal(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<RoutedMessage>, id: u8, buf: &mut VSizedBuffer) -> bool {
    let auth = buf.pull::<AuthType>();
    if let Some(user) = context.lock().unwrap().map.get(&auth) {
        let mut out = VSizedBuffer::new(256);
        out.push(&op::Route::Any(flavor));
        out.push(&command);
        out.push(&GateHeader::new(id, user.user, auth));
        out.xfer_bytes(buf);

        tx.send(RoutedMessage { route: op::Route::Local, buf: out }).is_ok()
    } else {
        false
    }
}

fn process_courtyard(context: Arc<Mutex<Gate>>, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    let command = buf.pull::<op::Command>();

    match command {
        op::Command::Authorize => c_authorize(context, &mut buf),
        op::Command::DM => c_marshal_name(command, context, &tx, &mut buf),
        op::Command::Chat => c_marshal_all(command, &tx, &mut buf),
        op::Command::InvList |
        op::Command::GameActivate |
        op::Command::GameBuild |
        op::Command::GameStartGame |
        op::Command::GameStartTurn |
        op::Command::GameRoll |
        op::Command::GameChooseAttr |
        op::Command::GameResources |
        op::Command::GamePlayCard |
        op::Command::GameResolveCards |
        op::Command::GameEndTurn |
        op::Command::GameTick |
        op::Command::GameUpdateState |
        op::Command::GameEndGame => c_marshal_one(command, &tx, &mut buf),
        op::Command::NoOp |
        op::Command::Register |
        op::Command::Hello |
        op::Command::UserAttr |
        op::Command::InvGen => VClientMode::Continue,
    }
}

fn convert_to_v6(iface: SocketAddr) -> Ipv6Addr {
    match iface.ip() {
        IpAddr::V4(ipv4) => ipv4.to_ipv6_compatible(),
        IpAddr::V6(ipv6) => ipv6
    }
}

fn c_authorize(context: Arc<Mutex<Gate>>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull::<NodeType>(); // auth (discard)

    let mut out = VSizedBuffer::new(256);

    let id = buf.pull::<NodeType>();
    out.push(&op::Route::One(id));
    out.push(&op::Command::Authorize);
    out.xfer::<NodeType>(buf);

    let user = buf.pull::<UserIdType>();
    let auth = buf.pull::<AuthType>();
    let name = buf.pull::<String>();

    let mut context = context.lock().unwrap();

    context.map.insert(auth, GateUser { name, user, vagabond: 0 });

    let ipv6 = convert_to_v6(context.myface);
    out.push_bytes(&ipv6.octets());
    out.push(&context.myface.port());
    out.push(&auth);
    out.xfer_bytes(buf);

    if context.reply.send(RoutedMessage { route: op::Route::Local, buf: out }).is_err() {
        return VClientMode::Disconnect;
    }

    let mut update = VSizedBuffer::new(128);
    update.push(&op::Route::Any(op::Flavor::Jail));
    update.push(&op::Command::UserAttr);
    update.push(&user);
    update.push(&"login".to_string());

    let now = Utc::now().timestamp() as TimestampType;
    update.push(&now);

    if context.reply.send(RoutedMessage { route: op::Route::Local, buf: update }).is_err() {
        VClientMode::Disconnect
    } else {
        VClientMode::Continue
    }
}

fn c_marshal_name(command: op::Command, context: Arc<Mutex<Gate>>, tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull::<NodeType>(); // forum (discard)

    let sendee = buf.pull::<String>();

    for (_, user) in context.lock().unwrap().map.iter_mut() {
        if user.name == sendee {
            return send_to_client(op::Route::One(user.vagabond), command, tx, buf);
        }
    }

    VClientMode::Continue
}

fn c_marshal_one(command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull::<NodeType>(); // sender (discard)
    let vagabond = buf.pull::<NodeType>();

    send_to_client(op::Route::One(vagabond), command, tx, buf)
}

fn c_marshal_all(command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let _ = buf.pull::<NodeType>(); // sender (discard)

    send_to_client(op::Route::All(op::Flavor::Vagabond), command, tx, buf)
}

fn send_to_client(route: op::Route, command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let mut out = VSizedBuffer::new(buf.remaining() + 1);
    out.push(&command);
    out.xfer_bytes(buf);

    if tx.send(RoutedMessage { route, buf: out }).is_err() {
        VClientMode::Disconnect
    } else {
        VClientMode::Continue
    }
}
