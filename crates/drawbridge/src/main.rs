use tokio::signal;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use crate::DrawbridgeError::{Client, Server};
use shared_net::{IdMessage, NodeType, RoutedMessage, SizedBuffer, VClientMode, op};

#[derive(Clone)]
struct NoContext;

#[allow(dead_code)]
#[derive(Debug)]
enum DrawbridgeError {
    Interrupt,
    Client(()),
    Server(()),
}

#[tokio::main]
async fn main() -> Result<(), DrawbridgeError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());
    let iface_to_vagabond = args.next().unwrap_or("[::]:23450".to_string());

    drawbridge_main(iface_to_vagabond, iface_to_courtyard).await
}

#[instrument]
async fn drawbridge_main(interface: String, courtyard: String) -> Result<(), DrawbridgeError> {
    info!("START");

    let (d2c_tx, d2c_rx) = mpsc::unbounded_channel();
    let (d2v_tx, d2v_rx) = mpsc::unbounded_channel();
    let drawbridge = shared_net::async_server(NoContext, d2v_tx, d2c_rx, interface, process_drawbridge);
    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Drawbridge, d2c_tx, d2v_rx, courtyard, process_courtyard);

    tokio::spawn(drawbridge);
    tokio::spawn(courtyard_client);

    signal::ctrl_c().await.map_err(|_| DrawbridgeError::Interrupt)?;

    info!("END");

    Ok(())
}

fn process_drawbridge(_context: NoContext, tx: UnboundedSender<RoutedMessage>, msg: IdMessage) -> bool {
    let id = msg.id;
    let mut buf = msg.buf;
    match buf.pull::<op::Command>() {
        Ok(op::Command::Authorize) => v_authorize(&tx, id, &mut buf),
        _ => Err(Client(())),
    }
    .is_ok()
}

fn v_authorize(tx: &UnboundedSender<RoutedMessage>, id: u8, buf: &mut SizedBuffer) -> Result<(), DrawbridgeError> {
    let mut out = SizedBuffer::new(256);
    out.push(&op::Route::Any(op::Flavor::Lookout)).map_err(|_| Client(()))?;
    out.push(&op::Command::Authorize).map_err(|_| Client(()))?;
    out.push(&id).map_err(|_| Client(()))?;
    out.xfer_bytes(buf).map_err(|_| Client(()))?;

    let message = RoutedMessage {
        route: op::Route::Local,
        buf: out,
    };
    tx.send(message).map_err(|_| Client(()))
}

fn process_courtyard(_context: NoContext, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    let result = match buf.pull::<op::Command>() {
        Ok(op::Command::Authorize) => c_authorize(&tx, &mut buf),
        _ => Ok(VClientMode::Continue),
    };
    result.unwrap_or(VClientMode::Disconnect)
}

fn c_authorize(tx: &UnboundedSender<RoutedMessage>, buf: &mut SizedBuffer) -> Result<VClientMode, DrawbridgeError> {
    let mut out = SizedBuffer::new(256);
    out.push(&op::Command::Authorize).map_err(|_| Server(()))?;

    let _ = buf.pull::<NodeType>(); //discard
    let route_id = buf.pull::<NodeType>().map_err(|_| Server(()))?;

    out.xfer_bytes(buf).map_err(|_| Server(()))?;

    let message = RoutedMessage {
        route: op::Route::One(route_id),
        buf: out,
    };
    tx.send(message).map_err(|_| Server(())).map(|_| VClientMode::Continue)
}
