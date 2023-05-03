use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::serde_as;

use crate::models::data::base_player_create::BasePlayerCreate;
use crate::models::data::entity_method::EntityMethod;
use crate::models::data::{read_pickled, read_quirky_length, read_string};
use crate::result::Result;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Payload {
    /// Type 0.
    BasePlayerCreate {
        author_nickname: String,
        arena_unique_id: u64,
        arena_type_id: u32,
        arguments: Box<BasePlayerCreate>,
    },

    /// Type 8.
    EntityMethod(EntityMethod),

    /// Default payload when type is not known by the parser.
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
                let type_0 = {
                    let pickled_length = read_quirky_length(&mut reader)?;
                    read_pickled(&mut reader, pickled_length)?
                };
                Self::BasePlayerCreate {
                    arguments: type_0,
                    author_nickname,
                    arena_unique_id,
                    arena_type_id,
                }
            }

            8 => Self::EntityMethod(EntityMethod::new(payload)?),

            _ => Self::Unknown { raw: payload, type_ },
        };
        Ok(this)
    }
}
