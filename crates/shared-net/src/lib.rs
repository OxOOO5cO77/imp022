pub use client::async_client;
pub use client::VClientMode;
pub use server::async_server;

use crate::op::Route;
#[doc(inline)]
pub use crate::sizedbuffers::VSizedBuffer;
#[doc(inline)]
pub use bufferable_derive;

mod client;
pub mod op;
mod server;
pub mod sizedbuffers;
pub mod types;
pub mod util;

#[derive(Clone)]
pub struct RoutedMessage {
    pub route: Route,
    pub buf: VSizedBuffer,
}

pub struct IdMessage {
    pub id: u8,
    pub buf: VSizedBuffer,
}
