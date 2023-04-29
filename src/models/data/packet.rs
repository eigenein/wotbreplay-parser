use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;

use crate::models::data::payload::Payload;
use crate::result::Result;

#[derive(Debug, Serialize)]
pub struct Packet {
    pub clock: f32,
    pub payload: Payload,
}

impl Packet {
    pub fn from_reader(reader: &mut impl Read) -> Result<Option<Self>> {
        let Ok(length) = reader.read_u32::<LittleEndian>() else { return Ok(None) };
        let type_ = reader.read_u32::<LittleEndian>()?;
        let clock = reader.read_f32::<LittleEndian>()?;
        let payload = Self::read_payload(reader, length as usize)?;
        let payload = Payload::new(type_, payload)?;
        let this = Self { clock, payload };
        Ok(Some(this))
    }

    fn read_payload(reader: &mut impl Read, length: usize) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        buffer.resize(length, 0);
        reader.read_exact(&mut buffer)?;
        Ok(buffer)
    }
}
