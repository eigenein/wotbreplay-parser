//! `data.wotreplay` models.

pub mod entity_method;
pub mod packet;
pub mod payload;
pub mod replay_header;
pub mod summary;

use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::entity_method::*;
pub use self::summary::*;
use crate::models::data::packet::Packet;
use crate::models::data::payload::Payload;
use crate::models::data::replay_header::ReplayHeader;
use crate::result::Result;
use crate::Error;

/// `data.wotreplay` root structure.
#[derive(Debug, Serialize)]
pub struct Data {
    /// For example: `9.8.5_apple`.
    pub client_version: String,

    pub replay_header: Option<ReplayHeader>,

    pub other_packets: Vec<Packet>,
}

impl Data {
    pub fn from_reader(mut reader: impl Read) -> Result<Self> {
        assert_magic(reader.read_u32::<LittleEndian>()?, 0x12345678)?;

        // No idea what it is:
        reader.read_u64::<LittleEndian>()?;

        // Some sort of client hash, e.g.: `6CF2A9EFA5C52D6F6CE43A6D4A699C05`:
        read_bytes(&mut reader)?;

        let client_version = read_string(&mut reader)?;

        // Some extra byte, no idea:
        reader.read_u8()?;

        let mut this = Self::new(client_version);

        loop {
            let Some(packet) = Packet::from_reader(&mut reader)? else { break };
            match packet.payload {
                Payload::ReplayHeader(header) => {
                    this.replay_header = Some(*header);
                }
                _ => {
                    this.other_packets.push(packet);
                }
            }
        }

        Ok(this)
    }

    pub const fn new(client_version: String) -> Self {
        Self {
            client_version,
            replay_header: None,
            other_packets: Vec::new(),
        }
    }
}

#[inline]
fn assert_magic<T: Into<u32> + PartialEq>(actual: T, expected: T) -> Result {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::InvalidMagic(actual.into(), expected.into()))
    }
}

#[inline]
fn read_bytes(reader: &mut impl Read) -> Result<Vec<u8>> {
    let length = reader.read_u8()? as usize;
    let mut buffer = Vec::new();
    buffer.resize(length, 0);
    reader.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[inline]
fn read_string(reader: &mut impl Read) -> Result<String> {
    Ok(String::from_utf8(read_bytes(reader)?)?)
}

#[inline]
fn read_pickled<T: DeserializeOwned>(reader: &mut impl Read, length: usize) -> Result<T> {
    let mut buffer = Vec::new();
    buffer.resize(length, 0);
    reader.read_exact(&mut buffer)?;
    Ok(serde_pickle::from_slice(&buffer, Default::default())?)
}
