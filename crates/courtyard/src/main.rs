use std::env;

use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{op, IdMessage, RoutedMessage, VSizedBuffer};
use shared_net::sizedbuffers::Bufferable;

#[derive(Clone)]
struct NoContext;

#[tokio::main]
async fn main() {
    println!("[Courtyard] START");

    let mut args = env::args();
    let _ = args.next(); // program name
    let interface = args.next().unwrap_or("[::]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let _ = shared_net::async_server(NoContext, dummy_tx, dummy_rx, interface, process).await;

    println!("[Courtyard] END");
}

fn process(_context: NoContext, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let mut in_buf = msg.buf;
    let route = in_buf.pull::<op::Route>();
    let command = in_buf.pull::<op::Command>();

    let mut out_buf = VSizedBuffer::new(command.size_in_buffer() + msg.id.size_in_buffer() + in_buf.remaining());
    out_buf.push(&command);
    out_buf.push(&msg.id);
    out_buf.xfer_bytes(&mut in_buf);

    println!("[Courtyard] [{}] {:?} ==> {:?} **{} bytes**", msg.id, command, route, out_buf.size());

    tx.send(RoutedMessage { route, buf: out_buf }).is_ok()
}
