//! Parser for CoH data (Piggs + MessageStore + Bins)
#![warn(missing_docs)]

pub mod defs;
pub mod error;
mod parse7;
pub mod parse_messages;
mod pigg;

pub use pigg::Pigg;
