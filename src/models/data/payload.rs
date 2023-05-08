use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::serde_as;

use crate::models::data::base_player_create::BasePlayerCreate;
use crate::models::data::entity_method::EntityMethod;
use crate::models::data::{read_pickled, read_quirky_length, read_string};
use crate::result::Result;

/// Packet payload.
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
    Unknown { packet_type: u32 },
}

impl Payload {
    /// Parse the packet payload.
    pub fn new(packet_type: u32, payload: &[u8]) -> Result<Self> {
        let mut payload = payload;

        let this = match packet_type {
            0 => {
                payload.read_exact(&mut [0; 10])?;

                let author_nickname = read_string(&mut payload)?;
                let arena_unique_id = payload.read_u64::<LittleEndian>()?;
                let arena_type_id = payload.read_u32::<LittleEndian>()?;
                let arguments = {
                    let pickled_length = read_quirky_length(&mut payload)?;
                    read_pickled(&mut payload, pickled_length)?
                };
                Self::BasePlayerCreate {
                    arguments,
                    author_nickname,
                    arena_unique_id,
                    arena_type_id,
                }
            }

            8 => Self::EntityMethod(EntityMethod::new(payload)?),

            _ => Self::Unknown { packet_type },
        };
        Ok(this)
    }
}
