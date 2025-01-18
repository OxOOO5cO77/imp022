use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use shared_net::{op, NodeType, RoutedMessage, SizedBuffer, SizedBufferError, TimestampType, UserIdType, VClientMode};

#[derive(Clone)]
struct NoContext;

#[allow(dead_code)]
#[derive(Debug)]
enum JailError {
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), JailError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    jail_main(courtyard).await
}

#[instrument]
async fn jail_main(courtyard: String) -> Result<(), JailError> {
    info!("START");

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Jail, dummy_tx, dummy_rx, courtyard, process_courtyard);

    courtyard_client.await.map_err(JailError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(_context: NoContext, _tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    let _result = match buf.pull::<op::Command>() {
        Ok(op::Command::UserAttr) => c_userattr(buf),
        _ => Ok(()),
    };
    VClientMode::Continue
}

fn c_userattr(mut buf: SizedBuffer) -> Result<(), SizedBufferError> {
    let _ = buf.pull::<NodeType>(); // gate (discard)

    let user = buf.pull::<UserIdType>()?;
    let attr = buf.pull::<String>()?;
    let time = buf.pull::<TimestampType>()?;

    info!(user, attr, time);
    Ok(())
}
