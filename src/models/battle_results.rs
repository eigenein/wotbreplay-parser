use prost::bytes::Buf;
use prost::{Enumeration, Message};
use serde::Serialize;

use crate::models::room_type::RoomType;
use crate::result::Result;

impl BattleResults {
    /// Parse battle results from the buffer.
    ///
    /// # Note
    ///
    /// This can **not** parse `battle_results.dat` itself, but the un-pickled tuple element.
    /// To parse `battle_results.dat`, use [`crate::models::battle_results_dat::BattleResultsDat`].
    pub fn from_buffer(buffer: impl Buf) -> Result<Self> {
        Ok(Self::decode(buffer)?)
    }
}

/// Parsed battle results.
///
/// TODO: 150[0].21[0].8[0].801[0].unsigned is arena end time.
#[derive(Message, Serialize)]
pub struct BattleResults {
    /// The two least-significant bytes is map ID.
    ///
    /// The 3rd byte is probably a game mode:
    ///
    /// - `0x0xxxx` is «encounter»
    /// - `0x1xxxx` flag is «supremacy»
    #[prost(uint32, tag = "1")]
    pub mode_map_id: u32,

    /// Battle timestamp.
    #[prost(int64, tag = "2")]
    pub timestamp_secs: i64,

    #[prost(enumeration = "TeamNumber", optional, tag = "3")]
    pub winner_team_number: Option<i32>,

    /// Replay's author results.
    #[prost(message, required, tag = "8")]
    pub author: Author,

    #[prost(enumeration = "RoomType", tag = "9")]
    pub room_type: i32,

    /// Author's free XP, including premium.
    #[prost(uint32, tag = "137")]
    pub free_xp: u32,

    /// Players in the battle.
    #[prost(message, repeated, tag = "201")]
    pub players: Vec<Player>,

    /// Individual player's results.
    #[prost(message, repeated, tag = "301")]
    pub player_results: Vec<PlayerResults>,
}

/// Battle player.
#[derive(Message, Serialize)]
pub struct Player {
    /// Player's account ID.
    #[prost(uint32, tag = "1")]
    pub account_id: u32,

    /// Player's extended information.
    #[prost(message, required, tag = "2")]
    pub info: PlayerInfo,
}

/// Player's extended information.
#[derive(Message, Serialize)]
pub struct PlayerInfo {
    /// Player's nickname.
    #[prost(string, tag = "1")]
    pub nickname: String,

    /// Some sort of platoon ID:
    ///
    /// - contains same ID for the platoon members, or
    /// - [`None`] for non-platoon players
    #[prost(uint32, optional, tag = "2")]
    pub platoon_id: Option<u32>,

    /// Player's team assignment.
    #[prost(enumeration = "TeamNumber", tag = "3")]
    pub team: i32,

    #[prost(string, optional, tag = "5")]
    pub clan_tag: Option<String>,

    #[prost(message, required, tag = "7")]
    pub avatar: Avatar,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration, Serialize)]
pub enum TeamNumber {
    One = 1,
    Two = 2,
}

/// Player's battle results.
#[derive(Message, Serialize)]
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
#[derive(Message, Serialize)]
pub struct PlayerResultsInfo {
    /// Credits earned – without special awards and medals and premium account excluded.
    #[prost(uint32, tag = "2")]
    pub credits_earned: u32,

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

    /// TODO: distinguish the kinds of assisted damage.
    #[prost(uint32, tag = "9")]
    pub damage_assisted_1: u32,

    /// TODO: distinguish the kinds of assisted damage.
    #[prost(uint32, tag = "10")]
    pub damage_assisted_2: u32,

    #[prost(uint32, tag = "12")]
    pub n_hits_received: u32,

    #[prost(uint32, tag = "13")]
    pub n_non_penetrating_hits_received: u32,

    #[prost(uint32, tag = "15")]
    pub n_penetrations_received: u32,

    #[prost(uint32, tag = "17")]
    pub n_enemies_damaged: u32,

    #[prost(uint32, tag = "18")]
    pub n_enemies_destroyed: u32,

    #[prost(uint32, tag = "32")]
    pub victory_points_earned: u32,

    #[prost(uint32, tag = "33")]
    pub victory_points_seized: u32,

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
    ///
    /// The display rating is calculated as: `3000.0 + mm_rating * 10.0`.
    #[prost(float, optional, tag = "107")]
    pub mm_rating: Option<f32>,

    #[prost(uint32, tag = "117")]
    pub damage_blocked: u32,
}

impl PlayerResultsInfo {
    /// Calculate displayed rating according to the game client.
    pub fn display_rating(&self) -> Option<u32> {
        self.mm_rating
            .map(|mm_rating| (mm_rating * 10.0 + 3000.0) as u32)
    }
}

#[derive(Message, Serialize)]
pub struct Author {
    /// Earned credits, including premium and special awards and medals.
    #[prost(uint32, tag = "2")]
    pub credits_earned: u32,

    /// Combat XP, including premium.
    #[prost(uint32, tag = "3")]
    pub combat_xp: u32,

    #[prost(uint32, tag = "8")]
    pub damage_dealt: u32,

    #[prost(uint32, tag = "101")]
    pub account_id: u32,

    #[prost(enumeration = "TeamNumber", tag = "102")]
    pub team_number: i32,
}

#[derive(Message, Serialize)]
pub struct Avatar {
    #[prost(message, required, tag = "2")]
    pub info: AvatarInfo,
}

#[derive(Message, Serialize)]
pub struct AvatarInfo {
    #[prost(string, tag = "2")]
    pub gfx_url: String,

    #[prost(string, tag = "3")]
    pub gfx2_url: String,

    #[prost(string, tag = "4")]
    pub kind: String,
}
