use std::collections::HashMap;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};

use archive_lib::core::ArchiveSubCommand;
use chrono::Utc;
use forum_lib::core::ForumSubCommand;
use gate_lib::message::gate_header::GateHeader;
use hall_lib::core::GameSubCommand;
use shared_net::op::SubCommandType;
use shared_net::{op, AuthType, Bufferable, IdMessage, NodeType, RoutedMessage, SizedBuffer, SizedBufferError, TimestampType, UserIdType, VClientMode};
use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{error, info, instrument};

struct GateUser {
    name: String,
    user: UserIdType,
    vagabond: NodeType,
}

struct Gate {
    interface: SocketAddr,
    reply: UnboundedSender<RoutedMessage>,
    map: HashMap<u128, GateUser>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum GateError {
    Interrupt,
    Parse(std::net::AddrParseError),
    SizedBuffer(SizedBufferError),
    Client(()),
    Server(()),
}

#[tokio::main]
async fn main() -> Result<(), GateError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let iface_to_vagabond = args.next().unwrap_or("[::]:23451".to_string());

    gate_main(iface_to_vagabond, iface_to_courtyard).await
}

#[instrument]
async fn gate_main(interface: String, courtyard: String) -> Result<(), GateError> {
    info!("START");

    let (g2c_tx, g2c_rx) = mpsc::unbounded_channel();
    let (g2v_tx, g2v_rx) = mpsc::unbounded_channel();

    let gate_context = Arc::new(Mutex::new(Gate {
        interface: interface.parse().map_err(GateError::Parse)?,
        reply: g2v_tx.clone(),
        map: HashMap::new(),
    }));

    let gate = shared_net::async_server(gate_context.clone(), g2v_tx, g2c_rx, interface, process_vagabond);
    let courtyard_client = shared_net::async_client(gate_context.clone(), op::Flavor::Gate, g2c_tx, g2v_rx, courtyard, process_courtyard);

    tokio::spawn(gate);
    tokio::spawn(courtyard_client);

    signal::ctrl_c().await.map_err(|_| GateError::Interrupt)?;

    info!("END");

    Ok(())
}

#[rustfmt::skip]
fn should_marshal_game_to_vagabond(subcommand: SubCommandType) -> bool {
    match subcommand.into() {
        GameSubCommand::Activate
        | GameSubCommand::Build
        | GameSubCommand::ChooseIntent
        | GameSubCommand::ChooseAttr
        | GameSubCommand::PlayCard
        | GameSubCommand::UpdateState
        | GameSubCommand::EndTurn => true,
        GameSubCommand::StartGame
        | GameSubCommand::Roll
        | GameSubCommand::Resources
        | GameSubCommand::ResolveCards
        | GameSubCommand::Tick
        | GameSubCommand::UpdateMission
        | GameSubCommand::UpdateTokens
        | GameSubCommand::EndGame => false,
    }
}

#[rustfmt::skip]
fn process_vagabond(context: Arc<Mutex<Gate>>, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let id = msg.id;
    let mut buf = msg.buf;
    if let Ok(command) = buf.pull::<op::Command>() {
        match command {
            op::Command::Hello => v_hello(context, id, &mut buf).is_ok(),
            op::Command::Message(_) => v_marshal_username(context, op::Flavor::Forum, command, &tx, &mut buf).is_ok(),
            op::Command::Inventory(_) => v_marshal(context, op::Flavor::Archive, command, &tx, id, &mut buf).is_ok(),
            op::Command::Game(subcommand) if should_marshal_game_to_vagabond(subcommand) => v_marshal(context, op::Flavor::Hall, command, &tx, id, &mut buf).is_ok(),
            op::Command::NoOp
            | op::Command::Register
            | op::Command::Authorize
            | op::Command::UserAttr
            | op::Command::Game(_)
            => false,
        }
    } else {
        false
    }
}

fn v_hello(context: Arc<Mutex<Gate>>, id: u8, buf: &mut SizedBuffer) -> Result<(), GateError> {
    let auth = buf.pull::<AuthType>().map_err(GateError::SizedBuffer)?;
    if let Some(user) = context.lock().unwrap().map.get_mut(&auth) {
        user.vagabond = id;
        Ok(())
    } else {
        Err(GateError::Client(()))
    }
}

