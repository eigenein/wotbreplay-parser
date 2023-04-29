use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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
