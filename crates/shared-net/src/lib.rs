mod client;
mod server;
mod sizedbuffers;
mod types;
mod util;

pub mod op;

pub use bufferable_derive;
pub use client::{async_client, VClientMode};
pub use server::async_server;
pub use sizedbuffers::{Bufferable, VSizedBuffer};
pub use types::*;

#[derive(Clone)]
pub struct RoutedMessage {
    pub route: op::Route,
    pub buf: VSizedBuffer,
}

pub struct IdMessage {
    pub id: u8,
    pub buf: VSizedBuffer,
}
