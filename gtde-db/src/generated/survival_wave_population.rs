use serde::{Deserialize, Serialize};

use crate::{datablocks::reference::Reference, generated::enemy::Enemy};

#[derive(Clone, PartialEq, Serialize, Default, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SurvivalWavePopulation {
    pub wave_role_boss: Reference<Enemy>,
    pub wave_role_mini_boss: Reference<Enemy>,
    pub wave_role_special: Reference<Enemy>,
    pub wave_role_standard: Reference<Enemy>,
    pub wave_role_weakling: Reference<Enemy>,
}
