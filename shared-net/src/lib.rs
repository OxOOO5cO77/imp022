use std::fmt;

pub use client::async_client;
pub use client::VClientMode;
pub use server::async_server;

#[doc(inline)]
pub use crate::sizedbuffers::VSizedBuffer;

mod client;
mod server;
pub mod op;
pub mod util;
pub mod sizedbuffers;


pub enum VRoute {
    None,
    Local,
    One(u8),
    Any(u8),
    All(u8),
}

impl VRoute {
    pub fn from_op(op: u8, arg: u8) -> Option<VRoute> {
        match op {
            op if op == op::Route::Local as u8 => Some(VRoute::None),
            op if op == op::Route::One as u8 => Some(VRoute::One(arg)),
            op if op == op::Route::Any as u8 => Some(VRoute::Any(arg)),
            op if op == op::Route::All as u8 => Some(VRoute::All(arg)),
            _ => None
        }
    }
}

impl fmt::Debug for VRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VRoute::None => write!(f, "None"),
            VRoute::Local => write!(f, "Local"),
            VRoute::One(id) => write!(f, "One({})", id),
            VRoute::Any(flavor) => write!(f, "Any({:?})", op::Flavor::from(*flavor)),
            VRoute::All(flavor) => write!(f, "All({:?})", op::Flavor::from(*flavor)),
        }
    }
}

pub struct VRoutedMessage {
    pub route: VRoute,
    pub buf: VSizedBuffer,
}

pub struct VIdMessage {
    pub id: u8,
    pub buf: VSizedBuffer,
}
