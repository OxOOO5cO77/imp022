mod client;
mod server;
mod sizedbuffers;
mod types;
mod util;

pub mod op;

pub use bufferable_derive::Bufferable;
pub use client::{async_client, VClientMode};
pub use server::async_server;
pub use sizedbuffers::{Bufferable, SizedBuffer, SizedBufferError};
pub use types::*;

#[derive(Clone)]
pub struct RoutedMessage {
    pub route: op::Route,
    pub buf: SizedBuffer,
}

impl RoutedMessage {
    pub fn new(route: op::Route, buf: SizedBuffer) -> Self {
        Self {
            route,
            buf,
        }
    }

    pub fn local(buf: SizedBuffer) -> Self {
        Self {
            route: op::Route::Local,
            buf,
        }
    }
}

impl From<SizedBuffer> for RoutedMessage {
    fn from(value: SizedBuffer) -> Self {
        RoutedMessage::new(op::Route::None, value)
    }
}

pub struct IdMessage {
    pub id: u8,
    pub buf: SizedBuffer,
}
