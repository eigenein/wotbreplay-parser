use byteorder::{LittleEndian, ReadBytesExt};
use prost::encoding::decode_varint;
use prost::Message;
use serde::Serialize;
use serde_with::serde_as;

use crate::models::data::read_quirky_length;
use crate::models::data::update_arena::UpdateArena;
use crate::result::Result;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum EntityMethod {
    Subtype2F {
        field_number: u64,
        sub_type_2f: UpdateArena,

        /// Original payload for investigation.
        #[serde_as(as = "serde_with::hex::Hex")]
        payload: Vec<u8>,
    },

    /// Default variant when subtype is not known by the parser.
    Unknown {
        sub_type: u32,

        /// Whole packet payload (including the sub-type).
        #[serde_as(as = "serde_with::hex::Hex")]
        payload: Vec<u8>,
    },
}

impl EntityMethod {
    pub fn new(payload: Vec<u8>) -> Result<Self> {
        let mut reader = payload.as_slice();

        reader.read_u32::<LittleEndian>()?;
        let sub_type = reader.read_u32::<LittleEndian>()?;

        let this = match sub_type {
            0x2F => {
                let _inner_length = reader.read_u32::<LittleEndian>()?;

                let field_number = decode_varint(&mut reader)?;
                let message_length = read_quirky_length(&mut reader)?;
                Self::Subtype2F {
                    field_number,
                    sub_type_2f: UpdateArena::decode(&reader[..message_length])?,
                    payload,
                }
            }

            _ => Self::Unknown { sub_type, payload },
        };
        Ok(this)
    }
}
