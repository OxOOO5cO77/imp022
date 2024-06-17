use std::env;

use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use shared_data::types::NodeType;

use shared_net::{op, VClientMode, IdMessage, RoutedMessage, VSizedBuffer};

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() {
    println!("[Drawbridge] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let iface_to_vagabond = args.next().unwrap_or("[::]:23450".to_string());

    let (d2c_tx, d2c_rx) = mpsc::unbounded_channel();
    let (d2v_tx, d2v_rx) = mpsc::unbounded_channel();
    let drawbridge = shared_net::async_server(NoContext, d2v_tx, d2c_rx, iface_to_vagabond, process_drawbridge);
    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Drawbridge, d2c_tx, d2v_rx, iface_to_courtyard, process_courtyard);

    tokio::spawn(drawbridge);
    tokio::spawn(courtyard_client);

    let _ = signal::ctrl_c().await;

    println!("[Drawbridge] END");
}

fn process_drawbridge(_context: NoContext, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let id = msg.id;
    let mut buf = msg.buf;
    match buf.pull::<op::Command>() {
        op::Command::Authorize => v_authorize(&tx, id, &mut buf),
        _ => false
    }
}

fn v_authorize(tx: &UnboundedSender<RoutedMessage>, id: u8, buf: &mut VSizedBuffer) -> bool {
    let mut out = VSizedBuffer::new(256);
    out.push(&op::Route::Any(op::Flavor::Lookout));
    out.push(&op::Command::Authorize);
    out.push(&id);
    out.xfer_bytes(buf);

    tx.send(RoutedMessage { route: op::Route::Local, buf: out }).is_ok()
}

fn process_courtyard(_context: NoContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if let op::Command::Authorize = buf.pull::<op::Command>() {
        c_authorize(&tx, &mut buf)
    } else {
        VClientMode::Continue
    }
}

fn c_authorize(tx: &UnboundedSender<RoutedMessage>, buf: &mut VSizedBuffer) -> VClientMode {
    let mut out = VSizedBuffer::new(256);
    out.push(&op::Command::Authorize);

    let _ = buf.pull::<NodeType>();//discard
    let route_id = buf.pull::<NodeType>();

    out.xfer_bytes(buf);

    if tx.send(RoutedMessage { route: op::Route::One(route_id), buf: out }).is_err() {
        VClientMode::Disconnect
    } else {
        VClientMode::Continue
    }
}
