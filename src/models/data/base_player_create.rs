use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasePlayerCreate {
    #[serde(default, rename(deserialize = "playersBattleCategoriesIds"))]
    pub players_battle_categories_ids: Option<HashMap<u64, (u8, u32)>>,

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

    /// Average team ratings for rating battles.
    ///
    /// # Contains
    ///
    /// - [`Some`] 2-vector, if the attribute is present in the replay. Each element represents one team:
    ///     - [`Some`] rating, if known
    ///     - [`None`], if the average rating is unknown (all the players are calibrating)
    /// - [`None`], if the attribute is missing in the replay
    #[serde(default, rename(deserialize = "avgMmr"))]
    pub average_mm_ratings: Option<Vec<Option<f64>>>,

    #[serde(default, rename(deserialize = "playerWaitTimes"))]
    pub player_wait_times: Option<HashMap<u64, f64>>,

    #[serde(rename(deserialize = "accountDatabaseIds"))]
    pub account_database_ids: Option<Vec<u64>>,

    #[serde(default, rename(deserialize = "turboBattlesStats"))]
    pub turbo_battles_statistics: Option<TurboBattlesStatistics>,
}

/// I don't know what that is:
///
/// - On EU, Asia, and NA the values are just constants
/// - On Russia, the `alpha` and `beta` are different constants, and the chances actually differ for different replays
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
