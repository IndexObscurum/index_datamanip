//! Parser for CoH data (Piggs + MessageStore + Bins)
#![warn(missing_docs)]

mod common;
pub mod defs;
pub mod error;
pub mod objects;
mod parse7;
pub mod parse_messages;
mod pigg;

pub use pigg::Pigg;
