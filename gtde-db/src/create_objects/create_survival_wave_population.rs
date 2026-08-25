use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    datablocks::satisfied::Satisfied,
    generated::{EnumWrapper, enums::EEnemyName, survival_wave_population::SurvivalWavePopulation},
};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SurvivalWavePopulationIntermediary {
    pub weakling: Option<EnumWrapper<EEnemyName>>,
    pub standard: Option<EnumWrapper<EEnemyName>>,
    pub special: Option<EnumWrapper<EEnemyName>>,
    pub mini_boss: Option<EnumWrapper<EEnemyName>>,
    pub boss: Option<EnumWrapper<EEnemyName>>,
}

impl Satisfied for SurvivalWavePopulationIntermediary {
    type Target = SurvivalWavePopulation;

    fn satisfied_by(&self, data: &SurvivalWavePopulation) -> bool {
        self.weakling
            .clone()
            .is_none_or(|e| e.as_u32() == data.wave_role_weakling.as_u32())
            && self
                .standard
                .clone()
                .is_none_or(|e| e.as_u32() == data.wave_role_standard.as_u32())
            && self
                .special
                .clone()
                .is_none_or(|e| e.as_u32() == data.wave_role_special.as_u32())
            && self
                .mini_boss
                .clone()
                .is_none_or(|e| e.as_u32() == data.wave_role_mini_boss.as_u32())
            && self
                .boss
                .clone()
                .is_none_or(|e| e.as_u32() == data.wave_role_boss.as_u32())
    }
}

impl Into<SurvivalWavePopulation> for SurvivalWavePopulationIntermediary {
    fn into(self) -> SurvivalWavePopulation {
        SurvivalWavePopulation {
            wave_role_boss: self.boss.unwrap_or_default().as_u32().into(),
            wave_role_mini_boss: self.mini_boss.unwrap_or_default().as_u32().into(),
            wave_role_special: self.special.unwrap_or_default().as_u32().into(),
            wave_role_standard: self.standard.unwrap_or_default().as_u32().into(),
            wave_role_weakling: self.weakling.unwrap_or_default().as_u32().into(),
        }
    }
}
