use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, WriteHalf};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{VIdMessage, VRoute, VRoutedMessage, VSizedBuffer};
use crate::op;
use crate::util::write_buf;

struct VConnection<T> {
    write: WriteHalf<T>,
    flavor: Option<op::Flavor>,
}

type VConnectionMap<T> = HashMap<u8, VConnection<T>>;

type FnProcess<T> = fn(context: T, UnboundedSender<VRoutedMessage>, msg: VIdMessage) -> bool;

pub async fn async_server<T>(context: T, external_tx: UnboundedSender<VRoutedMessage>, mut external_rx: UnboundedReceiver<VRoutedMessage>, interface: String, process: FnProcess<T>) -> Result<(), ()> where T: Clone {
    let listener = TcpListener::bind(interface).await.unwrap();

    let connections = Arc::new(Mutex::new(VConnectionMap::new()));
    let mut last_id = 0_u8;

    let (incoming_tx, mut incoming_rx) = mpsc::unbounded_channel();
    let (outgoing_tx, mut outgoing_rx) = mpsc::unbounded_channel();

    let mut cleanup_needed = Vec::new();

    loop {
        tokio::select! {
            result = listener.accept() => {
                // handle connections
                let stream = match result {
                    Ok((socket,_)) => socket,
                    Err(_) => continue,
                };

                let local_addr = stream.local_addr().unwrap();
                let (mut read, write) = tokio::io::split(stream);

                let connection = VConnection {
                    write,
                    flavor: None,
                };

                let mut connections = connections.lock().await;
                let id = match next_available_id(&connections, last_id) {
                    Ok(id) => id,
                    Err(_) => continue,
                };
                println!("Connection {} from {}", id, local_addr);
                connections.insert(id, connection);
                last_id = id;

                let incoming_tx = incoming_tx.clone();

                tokio::spawn( async move {
                    loop {
                        let mut size_buf = [0_u8; VSizedBuffer::sizesize()];
                        let error = match read.read(&mut size_buf[..]).await {
                            Ok(bytes) => {
                                let mut error = false;
                                if bytes == VSizedBuffer::sizesize() {
                                    let expected_bytes = VSizedBuffer::extract_size(&size_buf);
                                    let mut buf = VSizedBuffer::new(expected_bytes);

                                    error = match read.read(&mut buf.raw[VSizedBuffer::sizesize()..]).await {
                                        Ok(bytes) => {

                                            if bytes != expected_bytes {
                                                println!("Bytes:{} Expected:{}", bytes, expected_bytes);
                                                true
                                            } else {
                                                buf.set_size(expected_bytes);
                                                incoming_tx.send( VIdMessage { id, buf } ).is_err()
                                            }
                                        }
                                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => false,
                                        Err(_) => true,
                                    }
                                } else if bytes == 0 {
                                    error = true;
                                }
                                error
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => false,
                            Err(_) => true,
                        };
                        if error {
                            break;
                        }
                    }
                });
            }
            Some(msg) = incoming_rx.recv() => {
                let mut msg = msg;
                let id = msg.id;
                let builtin = msg.buf.pull_command();
                let is_ok = match builtin {
                    op::Command::NoOp => false,
                    op::Command::Register => {
                        let flavor = msg.buf.pull_flavor();
                        let mut connections = connections.lock().await;
                        if let Some(cx) = connections.get_mut(&id) {
                            cx.flavor = Some(flavor);
                        }
                        println!("Registered {} as {:?}", id, flavor);
                        let mut out = VSizedBuffer::new(32);
                        out.push_command(op::Command::Hello);
                        outgoing_tx.send(VRoutedMessage { route: VRoute::One(id), buf: out }).is_ok()
                    }
                    _ => {
                        msg.buf.rewind();
                        process( context.clone(), outgoing_tx.clone(), msg )
                    }
                };
                if !is_ok {
                    cleanup_needed.push(id);
                }

            }
            Some(msg) = external_rx.recv() => {
                let _ = outgoing_tx.send(msg);
            }
            Some(msg) = outgoing_rx.recv() => {
                // handle outgoing
                match msg.route {
                    VRoute::Local => {
                        let _ = external_tx.send( msg );
                    }
                    VRoute::One(msg_id) => {
                        let msg_buf = msg.buf;
                        let mut connections = connections.lock().await;

                        if let Some(cx) = connections.get_mut(&msg_id) {
                            if write_buf(&mut cx.write, &msg_buf).await.is_err() {
                                cleanup_needed.push(msg_id);

                            }
                        }
                    }
                    VRoute::Any(flavor) => {
                        let flavor = op::Flavor::from(flavor);
                        let msg_buf = msg.buf;
                        let mut connections = connections.lock().await;
                        if let Some((id,cx)) = connections.iter_mut().find(|(_,cx)| cx.flavor == Some(flavor)) {
                            if write_buf(&mut cx.write, &msg_buf).await.is_err() {
                                cleanup_needed.push(*id);
                            }
                        }
                    }
                    VRoute::All(flavor) => {
                        let flavor = op::Flavor::from(flavor);
                        let msg_buf = msg.buf;
                        let mut connections = connections.lock().await;
                        for (id,cx) in connections.iter_mut().filter(|(_,cx)| cx.flavor == Some(flavor)) {
                            if write_buf(&mut cx.write, &msg_buf).await.is_err() {
                                cleanup_needed.push(*id);
                            }
                        }
                    }
                    VRoute::None => {}
                }
            }
            _ = signal::ctrl_c() => {
                cleanup_needed.clear();
                connections.lock().await.clear();
                return Err(())
            },
        }

        if !cleanup_needed.is_empty() {
            let mut connections = connections.lock().await;
            connections.retain(|id, _| !cleanup_needed.contains(id));
            cleanup_needed.clear();
        }
    }
}

fn next_available_id<T>(connections: &VConnectionMap<T>, last_id: u8) -> Result<u8, ()> {
    let mut id = last_id;

    loop {
        id = id.wrapping_add(1);

        if id == last_id {
            break;
        }

        if connections.contains_key(&id) {
            continue;
        }

        return Ok(id);
    }

    Err(())
}
