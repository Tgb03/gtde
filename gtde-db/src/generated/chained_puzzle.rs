use crate::{
    datablocks::reference::Reference,
    generated::{
        survival_wave_population::SurvivalWavePopulation,
        survival_wave_settings::SurvivalWaveSettings,
    },
};
use gtde_file::named_data::NamedData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainedPuzzle {
    pub alarm_sound_start: u32,
    pub alarm_sound_stop: u32,
    pub chained_puzzle: Vec<ChainedPuzzleComponent>,
    pub disable_survival_wave_on_complete: bool,
    #[serde(rename = "OnlyShowHUDWhenPlayerIsClose")]
    pub only_show_hud_when_player_is_close: bool,
    pub public_alarm_name: String,
    pub survival_wave_area_distance: i32,
    pub survival_wave_population: Reference<SurvivalWavePopulation>,
    pub survival_wave_settings: Reference<SurvivalWaveSettings>,
    pub trigger_alarm_on_activate: bool,
    pub use_random_positions: bool,
    pub wanted_distance_between_puzzle_components: f32,
    pub wanted_distance_from_start_pos: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "PascalCase")]
pub struct ChainedPuzzleComponent {
    pub puzzle_type: i32,
}

impl NamedData for ChainedPuzzle {
    fn get_name() -> &'static str {
        "ChainedPuzzle"
    }
}
