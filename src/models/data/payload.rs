use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::serde_as;

use crate::models::data::replay_header::ReplayHeader;
use crate::models::data::{assert_magic, read_pickled, read_string, EntityMethod};
use crate::result::Result;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Payload {
    ReplayHeader(ReplayHeader),
    EntityMethod(EntityMethod),

    /// Default payload when type is not known.
    Unknown {
        type_: u32,

        /// Whole packet payload.
        #[serde_as(as = "serde_with::hex::Hex")]
        raw: Vec<u8>,
    },
}

impl Payload {
    pub fn new(type_: u32, payload: Vec<u8>) -> Result<Self> {
        let this = match type_ {
            0 => {
                let mut reader = payload.as_slice();
                reader.read_exact(&mut [0; 10])?;

                let author_nickname = read_string(&mut reader)?;
                let arena_unique_id = reader.read_u64::<LittleEndian>()?;
                let arena_type_id = reader.read_u32::<LittleEndian>()?;
                let summary = {
                    assert_magic(reader.read_u8()?, 0xFF)?;
                    let pickled_length = reader.read_u16::<LittleEndian>()?;
                    assert_magic(reader.read_u8()?, 0x00)?;
                    read_pickled(&mut reader, pickled_length as usize)?
                };
                Self::ReplayHeader(ReplayHeader {
                    summary,
                    author_nickname,
                    arena_unique_id,
                    arena_type_id,
                })
            }

            8 => Self::EntityMethod(EntityMethod::new(payload)?),

            _ => Self::Unknown {
                raw: payload,
                type_,
            },
        };
        Ok(this)
    }
}
