pub use client::async_client;
pub use client::VClientMode;
pub use server::async_server;

use crate::op::Route;
#[doc(inline)]
pub use crate::sizedbuffers::VSizedBuffer;

mod client;
mod server;
pub mod op;
pub mod util;
pub mod sizedbuffers;

#[derive(Clone)]
pub struct RoutedMessage {
    pub route: Route,
    pub buf: VSizedBuffer,
}

pub struct IdMessage {
    pub id: u8,
    pub buf: VSizedBuffer,
}
