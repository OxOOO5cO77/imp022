use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use shared_net::types::NodeType;

use shared_net::{op, VClientMode, RoutedMessage, VSizedBuffer};

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Forum] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Forum, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);
    
    let result = courtyard_client.await;

    println!("[Forum] END");

    result
}

fn process_courtyard(_context: NoContext, tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull::<op::Command>() {
        op::Command::Chat => c_chat(tx, buf),
        op::Command::DM => c_dm(tx, buf),
        _ => {}
    }
    VClientMode::Continue
}

fn c_chat(tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let _gate = buf.pull::<NodeType>(); // discard gate id

    let mut out = VSizedBuffer::new(256);
    out.push(&op::Route::All(op::Flavor::Gate));
    out.push(&op::Command::Chat);
    out.xfer_bytes(&mut buf);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}

fn c_dm(tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) {
    let _gate = buf.pull::<NodeType>();

    let sender = buf.pull::<String>();
    let sendee = buf.pull::<String>();

    let mut out = VSizedBuffer::new(256);
    out.push(&op::Route::All(op::Flavor::Gate));
    out.push(&op::Command::DM);
    out.push(&sendee);
    out.push(&sender);
    out.xfer_bytes(&mut buf);

    let _ = tx.send(RoutedMessage { route: op::Route::None, buf: out });
}
