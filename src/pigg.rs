use std::collections::HashMap;
use std::convert::AsRef;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str;

use libflate::zlib;
use memmap::MmapOptions;
use nom::{bytes::complete::*, number::complete::*};

use crate::error::{Error, Result};

/// PIGG extractor
#[derive(Debug)]
pub struct Pigg {
    files: HashMap<String, FileHeader>,
    mmap: memmap::Mmap,
}

impl Pigg {
    /// Opens and parses the given pigg file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Pigg> {
        let f = File::open(path)?;
        // TODO: Document this
        let mmap = unsafe { MmapOptions::new().map(&f)? };
        let (mut remaining, header) = parse_header(&mmap[..])?;

        let mut file_headers = vec![];
        for _ in 0..header.num_files {
            let (new_remaining, file_header) = parse_file_header(&remaining)?;
            remaining = new_remaining;
            file_headers.push(file_header);
        }

        let (_, string_pool) = StringPool::with(remaining)?;

        let files: HashMap<_, _> = string_pool
            .strings
            .into_iter()
            .zip(file_headers.into_iter())
            .collect();

        Ok(Pigg { files, mmap })
    }

    /// Gets the data chunk for the given path from the pigg file
    pub fn get_data(&self, path: &str) -> Result<Vec<u8>> {
        // TODO: Would this be better served as returning a Cow instead?
        // Return a Cow::Borrowed in the case we can just point at the map,
        // and return a Cow::Owned if we need to decompress
        let header = self
            .files
            .get(path)
            .ok_or_else(|| Error::ItemNotFound(path.into()))?;

        // TODO: Only decompress if if header.size bigger than header.pack_size
        let begin = header.offset as usize;
        let end = begin + header.pack_size as usize;
        let mut data = Vec::new();
        let mut decoder = zlib::Decoder::new(&self.mmap[begin..end])?;
        decoder.read_to_end(&mut data)?;

        Ok(data)
    }

    // TODO: Add a method to list the "files"
}

fn parse_header(input: &[u8]) -> Result<(&[u8], Header)> {
    let (input, _) = tag(&0x123u32.to_le_bytes())(input)?;

    let (input, creator_version) = le_u16(input)?;
    let (input, required_read_version) = le_u16(input)?;
    let (input, archive_header_size) = le_u16(input)?;
    let (input, file_header_size) = le_u16(input)?;
    let (input, num_files) = le_u32(input)?;

    Ok((
        input,
        Header {
            creator_version,
            required_read_version,
            archive_header_size,
            file_header_size,
            num_files,
        },
    ))
}

#[derive(Debug)]
struct Header {
    creator_version: u16,
    required_read_version: u16,
    archive_header_size: u16,
    file_header_size: u16,
    num_files: u32,
}

fn parse_file_header(input: &[u8]) -> Result<(&[u8], FileHeader)> {
    let (input, _) = tag(&0x3456u32.to_le_bytes())(input)?;
    let (input, name_id) = le_i32(input)?;
    let (input, size) = le_u32(input)?;
    let (input, timestamp) = le_u32(input)?;
    let (input, offset) = le_u32(input)?;
    let (input, reserved) = le_u32(input)?;
    let (input, header_data_id) = le_u32(input)?;
    let (input, checksum) = le_u128(input)?;
    let (input, pack_size) = le_u32(input)?;

    Ok((
        input,
        FileHeader {
            name_id,
            size,
            timestamp,
            offset,
            reserved,
            header_data_id,
            checksum,
            pack_size,
        },
    ))
}

#[derive(Debug)]
struct FileHeader {
    name_id: i32,
    size: u32,
    timestamp: u32,
    offset: u32,
    reserved: u32,
    header_data_id: u32,
    checksum: u128,
    pack_size: u32,
}

#[derive(Debug)]
struct StringPool {
    strings: Vec<String>,
}

impl StringPool {
    fn with(input: &[u8]) -> Result<(&[u8], StringPool)> {
        let (mut input, (num_strings, _pool_size)) = Self::get_header(input)?;
        // TODO: Actually validate pool size?

        let mut strings = vec![];

        for _ in 0..num_strings {
            let (remaining, s) = Self::read_string(input)?;
            strings.push(s);
            input = remaining;
        }

        Ok((input, StringPool { strings }))
    }

    fn get_header(input: &[u8]) -> Result<(&[u8], (u32, u32))> {
        let (input, _) = tag(&0x6789u32.to_le_bytes())(input)?;
        let (input, num_strings) = le_u32(input)?;
        let (input, pool_size) = le_u32(input)?;
        Ok((input, (num_strings, pool_size)))
    }

    fn read_string(input: &[u8]) -> Result<(&[u8], String)> {
        let (input, str_length) = le_u32(input)?;
        let (input, str_bytes) = take(str_length - 1)(input)?;
        let (input, _) = tag(&0u8.to_le_bytes())(input)?;

        // Probably should change this to from_utf8_lossy
        Ok((input, str::from_utf8(str_bytes)?.to_string()))
    }
}
