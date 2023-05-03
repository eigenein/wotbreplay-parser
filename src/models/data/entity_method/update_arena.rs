use prost::Message;
use serde::Serialize;

#[derive(Message, Serialize)]
pub struct UpdateArena {
    #[prost(message, tag = "1")]
    pub players: Option<Players>,
}

#[derive(Message, Serialize)]
pub struct Players {
    #[prost(message, repeated, tag = "1")]
    pub players: Vec<Player>,
}

#[derive(Message, Serialize)]
pub struct Player {
    #[prost(string, required, tag = "3")]
    pub nickname: String,

    #[prost(uint32, required, tag = "4")]
    pub team_number: u32,

    #[prost(uint32, required, tag = "7")]
    pub account_id: u32,

    #[prost(string, optional, tag = "8")]
    pub clan_tag: Option<String>,

    #[prost(uint32, optional, tag = "11")]
    pub platoon_number: Option<u32>,
}
