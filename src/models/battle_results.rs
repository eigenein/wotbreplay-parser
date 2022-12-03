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

/// Parsed battle results.
#[derive(Message)]
pub struct BattleResults {
    /// Battle timestamp.
    #[prost(int64, tag = "2")]
    pub timestamp: i64,

    /// Players in the battle.
    #[prost(message, repeated, tag = "201")]
    pub players: Vec<Player>,

    /// Player's results.
    #[prost(message, repeated, tag = "301")]
    pub player_results: Vec<PlayerResults>,
}

/// Battle player.
#[derive(Message)]
pub struct Player {
    /// Player's account ID.
    #[prost(uint32, tag = "1")]
    pub account_id: u32,

    /// Player's extended information.
    #[prost(message, required, tag = "2")]
    pub info: PlayerInfo,
}

/// Player's extended information.
#[derive(Message)]
pub struct PlayerInfo {
    /// Player's nickname.
    #[prost(string, tag = "1")]
    pub nickname: String,

    /// Some sort of platoon ID:
    /// - contains same ID for a platoon members, or
    /// - `None` for non-platoon players
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

/// Player's battle results.
#[derive(Message)]
pub struct PlayerResults {
    /// Looks like some sort of sequential result ID. They reference each other,
    /// but I haven't figured this out yet.
    #[prost(uint32, tag = "1")]
    pub result_id: u32,

    /// Extended player's results information.
    #[prost(message, required, tag = "2")]
    pub info: PlayerResultsInfo,
}

/// Extended player's results information.
#[derive(Message)]
pub struct PlayerResultsInfo {
    /// Base XP (the total without multipliers).
    #[prost(uint32, tag = "3")]
    pub base_xp: u32,

    #[prost(uint32, tag = "4")]
    pub n_shots: u32,

    #[prost(uint32, tag = "5")]
    pub n_hits_dealt: u32,

    #[prost(uint32, tag = "7")]
    pub n_penetrations_dealt: u32,

    #[prost(uint32, tag = "8")]
    pub damage_dealt: u32,

    #[prost(uint32, tag = "9")]
    pub damage_assisted_1: u32,

    #[prost(uint32, tag = "10")]
    pub damage_assisted_2: u32,

    #[prost(uint32, tag = "12")]
    pub n_hits_received: u32,

    #[prost(uint32, tag = "13")]
    pub n_non_penetrating_hits_received: u32,

    #[prost(uint32, tag = "15")]
    pub n_penetrations_received: u32,

    /// TODO: distinguish earned vs seized.
    #[prost(uint32, tag = "32")]
    pub victory_points_1: u32,

    /// TODO: distinguish earned vs seized.
    #[prost(uint32, tag = "33")]
    pub victory_points_2: u32,

    /// Player's account ID.
    #[prost(uint32, tag = "101")]
    pub account_id: u32,

    /// Player's tank ID as per Wargaming.net API.
    #[prost(uint32, tag = "103")]
    pub tank_id: u32,

    /// Rating for the Rating Battles.
    ///
    /// Note, that this is **not** the game client's displayed rating.
    /// This field matches the `mm_rating` as returned by the Wargaming.net API.
    #[prost(float, optional, tag = "107")]
    pub mm_rating: Option<f32>,

    #[prost(uint32, tag = "117")]
    pub damage_blocked: u32,
}

impl PlayerResultsInfo {
    /// Returns displayed rating as per the game client.
    pub fn display_rating(&self) -> Option<u32> {
        self.mm_rating
            .map(|mm_rating| (mm_rating * 10.0 + 3000.0) as u32)
    }
}
