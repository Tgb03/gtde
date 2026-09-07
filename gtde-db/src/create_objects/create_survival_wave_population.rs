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
            .is_none_or(|e| e.as_u32() as i64 == data.wave_role_weakling.as_i64())
            && self
                .standard
                .clone()
                .is_none_or(|e| e.as_u32() as i64 == data.wave_role_standard.as_i64())
            && self
                .special
                .clone()
                .is_none_or(|e| e.as_u32() as i64 == data.wave_role_special.as_i64())
            && self
                .mini_boss
                .clone()
                .is_none_or(|e| e.as_u32() as i64 == data.wave_role_mini_boss.as_i64())
            && self
                .boss
                .clone()
                .is_none_or(|e| e.as_u32() as i64 == data.wave_role_boss.as_i64())
    }
}

impl Into<SurvivalWavePopulation> for SurvivalWavePopulationIntermediary {
    fn into(self) -> SurvivalWavePopulation {
        SurvivalWavePopulation {
            wave_role_boss: self.boss.map(|e| e.as_u32() as i64).unwrap_or(-1).into(),
            wave_role_mini_boss: self
                .mini_boss
                .map(|e| e.as_u32() as i64)
                .unwrap_or(-1)
                .into(),
            wave_role_special: self.special.map(|e| e.as_u32() as i64).unwrap_or(-1).into(),
            wave_role_standard: self
                .standard
                .map(|e| e.as_u32() as i64)
                .unwrap_or(-1)
                .into(),
            wave_role_weakling: self
                .weakling
                .map(|e| e.as_u32() as i64)
                .unwrap_or(-1)
                .into(),
        }
    }
}
