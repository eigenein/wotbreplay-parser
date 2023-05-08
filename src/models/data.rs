//! Models for `data.wotreplay` contents.

pub mod base_player_create;
pub mod entity_method;
pub mod packet;
pub mod payload;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::data::packet::Packet;
use crate::result::Result;
use crate::Error;

/// `data.wotreplay` root structure.
#[derive(Debug, Serialize)]
pub struct Data {
    /// For example: `9.8.5_apple`.
    pub client_version: String,

    pub packets: Vec<Packet>,
}

impl Data {
    pub fn from_reader(mut reader: impl Read) -> Result<Self> {
        assert_magic(reader.read_u32::<LittleEndian>()?, 0x12345678)?;

        // No idea what it is:
        reader.read_u64::<LittleEndian>()?;

        // Some sort of client hash, e.g.: `6CF2A9EFA5C52D6F6CE43A6D4A699C05`:
        read_length_delimited(&mut reader)?;

        let client_version = read_string(&mut reader)?;

        // Some extra byte, no idea:
        reader.read_u8()?;

        let mut packets = Vec::new();
        while let Some(packet) = Packet::from_reader(&mut reader)? {
            packets.push(packet);
        }

        Ok(Self { client_version, packets })
    }
}

#[inline]
fn assert_magic<T: Into<u32> + PartialEq>(actual: T, expected: T) -> Result {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::InvalidMagic {
            actual: actual.into(),
            expected: expected.into(),
        })
    }
}

#[inline]
fn read_length_delimited(reader: &mut impl Read) -> Result<Vec<u8>> {
    let length = reader.read_u8()? as usize;
    let mut buffer = Vec::new();
    buffer.resize(length, 0);
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[inline]
fn read_string(reader: &mut impl Read) -> Result<String> {
    Ok(String::from_utf8(read_length_delimited(reader)?)?)
}

#[inline]
fn read_pickled<T: DeserializeOwned>(reader: &mut impl Read, length: usize) -> Result<T> {
    let mut buffer = Vec::new();
    buffer.resize(length, 0);
    reader.read_exact(&mut buffer)?;
    Ok(serde_pickle::from_slice(&buffer, Default::default())?)
}

/// Read 2-byte length surrounded by the magical `0xFF` and `0x00`,
/// unless it's a simple 1-byte length.
///
/// # Context
///
/// It seems that for payloads larger than 254 bytes, Wargaming writes `00 <XX XX> FF`,
/// where the word in the middle is the actual length of a payload, and the `00` and `FF`
/// are magic values.
#[inline]
fn read_quirky_length(reader: &mut impl Read) -> Result<usize> {
    match reader.read_u8()? {
        0xFF => {
            let length = reader.read_u16::<LittleEndian>()?;
            assert_magic(reader.read_u8()?, 0x00)?;
            Ok(length as usize)
        }
        length => Ok(length as usize),
    }
}
