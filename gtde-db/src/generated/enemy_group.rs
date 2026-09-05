use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::generated::{
    EnumWrapper,
    enums::{
        EEnemyGroupType, EEnemyRole, EEnemyRoleDifficulty, EEnemyRoleDistribution,
        ESpawnPlacementType,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyGroupDataBlock {
    pub difficulty: EnumWrapper<EEnemyRoleDifficulty>,
    pub max_score: f64,
    pub relative_weight: f64,
    pub roles: Vec<EnemyGroupCompositionData>,
    pub score_in_area_padding_multi: f64,
    pub spawn_placement_type: EnumWrapper<ESpawnPlacementType>,
    #[serde(rename = "Type")]
    pub enemy_group_data_block_type: EnumWrapper<EEnemyGroupType>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyGroupCompositionData {
    pub distribution: EnumWrapper<EEnemyRoleDistribution>,
    pub role: EnumWrapper<EEnemyRole>,
}
