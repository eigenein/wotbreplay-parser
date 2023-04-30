use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Looks like some sort of replay summary.
#[derive(Debug, Serialize, Deserialize)]
pub struct Type0 {
    #[serde(rename(deserialize = "playersBattleCategoriesIds"))]
    pub players_battle_categories_ids: HashMap<u32, (u8, u32)>,

    #[serde(rename(deserialize = "battleLevel"))]
    pub battle_level: u8,

    #[serde(rename(deserialize = "battleCategoryId"))]
    pub battle_category_id: u8,

    #[serde(rename(deserialize = "mouseEnabled"))]
    pub is_mouse_enabled: bool,

    #[serde(rename(deserialize = "mmType"))]
    pub matchmaker_type: u8,

    #[serde(rename(deserialize = "camouflageSlot"))]
    pub camouflage_slot: u8,

    #[serde(rename(deserialize = "avgMmr"))]
    pub average_mmr: Vec<Option<f64>>,

    #[serde(rename(deserialize = "playerWaitTimes"))]
    pub player_wait_times: HashMap<u32, f64>,

    #[serde(rename(deserialize = "accountDatabaseIds"))]
    pub account_database_ids: Vec<u32>,

    #[serde(rename(deserialize = "turboBattlesStats"))]
    pub turbo_battles_statistics: TurboBattlesStatistics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TurboBattlesStatistics {
    #[serde(rename(deserialize = "turboBattleChance"))]
    pub battle_chance: f64,

    #[serde(rename(deserialize = "turboLeaveChance"))]
    pub leave_chance: f64,

    #[serde(rename(deserialize = "possibleTurboBattlesNotForSkip"))]
    pub possible_battles_not_for_skip: u32,

    #[serde(rename(deserialize = "possibleTurboBattlesForSkip"))]
    pub possible_battles_for_skip: u32,

    #[serde(rename(deserialize = "turboBattlesForSkip"))]
    pub battles_for_skip: u32,

    #[serde(rename(deserialize = "turboBattlesNotForSkip"))]
    pub battles_not_for_skip: u32,

    #[serde(rename(deserialize = "abSkip"))]
    pub ab_skip: i32,

    #[serde(rename(deserialize = "beta"))]
    pub beta: f64,

    #[serde(rename(deserialize = "premiumVehDiff"))]
    pub premium_vehicle_difference: i32,

    #[serde(rename(deserialize = "htDiff"))]
    pub ht_difference: i32,

    #[serde(rename(deserialize = "alpha"))]
    pub alpha: f64,
}
