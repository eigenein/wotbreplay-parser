use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use serde_with::serde_as;

use crate::result::Result;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum EntityMethod {
    /// TODO: team composition.
    Subtype2F,

    /// Default variant when subtype is not known.
    Unknown {
        sub_type: u32,

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
            0x2F => Self::Subtype2F,
            _ => Self::Unknown { sub_type, payload },
        };
        Ok(this)
    }
}
