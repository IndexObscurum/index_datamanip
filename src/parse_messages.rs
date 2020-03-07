//! MessageStore decoder

use std::collections::HashMap;
use std::fmt::Debug;

use nom::bytes::complete::*;
use nom::number::complete::*;

use crate::error::{Error, Result};

/// Gets P-string -> display string mapping
///
/// This is a really simplistic interpretation of the MessageStore format, and
/// doesn't support most of the functionality, but is enough for handling the
/// power related data.
pub fn get_pmessages(input: &[u8]) -> Result<HashMap<String, String>> {
    let (input, _) = validate_header(input)?;
    let (input, messages) = parse_strings_list(input)?;
    // We don't currently support variables in strings
    let (input, _variables) = parse_strings_list(input)?;
    let (input, stash) = parse_stash(input)?;

    let mut pmessages = HashMap::new();
    for item in stash {
        let target = messages
            .get(item.message_index as usize)
            .ok_or_else(|| Error::ParseError(format!("{} out of range", item.message_index)))?;
        pmessages.insert(item.message_id, target.to_owned());
    }

    // TODO: Return an error instead
    assert!(input.is_empty(), "Data unexpectedly found at end of file");

    Ok(pmessages)
}

fn validate_header(input: &[u8]) -> Result<(&[u8], ())> {
    let (input, version) = le_u32(input)?;
    // TODO: Make it return an error instead
    assert!(
        version == 20_090_521,
        "Unrecognized ms version: {}",
        version
    );
    Ok((input, ()))
}

#[derive(Debug)]
struct StashItem {
    message_id: String,
    message_index: u32,
    help_index: u32,
    attributes: Vec<u32>,
}

fn parse_stash(input: &[u8]) -> Result<(&[u8], Vec<StashItem>)> {
    let (mut input, _count) = le_u32(input)?;

    // let mut input = input;
    let mut stash = vec![];
    while !input.is_empty() {
        let (new_input, str_len) = le_u32(input)?;
        let (new_input, s) = take(str_len)(new_input)?;

        let (new_input, message_index) = le_u32(new_input)?;
        let (new_input, help_index) = le_u32(new_input)?;
        let (new_input, var_count) = le_u32(new_input)?;

        let mut attributes = vec![];
        let mut new_input = new_input;
        for _ in 0..var_count {
            let (new_input2, var) = le_u32(new_input)?;
            new_input = new_input2;
            attributes.push(var);
        }

        stash.push(StashItem {
            message_id: String::from_utf8_lossy(s).to_string(),
            message_index,
            help_index,
            attributes,
        });
        input = new_input;
    }

    Ok((input, stash))
}

fn parse_strings_list(input: &[u8]) -> Result<(&[u8], Vec<String>)> {
    let (input, _count) = le_u32(input)?;
    let (input, bytes) = le_u32(input)?;
    let (ret_input, mut input) = take(bytes)(input)?;

    let mut strings = Vec::new();
    while !input.is_empty() {
        let (new_input, s) = take_while(is_nonnull)(input)?;
        let (new_input, _) = tag(b"\0")(new_input)?;
        input = new_input;
        strings.push(String::from_utf8_lossy(s).to_string());
    }

    // This format makes no effort to maintain alignment

    Ok((ret_input, strings))
}

fn is_nonnull(c: u8) -> bool {
    c != 0
}
