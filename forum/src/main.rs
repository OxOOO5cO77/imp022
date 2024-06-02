use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoute, VRoutedMessage, VSizedBuffer};
use shared_net::op;

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Forum, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}

fn process_courtyard(_context: NoContext, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        op::Command::Chat => c_chat(tx, buf),
        op::Command::DM => c_dm(tx, buf),
        _ => {}
    }
    VClientMode::Continue
}

fn c_chat(tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let _gate = buf.pull_u8(); // discard gate

    let mut out = VSizedBuffer::new(256);
    out.push_route(op::Route::All);
    out.push_flavor(op::Flavor::Gate);
    out.push_command(op::Command::Chat);
    out.xfer_bytes(&mut buf);

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
}

fn c_dm(tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let _gate = buf.pull_u8();

    let sender = buf.pull_string();
    let sendee = buf.pull_string();

    let mut out = VSizedBuffer::new(256);
    out.push_route(op::Route::All);
    out.push_flavor(op::Flavor::Gate);
    out.push_command(op::Command::DM);
    out.push_string(&sendee);
    out.push_string(&sender);
    out.xfer_bytes(&mut buf);

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
}
