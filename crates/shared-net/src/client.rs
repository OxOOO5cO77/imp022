use std::io::ErrorKind;
use std::net::{SocketAddr, ToSocketAddrs};

use crate::op;
use crate::util::write_buf;
use crate::{RoutedMessage, SizedBuffer};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::signal;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tracing::error;

#[derive(PartialEq)]
pub enum VClientMode {
    Continue,
    Disconnect,
    Shutdown,
}

type FnProcess<T> = fn(context: T, UnboundedSender<RoutedMessage>, msg: SizedBuffer) -> VClientMode;

pub async fn async_client<T>(context: T, flavor: op::Flavor, external_tx: UnboundedSender<RoutedMessage>, mut external_rx: UnboundedReceiver<RoutedMessage>, interface: String, process: FnProcess<T>) -> Result<(), ()>
where
    T: Clone,
{
    let mut addr = interface.to_socket_addrs().expect("Invalid interface for async_client");
    let addr = addr.next().unwrap();

    while let Some(mut active_connection) = handle_client_connection(&addr, flavor).await {
        let mut buf = [0_u8; SizedBuffer::sizesize()];
        let mode = loop {
            tokio::select! {
                read_result = active_connection.read_exact(&mut buf[..]) => {
                    match read_result {
                        Ok(bytes) => {
                            if bytes == SizedBuffer::sizesize() {
                                let expected_bytes = SizedBuffer::extract_size(&buf);
                                let mut sized_buf = SizedBuffer::new(expected_bytes);
                                match active_connection.read_exact(&mut sized_buf.raw[SizedBuffer::sizesize()..]).await {
                                    Ok(bytes) => {
                                        if bytes != expected_bytes {
                                            error!("Bytes:{} Expected:{}", bytes, expected_bytes);
                                            break VClientMode::Shutdown;
                                        }
                                        sized_buf.set_size(expected_bytes);
                                        let mode = process(context.clone(), external_tx.clone(), sized_buf);
                                        if mode != VClientMode::Continue {
                                            break mode;
                                        }
                                    }
                                    Err(_) => {
                                        break VClientMode::Shutdown;
                                    }
                                }
                            } else if bytes == 0 {
                                break VClientMode::Shutdown;
                            }
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => {}
                        Err(_) => break VClientMode::Shutdown,
                    }
                }
                Some(msg) = external_rx.recv() => {
                    if write_buf(&mut active_connection, &msg.buf).await.is_err() {
                        break VClientMode::Shutdown;
                    }
                }
                _ = signal::ctrl_c() => {
                    break VClientMode::Shutdown;
                }
            }
        };

        if mode != VClientMode::Continue {
            let _ = active_connection.shutdown().await;
            if mode == VClientMode::Shutdown {
                return Err(());
            }
        }
    }

    Ok(())
}

async fn handle_client_connection(addr: &SocketAddr, flavor: op::Flavor) -> Option<TcpStream> {
    loop {
        if let Ok(mut stream) = TcpStream::connect(addr).await {
            let mut buf = SizedBuffer::new(32);
            buf.push(&op::Command::Register).ok()?;
            buf.push(&flavor).ok()?;

            if write_buf(&mut stream, &buf).await.is_err() {
                let _ = stream.shutdown().await;
                break;
            } else {
                return Some(stream);
            }
        } else {
            sleep(Duration::from_secs(5)).await;
        }
    }

    None
}
