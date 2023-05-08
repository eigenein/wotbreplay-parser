//! Structures for «entity method» call packets.

use byteorder::{LittleEndian, ReadBytesExt};
use prost::encoding::decode_varint;
use prost::Message;
use serde::Serialize;
use serde_with::serde_as;

use self::update_arena::UpdateArena;
use crate::models::data::read_quirky_length;
use crate::result::Result;

pub mod update_arena;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum EntityMethod {
    /// Subtype 47.
    UpdateArena {
        field_number: u64,
        arguments: UpdateArena,
    },

    /// Default variant when subtype is not known by the parser.
    Unknown { sub_type: u32 },
}

impl EntityMethod {
    /// Parse the entity method raw payload.
    pub fn new(raw_payload: &[u8]) -> Result<Self> {
        let mut raw_payload = raw_payload;

        raw_payload.read_u32::<LittleEndian>()?;
        let sub_type = raw_payload.read_u32::<LittleEndian>()?;

        let this = match sub_type {
            47 => {
                let _inner_length = raw_payload.read_u32::<LittleEndian>()?;

                let field_number = decode_varint(&mut raw_payload)?;
                let message_length = read_quirky_length(&mut raw_payload)?;
                Self::UpdateArena {
                    field_number,
                    arguments: UpdateArena::decode(&raw_payload[..message_length])?,
                }
            }

            _ => Self::Unknown { sub_type },
        };
        Ok(this)
    }
}
