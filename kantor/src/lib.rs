#![warn(missing_docs)]

//! A crate for distributed systems

mod aid;
pub mod message;
pub mod node;
pub mod proxy;

pub use aid::*;