fn v_marshal_username(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<(), GateError> {
    if let Some(user) = context.lock().unwrap().map.get(&buf.pull::<u128>().map_err(GateError::SizedBuffer)?) {
        let mut out = SizedBuffer::new(256);
        out.push(&op::Route::Any(flavor)).map_err(GateError::SizedBuffer)?;
        out.push(&command).map_err(GateError::SizedBuffer)?;
        out.push(&user.name).map_err(GateError::SizedBuffer)?;
        out.xfer_bytes(buf).map_err(GateError::SizedBuffer)?;

        tx.send(RoutedMessage::local(out)).map_err(|_| GateError::Client(()))
    } else {
        Err(GateError::Client(()))
    }
}

fn v_marshal(context: Arc<Mutex<Gate>>, flavor: op::Flavor, command: op::Command, tx: &UnboundedSender<RoutedMessage>, id: u8, buf: &mut SizedBuffer) -> Result<(), GateError> {
    let auth = buf.pull::<AuthType>().map_err(GateError::SizedBuffer)?;
    if let Some(user) = context.lock().unwrap().map.get(&auth) {
        let mut out = SizedBuffer::new(256);
        out.push(&op::Route::Any(flavor)).map_err(GateError::SizedBuffer)?;
        out.push(&command).map_err(GateError::SizedBuffer)?;
        out.push(&GateHeader::new(id, user.user, auth)).map_err(GateError::SizedBuffer)?;
        out.xfer_bytes(buf).map_err(GateError::SizedBuffer)?;

        tx.send(RoutedMessage::local(out)).map_err(|_| GateError::Client(()))
    } else {
        Err(GateError::Client(()))
    }
}

#[rustfmt::skip]
fn process_courtyard(context: Arc<Mutex<Gate>>, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    if let Ok(command) = buf.pull::<op::Command>() {
        let result = match command {
            op::Command::Authorize => c_authorize(context, &mut buf),
            op::Command::Message(_) => c_marshal_message(command, context, &tx, &mut buf),
            op::Command::Inventory(_) => c_marshal_inventory(command, &tx, &mut buf),
            op::Command::Game(_) => c_marshal_one(command, &tx, &mut buf),
            op::Command::NoOp
            | op::Command::Register
            | op::Command::Hello
            | op::Command::UserAttr
            => Ok(VClientMode::Continue),
        };
        result.unwrap_or_else(|err| { error!(?err); VClientMode::Continue })
    } else {
        VClientMode::Continue
    }
}

fn convert_to_v6(iface: SocketAddr) -> Ipv6Addr {
    match iface.ip() {
        IpAddr::V4(ipv4) => ipv4.to_ipv6_compatible(),
        IpAddr::V6(ipv6) => ipv6,
    }
}

fn c_authorize(context: Arc<Mutex<Gate>>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    let _ = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?; // auth (discard)

    let mut out = SizedBuffer::new(256);

    let id = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?;
    out.push(&op::Route::One(id)).map_err(GateError::SizedBuffer)?;
    out.push(&op::Command::Authorize).map_err(GateError::SizedBuffer)?;
    out.xfer::<NodeType>(buf).map_err(GateError::SizedBuffer)?;

    let user = buf.pull::<UserIdType>().map_err(GateError::SizedBuffer)?;
    let auth = buf.pull::<AuthType>().map_err(GateError::SizedBuffer)?;
    let name = buf.pull::<String>().map_err(GateError::SizedBuffer)?;

    let mut context = context.lock().unwrap();

    context.map.insert(
        auth,
        GateUser {
            name,
            user,
            vagabond: 0,
        },
    );

    let ipv6 = convert_to_v6(context.interface);
    out.push_bytes(&ipv6.octets()).map_err(GateError::SizedBuffer)?;
    out.push(&context.interface.port()).map_err(GateError::SizedBuffer)?;
    out.push(&auth).map_err(GateError::SizedBuffer)?;
    out.xfer_bytes(buf).map_err(GateError::SizedBuffer)?;

    if context.reply.send(RoutedMessage::local(out)).is_err() {
        return Ok(VClientMode::Disconnect);
    }

    let mut update = SizedBuffer::new(128);
    update.push(&op::Route::Any(op::Flavor::Jail)).map_err(GateError::SizedBuffer)?;
    update.push(&op::Command::UserAttr).map_err(GateError::SizedBuffer)?;
    update.push(&user).map_err(GateError::SizedBuffer)?;
    update.push(&"login".to_string()).map_err(GateError::SizedBuffer)?;

    let now = Utc::now().timestamp() as TimestampType;
    update.push(&now).map_err(GateError::SizedBuffer)?;

    if context.reply.send(RoutedMessage::local(update)).is_err() {
        Ok(VClientMode::Disconnect)
    } else {
        Ok(VClientMode::Continue)
    }
}

fn c_marshal_inventory(command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    if let op::Command::Inventory(sub) = command {
        match sub.into() {
            ArchiveSubCommand::InvGen => Ok(VClientMode::Continue),
            ArchiveSubCommand::InvList => c_marshal_one(command, tx, buf),
        }
    } else {
        Ok(VClientMode::Continue)
    }
}

fn c_marshal_message(command: op::Command, context: Arc<Mutex<Gate>>, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    if let op::Command::Message(sub) = command {
        match sub.into() {
            ForumSubCommand::Chat => c_marshal_name(command, context, tx, buf),
            ForumSubCommand::DM => c_marshal_all(command, tx, buf),
        }
    } else {
        Ok(VClientMode::Continue)
    }
}

fn c_marshal_name(command: op::Command, context: Arc<Mutex<Gate>>, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    let _ = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?; // forum (discard)

    let sendee = buf.pull::<String>().map_err(GateError::SizedBuffer)?;

    for (_, user) in context.lock().unwrap().map.iter_mut() {
        if user.name == sendee {
            return send_to_client(op::Route::One(user.vagabond), command, tx, buf);
        }
    }

    Ok(VClientMode::Continue)
}

fn c_marshal_one(command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    let _ = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?; // sender (discard)
    let vagabond = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?;

    send_to_client(op::Route::One(vagabond), command, tx, buf)
}

fn c_marshal_all(command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    let _ = buf.pull::<NodeType>().map_err(GateError::SizedBuffer)?; // sender (discard)

    send_to_client(op::Route::All(op::Flavor::Vagabond), command, tx, buf)
}

fn send_to_client(route: op::Route, command: op::Command, tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, GateError> {
    let mut out = SizedBuffer::new(command.size_in_buffer() + buf.read_remain());
    out.push(&command).map_err(GateError::SizedBuffer)?;
    out.xfer_bytes(buf).map_err(GateError::SizedBuffer)?;

    info!(?route, ?command, "bytes: {}", buf.size());
    if tx.send(RoutedMessage::new(route, out)).is_err() {
        error!(?command);
        Ok(VClientMode::Disconnect)
    } else {
        Ok(VClientMode::Continue)
    }
}
