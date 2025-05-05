use mimalloc::MiMalloc;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tracing::{info, instrument};

use forum_lib::core::ForumSubCommand;
use shared_net::op::SubCommandType;
use shared_net::{NodeType, RoutedMessage, SizedBuffer, SizedBufferError, VClientMode, op};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Clone)]
struct NoContext;

#[derive(Debug)]
enum ForumError {
    Client(()),
}

#[tokio::main]
async fn main() -> Result<(), ForumError> {
    tracing_subscriber::fmt::init();

    let mut args = std::env::args();
    let _ = args.next(); // program name
    let courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    forum_main(courtyard).await
}

#[instrument]
async fn forum_main(courtyard: String) -> Result<(), ForumError> {
    info!("START");

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let courtyard_client = shared_net::async_client(NoContext, op::Flavor::Forum, dummy_tx, dummy_rx, courtyard, process_courtyard);

    courtyard_client.await.map_err(ForumError::Client)?;

    info!("END");

    Ok(())
}

fn process_courtyard(_context: NoContext, tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) -> VClientMode {
    if let Ok(op::Command::Message(subcommand)) = buf.pull::<op::Command>() {
        match subcommand.into() {
            ForumSubCommand::Chat => c_chat(tx, buf),
            ForumSubCommand::DM => c_dm(tx, buf),
        }
    }
    VClientMode::Continue
}

fn c_chat(tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) {
    if let Ok(out) = move || -> Result<SizedBuffer, SizedBufferError> {
        let _gate = buf.pull::<NodeType>()?; // discard gate id

        let mut out = SizedBuffer::new(256);
        out.push(&op::Route::All(op::Flavor::Gate))?;
        out.push(&op::Command::Message(ForumSubCommand::Chat as SubCommandType))?;
        out.xfer_bytes(&mut buf)?;
        Ok(out)
    }() {
        let _ = tx.send(out.into());
    }
}

fn c_dm(tx: UnboundedSender<RoutedMessage>, mut buf: SizedBuffer) {
    if let Ok(out) = move || -> Result<SizedBuffer, SizedBufferError> {
        let _gate = buf.pull::<NodeType>()?;

        let sender = buf.pull::<String>()?;
        let sendee = buf.pull::<String>()?;

        let mut out = SizedBuffer::new(256);
        out.push(&op::Route::All(op::Flavor::Gate))?;
        out.push(&op::Command::Message(ForumSubCommand::DM as SubCommandType))?;
        out.push(&sendee)?;
        out.push(&sender)?;
        out.xfer_bytes(&mut buf)?;
        Ok(out)
    }() {
        let _ = tx.send(out.into());
    }
}
