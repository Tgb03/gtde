use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Enemy {
    // #[serde(rename = "AI_Abilities")]
    // pub ai_abilities: Vec<AbilityData>,
    // pub arena_dimensions: Vec<i64>,
    // pub asset_bundle: BundleName,
    // pub balancing_data_id: i64,
    // pub base_prefabs: Vec<String>,
    // pub behavior_data_id: i64,
    // pub bundle_shard: Shard,
    // pub detection_data_id: i64,
    // pub enemy_spotted_dialog_id: i64,
    // pub enemy_type: EnemyTypeUnion,
    // pub internal_material: InternalMaterial,
    // #[serde(rename = "isCoccoon")]
    // pub is_coccoon: bool,
    // pub linked_slave_models: Vec<LinkedSlaveModelData>,
    // pub model_datas: Vec<ModelData>,
    // pub movement_data_id: i64,
    // #[serde(rename = "SFXDataId")]
    // pub sfx_data_id: i64,
}
