use gtde_error::error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    create_objects::{
        create_survival_wave_population::SurvivalWavePopulationIntermediary,
        generic_constructor::GenericConstructor,
        targetted_constructor::{TargettedConstructor, TargettedConstructorWithoutName},
    },
    datablocks::{block_wrapper::BlockWrapper, reference::Reference},
    generated::{
        chained_puzzle::{ChainedPuzzle, ChainedPuzzleComponent},
        survival_wave_population::SurvivalWavePopulation,
        survival_wave_settings::SurvivalWaveSettings,
    },
};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateChainedPuzzle {
    chained_puzzle_data: ChainedPuzzleIntermediary,
    survival_wave_population: SurvivalWavePopulationIntermediary,
    survival_wave_settings: SurvivalWaveSettings,

    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
struct ChainedPuzzleIntermediary {
    pub alarm_sound_start: u32,
    pub alarm_sound_stop: u32,
    pub chained_puzzle: Vec<i32>,
    pub disable_survival_wave_on_complete: bool,
    #[serde(rename = "OnlyShowHUDWhenPlayerIsClose")]
    pub only_show_hud_when_player_is_close: bool,
    pub public_alarm_name: String,
    pub survival_wave_area_distance: i32,
    pub trigger_alarm_on_activate: bool,
    pub use_random_positions: bool,
    pub wanted_distance_between_puzzle_components: f32,
    pub wanted_distance_from_start_pos: f32,
}

impl ChainedPuzzleIntermediary {
    pub fn into_chained_puzzle(
        self,
        population: Reference<SurvivalWavePopulation>,
        settings: Reference<SurvivalWaveSettings>,
    ) -> ChainedPuzzle {
        ChainedPuzzle {
            alarm_sound_start: self.alarm_sound_start,
            alarm_sound_stop: self.alarm_sound_stop,
            chained_puzzle: self
                .chained_puzzle
                .iter()
                .map(|e| ChainedPuzzleComponent { puzzle_type: *e })
                .collect(),
            disable_survival_wave_on_complete: self.disable_survival_wave_on_complete,
            only_show_hud_when_player_is_close: self.only_show_hud_when_player_is_close,
            public_alarm_name: self.public_alarm_name,
            survival_wave_area_distance: self.survival_wave_area_distance,
            survival_wave_population: population,
            survival_wave_settings: settings,
            trigger_alarm_on_activate: self.trigger_alarm_on_activate,
            use_random_positions: self.use_random_positions,
            wanted_distance_between_puzzle_components: self
                .wanted_distance_between_puzzle_components,
            wanted_distance_from_start_pos: self.wanted_distance_from_start_pos,
        }
    }
}

impl GenericConstructor for CreateChainedPuzzle {
    fn construct_all(self, env_path: impl AsRef<std::path::Path>) -> Result<(), Error> {
        let wave_population = self.survival_wave_population.construct(
            &env_path,
            self.name.clone(),
            "SurvivalWavePopulation",
        )?;
        let wave_settings = self.survival_wave_settings.clone().construct(
            &env_path,
            self.name.clone(),
            "SurvivalWaveSettings",
        )?;
        let block_chained_puzzle = BlockWrapper::new(
            self.chained_puzzle_data
                .into_chained_puzzle(wave_population, wave_settings),
            self.name,
            0,
        );
        let _ = block_chained_puzzle.construct(&env_path, "ChainedPuzzle")?;

        Ok(())
    }
}
