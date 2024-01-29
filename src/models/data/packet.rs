use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::serde_as;

use crate::models::data::payload::Payload;
use crate::result::Result;
use crate::Error;

/// «Packet» is a single traffic message in a replay,
/// packets tell us what is going on during the battle.
#[serde_as]
#[derive(Debug, Serialize)]
pub struct Packet {
    /// Replay time, starting from zero.
    #[serde(rename = "clock")]
    pub clock_secs: f32,

    /// Parsed packet payload.
    pub payload: Payload,

    /// Raw packet payload, excluding the clock and packet type, and including optional subtype.
    ///
    /// This attribute requires the `raw-payload` feature.
    #[cfg(feature = "raw-payload")]
    #[serde_as(as = "serde_with::hex::Hex")]
    pub raw_payload: Vec<u8>,
}

impl Packet {
    /// Parse next packet from the reader.
    ///
    /// # Returns
    ///
    /// Parsed packet, or [`None`], when no packets are left.
    pub fn from_reader(reader: &mut impl Read) -> Result<Option<Self>> {
        let Ok(length) = reader.read_u32::<LittleEndian>() else {
            return Ok(None);
        };
        let type_ = reader.read_u32::<LittleEndian>()?;
        let clock_secs = reader.read_f32::<LittleEndian>()?;
        let raw_payload = Self::read_raw_payload(reader, length as usize)?;
        let payload = Payload::new(type_, &raw_payload).map_err(|source| {
            Error::PacketPayloadParsingError {
                source: Box::new(source),
                type_,
                clock_secs,
            }
        })?;
        let this = Self {
            clock_secs,
            payload,

            #[cfg(feature = "raw-payload")]
            raw_payload,
        };
        Ok(Some(this))
    }

    fn read_raw_payload(reader: &mut impl Read, length: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; length];
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
