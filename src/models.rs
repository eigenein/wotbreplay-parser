use ::prost::Message;

#[derive(Message)]
pub struct BattleResults {
    #[prost(int64, tag = "2")]
    pub timestamp: i64,

    #[prost(message, repeated, tag = "201")]
    pub players: Vec<Player>,
}

#[derive(Message)]
pub struct Player {
    #[prost(uint32, tag = "1")]
    pub account_id: u32,

    #[prost(message, required, tag = "2")]
    pub info: PlayerInfo,
}

#[derive(Message)]
pub struct PlayerInfo {
    #[prost(string, tag = "1")]
    pub nickname: String,

    #[prost(uint32, optional, tag = "2")]
    pub platoon_id: Option<u32>,

    #[prost(uint32, tag = "3")]
    pub team_number: u32,
}
