use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VIdMessage, VRoute, VRoutedMessage, VSizedBuffer};

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() {
    let mut args = env::args();
    let _ = args.next(); // program name
    let interface = args.next().unwrap_or("[::]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let _ = shared_net::async_server(NoContext, dummy_tx, dummy_rx, interface, process).await;
}

fn process(_context: NoContext, tx: UnboundedSender<VRoutedMessage>, msg: VIdMessage) -> bool {
    let mut in_buf = msg.buf;
    let op = in_buf.pull_u8();
    let arg = in_buf.pull_u8();

    if let Some(route) = VRoute::from_op(op, arg) {
        let mut buf = VSizedBuffer::new(in_buf.remaining() + 2);
        buf.xfer_u8(&mut in_buf);
        buf.push_u8(&msg.id);
        buf.xfer_bytes(&mut in_buf);

        println!("[Courtyard] [{}] {:?} ==> {:?} **{} bytes**", msg.id, buf.pull_command(), route, buf.size());

        tx.send(VRoutedMessage { route, buf }).is_ok()
    } else {
        false
    }
}
