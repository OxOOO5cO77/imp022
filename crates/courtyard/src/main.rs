use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use shared_net::{op, Bufferable, IdMessage, RoutedMessage, VSizedBuffer};

#[derive(Clone)]
struct NoContext;

#[derive(Debug)]
enum CourtyardError {
    Server(()),
}

#[tokio::main]
async fn main() -> Result<(), CourtyardError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let interface = args.next().unwrap_or("[::]:12345".to_string());

    courtyard_main(interface).await
}

#[instrument]
async fn courtyard_main(interface: String) -> Result<(), CourtyardError> {
    info!("START");

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    shared_net::async_server(NoContext, dummy_tx, dummy_rx, interface, process).await.map_err(CourtyardError::Server)?;

    info!("END");

    Ok(())
}

fn process(_context: NoContext, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let mut in_buf = msg.buf;
    let route = in_buf.pull::<op::Route>();
    let command = in_buf.pull::<op::Command>();

    let mut out_buf = VSizedBuffer::new(command.size_in_buffer() + msg.id.size_in_buffer() + in_buf.remaining());
    out_buf.push(&command);
    out_buf.push(&msg.id);
    out_buf.xfer_bytes(&mut in_buf);

    info!(msg.id, ?command, ?route, bytes = out_buf.size());

    tx.send(RoutedMessage {
        route,
        buf: out_buf,
    })
    .is_ok()
}
