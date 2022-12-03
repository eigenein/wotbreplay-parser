use prost::bytes::Buf;
use prost::Message;

use crate::error::Error;
use crate::result::Result;

impl BattleResults {
    /// Parse battle results from the buffer.
    ///
    /// Note, that this does NOT parse `battle_results.dat` itself, but the un-pickled tuple element.
    pub fn parse(buffer: impl Buf) -> Result<Self> {
        Self::decode(buffer).map_err(Error::DecodeFailed)
    }
}

#[derive(Message)]
pub struct BattleResults {
    /// Battle timestamp.
    #[prost(int64, tag = "2")]
    pub timestamp: i64,

    /// Players in the battle.
    #[prost(message, repeated, tag = "201")]
    pub players: Vec<Player>,
}

#[derive(Message)]
pub struct Player {
    /// Player's account ID.
    #[prost(uint32, tag = "1")]
    pub account_id: u32,

    /// Player's extended information.
    #[prost(message, required, tag = "2")]
    pub info: PlayerInfo,
}

#[derive(Message)]
pub struct PlayerInfo {
    /// Player's nickname.
    #[prost(string, tag = "1")]
    pub nickname: String,

    #[prost(uint32, optional, tag = "2")]
    pub platoon_id: Option<u32>,

    /// Player's team assignment.
    #[prost(enumeration = "TeamNumber", tag = "3")]
    pub team_number: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, prost::Enumeration)]
#[repr(i32)]
pub enum TeamNumber {
    One = 1,
    Two = 2,
}
