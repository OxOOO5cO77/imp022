use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{error, info, instrument};

use shared_net::{Bufferable, IdMessage, RoutedMessage, SizedBuffer, op};

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
    if let Ok(route) = in_buf.pull::<op::Route>() {
        if let Ok(command) = in_buf.pull::<op::Command>() {
            let mut out_buf = SizedBuffer::new(command.size_in_buffer() + msg.id.size_in_buffer() + in_buf.read_remain());
            let success = out_buf.push(&command).and_then(|_| out_buf.push(&msg.id)).and_then(|_| out_buf.xfer_bytes(&mut in_buf)).is_ok();

            if success {
                info!(msg.id, ?command, ?route, bytes = out_buf.size());
            } else {
                error!(msg.id, ?command, ?route, bytes = out_buf.size());
            }

            let message = RoutedMessage {
                route,
                buf: out_buf,
            };
            return tx.send(message).is_ok();
        }
    }
    false
}
