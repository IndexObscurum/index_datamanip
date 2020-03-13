use std::str;

use nom::bytes::complete::*;
use nom::number::complete::*;

use crate::error::Result;

/// Determine how many extra bytes are necessary to round the given length up to a 32byte alignment
pub(crate) fn needed_padding(length: usize) -> usize {
    (4 - length % 4) % 4
}

/// Reads a length-prefixed string
///
/// The string begins with a 16bit length, the contents of the string (that many bytes), and then consumes 0-3 bytes, to fix the alignment
pub(crate) fn parse_lstring(input: &[u8]) -> Result<(&[u8], String)> {
    let (input, len) = le_u16(input)?;
    let (input, str_bytes) = take(len)(input)?;

    // There's junk data at the end of most strings, 0-3 bytes of
    // it, depending on their length. Discard that much data
    let (input, _) = take(needed_padding(len as usize + std::mem::size_of::<u16>()))(input)?;

    Ok((input, str::from_utf8(str_bytes)?.to_string()))
}
