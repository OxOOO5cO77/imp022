use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoutedMessage, VSizedBuffer};
use shared_net::op;
use shared_net::op::Flavor;

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, Flavor::Jail, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}

fn process_courtyard(_context: NoContext, _tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if let op::Command::UserAttr = buf.pull_command() {
        c_userattr(buf)
    }
    VClientMode::Continue
}

fn c_userattr(mut buf: VSizedBuffer) {
    let _ = buf.pull_u8(); // gate (discard)

    let user = buf.pull_u128();
    let attr = buf.pull_string();
    let time = buf.pull_u128();

    println!("[{}] {}: {}", user, attr, time);
}
