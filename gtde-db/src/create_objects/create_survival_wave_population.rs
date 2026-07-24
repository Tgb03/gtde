use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{create_objects::targetted_constructor::TargettedConstructor, datablocks::{block_wrapper::BlockWrapper, reference::Reference}, generated::{EnumWrapper, enums::EEnemyName, survival_wave_population::SurvivalWavePopulation}};

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct CreateSurvivalWavePopulation {
    name: String,
    enemies: [EnumWrapper<EEnemyName>; 5],
}

impl TargettedConstructor for CreateSurvivalWavePopulation {
    type Data = SurvivalWavePopulation;

    fn construct(
        self,
        env_path: impl AsRef<std::path::Path>,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, gtde_error::error::Error> {
        let block = BlockWrapper::new(
            SurvivalWavePopulation {
                wave_role_weakling: self.enemies[0].clone().into(),
                wave_role_standard: self.enemies[1].clone().into(),
                wave_role_special: self.enemies[2].clone().into(),
                wave_role_mini_boss: self.enemies[3].clone().into(),
                wave_role_boss: self.enemies[4].clone().into(),
            }, 
            self.name, 
            0
        );
        
        block.construct(env_path, datablock_name)
    }
} 
