use serde::{Deserialize, Serialize};

use crate::{
    datablocks::reference::Reference,
    generated::{
        EnumWrapper,
        enemy::Enemy,
        enums::{EEnemyRole, EEnemyRoleDifficulty},
    },
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyPopulation {
    pub role_datas: Vec<EnemyRoleData>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyRoleData {
    pub cost: f64,
    pub difficulty: EnumWrapper<EEnemyRoleDifficulty>,
    pub enemy: Reference<Enemy>,
    pub role: EnumWrapper<EEnemyRole>,
    pub weight: f64,
}
