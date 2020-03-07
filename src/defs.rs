//! Type-agnostic bin file parser (only Parse7 currently supported)

use std::collections::HashMap;
use std::fmt::Debug;
use std::str;

use nom::bytes::complete::*;
use nom::number::complete::*;
use serde::de::DeserializeOwned;

use crate::parse7;

use crate::error::Result;

/// Deserialize the bin data into a vec of T
///
/// Only Parse7 (HC, SCoRE) encoded bins are supported
///
/// TODO: This really should be an iterator, not a vec. It would offer better
/// performance if data is immediately transformed, and be trivially collected
/// into a vec if that's all that's desired
pub fn decode<T: DeserializeOwned + Debug>(input: &[u8]) -> Result<Vec<T>> {
    let (input, text_size) = parse_header(input)?;
    let (input, strings) = parse_strings_list(input, text_size)?;
    let (input, items) = parse_binary(input, &strings)?;

    // TODO: Change to erroring out
    assert!(input.is_empty(), "Data unexpectedly found at end of file");

    Ok(items)
}

fn is_nonnull(c: u8) -> bool {
    c != 0
}

fn parse_binary<'a, T: DeserializeOwned + Debug>(
    input: &'a [u8],
    strings: &HashMap<u32, String>,
) -> Result<(&'a [u8], Vec<T>)> {
    let (input, _binary_size) = le_u32(input)?;
    let (input, count) = le_u32(input)?;

    let mut input = input;
    let mut items = vec![];
    for i in 0..count {
        let (_, chunk_size) = le_u32(input)?;
        let (new_input, chunk) = take(chunk_size + 4)(input)?;
        input = new_input;

        match parse7::parse::<T>(chunk, &strings) {
            Ok(item) => items.push(item),
            Err(err) => eprintln!("Failed to parse binary index {}: {}", i, err),
        };
    }

    // TODO: Make an error (or just a warning?)
    assert!(
        input.is_empty(),
        "Shouldn't be any data left over after parsing the binary section"
    );

    Ok((input, items))
}

fn parse_strings_list(input: &[u8], text_size: u32) -> Result<(&[u8], HashMap<u32, String>)> {
    let (ret_input, mut input) = take(text_size)(input)?;

    let mut strings = HashMap::<u32, String>::new();
    let mut offset = 0u32;
    while !input.is_empty() {
        let (new_input, s) = take_while(is_nonnull)(input)?;
        let (new_input, _) = tag(b"\0")(new_input)?;
        input = new_input;
        strings.insert(offset, String::from_utf8_lossy(s).to_string());
        offset += 1u32 + s.len() as u32;
    }

    // Fix our alignment
    let (ret_input, _) = take(needed_padding(text_size as usize))(ret_input)?;

    Ok((ret_input, strings))
}

/// Determine how many extra bytes are necessary to round the given length up to a 32byte alignment
fn needed_padding(length: usize) -> usize {
    (4 - length % 4) % 4
}

/// Reads a length-prefixed string
///
/// The string begins with a 16bit length, the contents of the string (that many bytes), and then consumes 0-3 bytes, to fix the alignment
fn parse_lstring(input: &[u8]) -> Result<(&[u8], String)> {
    let (input, len) = le_u16(input)?;
    let (input, str_bytes) = take(len)(input)?;

    // There's junk data at the end of most strings, 0-3 bytes of
    // it, depending on their length. Discard that much data
    let (input, _) = take(needed_padding(len as usize + std::mem::size_of::<u16>()))(input)?;

    Ok((input, str::from_utf8(str_bytes)?.to_string()))
}

fn parse_header(input: &[u8]) -> Result<(&[u8], u32)> {
    let (input, _) = tag(b"CrypticS")(input)?;

    // Extract checksum and ignore it
    let (input, _addler32) = le_u32(input)?;

    let (input, sig) = parse_lstring(input)?;
    // TODO: Make an error
    assert!(sig == "Parse7", "Failed to find Parse7 sig: Found: {}", sig);

    // let (input, sig) = parse_lstring(input)?;
    // assert!(sig == "Files1", "Failed to find Files1 sig");

    let (input, names_size) = le_u32(input)?;

    Ok((input, names_size))
}
