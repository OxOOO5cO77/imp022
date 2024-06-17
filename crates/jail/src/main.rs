use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use shared_data::types::{NodeType, TimestampType, UserType};

use shared_net::{VClientMode, RoutedMessage, VSizedBuffer};
use shared_net::op;
use shared_net::op::Flavor;

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("[Jail] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, Flavor::Jail, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    let result = courtyard_client.await;

    println!("[Jail] END");

    result
}

fn process_courtyard(_context: NoContext, _tx: UnboundedSender<RoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    if let op::Command::UserAttr = buf.pull::<op::Command>() {
        c_userattr(buf)
    }
    VClientMode::Continue
}

fn c_userattr(mut buf: VSizedBuffer) {
    let _ = buf.pull::<NodeType>(); // gate (discard)

    let user = buf.pull::<UserType>();
    let attr = buf.pull::<String>();
    let time = buf.pull::<TimestampType>();

    println!("[{}] {}: {}", user, attr, time);
}
