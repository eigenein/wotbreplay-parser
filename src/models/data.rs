//! `data.wotreplay` models.

use std::collections::HashMap;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::result::Result;
use crate::Error;

/// `data.wotreplay` root structure.
#[derive(Debug, Serialize)]
pub struct Data {
    pub client_version: String,
    pub author_nickname: String,
    pub arena_unique_id: u64,
    pub arena_type_id: u32,
    pub summary: Summary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "playersBattleCategoriesIds")]
    pub players_battle_categories_ids: HashMap<u32, (u8, u32)>,

    #[serde(rename = "battleLevel")]
    pub battle_level: u8,

    #[serde(rename = "battleCategoryId")]
    pub battle_category_id: u8,

    #[serde(rename = "mouseEnabled")]
    pub is_mouse_enabled: bool,

    #[serde(rename = "mmType")]
    pub matchmaker_type: u8,

    #[serde(rename = "camouflageSlot")]
    pub camouflage_slot: u8,

    #[serde(rename = "avgMmr")]
    pub average_mmr: Vec<f64>,
}

impl Data {
    pub fn from_reader(mut reader: impl Read) -> Result<Self> {
        Self::assert_magic(reader.read_u32::<LittleEndian>()?, 0x12345678)?;
        reader.read_u64::<LittleEndian>()?; // TODO
        read_bytes(&mut reader)?; // TODO
        let client_version = read_string(&mut reader)?;
        reader.read_exact(&mut [0; 49])?; // TODO
        let author_nickname = read_string(&mut reader)?;
        let arena_unique_id = reader.read_u64::<LittleEndian>()?;
        let arena_type_id = reader.read_u32::<LittleEndian>()?;
        let summary = {
            Self::assert_magic(reader.read_u8()?, 0xFF)?;
            let pickled_length = reader.read_u16::<LittleEndian>()?;
            Self::assert_magic(reader.read_u8()?, 0x00)?;
            read_pickled(&mut reader, pickled_length as usize)?
        };

        let this = Self {
            client_version,
            author_nickname,
            arena_unique_id,
            arena_type_id,
            summary,
        };
        Ok(this)
    }

    #[inline]
    fn assert_magic<T: Into<u32> + PartialEq>(actual: T, expected: T) -> Result {
        if actual == expected {
            Ok(())
        } else {
            Err(Error::InvalidMagic(actual.into(), expected.into()))
        }
    }
}

#[derive(Debug)]
struct Packet {
    clock: f32,
    payload: Payload,
}

impl Packet {
    fn from_reader(reader: &mut impl Read) -> Result<Self> {
        let length = reader.read_u32::<LittleEndian>()?;
        let type_ = reader.read_u32::<LittleEndian>()?;
        let clock = reader.read_f32::<LittleEndian>()?;
        let payload = Payload::from_reader(reader, type_, length)?;
        let this = Self { clock, payload };
        Ok(this)
    }
}

#[derive(Debug)]
pub enum Payload {
    EntityProperty,
    EntityMethod,
    Other,
}

impl Payload {
    fn from_reader(reader: &mut impl Read, type_: u32, length: u32) -> Result<Self> {
        let this = match type_ {
            _ => {
                let mut buffer = Vec::new();
                buffer.resize(length as usize, 0);
                reader.read_exact(&mut buffer)?;
                Self::Other
            }
        };
        Ok(this)
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
