use gtde_file::named_data::NamedData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::generated::{EnumWrapper, enums::EEnemyType};

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub struct SurvivalWaveSettings {
    #[serde(rename = "m_chanceToRandomizeSpawnDirectionPerGroup")]
    pub m_chance_to_randomize_spawn_direction_per_group: f32,

    #[serde(rename = "m_chanceToRandomizeSpawnDirectionPerWave")]
    pub m_chance_to_randomize_spawn_direction_per_wave: f32,

    #[serde(rename = "m_filterType")]
    pub m_filter_type: EnumWrapper<EEnemyFilterType>,

    #[serde(rename = "m_overrideWaveSpawnType")]
    pub m_override_wave_spawn_type: bool,

    #[serde(rename = "m_pauseBeforeStart")]
    pub m_pause_before_start: f32,

    #[serde(rename = "m_pauseBetweenGroups")]
    pub m_pause_between_groups: f32,

    #[serde(rename = "m_populationFilter")]
    pub m_population_filter: Vec<EnumWrapper<EEnemyType>>,

    #[serde(rename = "m_populationPointsMinPerGroup")]
    pub m_population_points_min_per_group: f32,

    #[serde(rename = "m_populationPointsPerGroupEnd")]
    pub m_population_points_per_group_end: f32,

    #[serde(rename = "m_populationPointsPerGroupStart")]
    pub m_population_points_per_group_start: f32,

    #[serde(rename = "m_populationPointsPerWaveEnd")]
    pub m_population_points_per_wave_end: f32,

    #[serde(rename = "m_populationPointsPerWaveStart")]
    pub m_population_points_per_wave_start: f32,

    #[serde(rename = "m_populationPointsTotal")]
    pub m_population_points_total: f32,

    #[serde(rename = "m_populationRampOverTime")]
    pub m_population_ramp_over_time: f32,

    #[serde(rename = "m_survivalWaveSpawnType")]
    pub m_survival_wave_spawn_type: EnumWrapper<SurvivalWaveSpawnType>,

    #[serde(rename = "m_wavePauseMax")]
    pub m_wave_pause_max: f32,

    #[serde(rename = "m_wavePauseMax_atCost")]
    pub m_wave_pause_max_at_cost: f32,

    #[serde(rename = "m_wavePauseMin")]
    pub m_wave_pause_min: f32,

    #[serde(rename = "m_wavePauseMin_atCost")]
    pub m_wave_pause_min_at_cost: f32,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub enum EEnemyFilterType {
    #[default]
    Exclude,
    Include,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, JsonSchema)]
pub enum SurvivalWaveSpawnType {
    #[default]
    #[serde(rename = "ClosestToSuppliedNodeButNoBetweenPlayers")]
    ClosestToSuppliedNodeButNoBetweenPlayers,
    #[serde(rename = "FromElevatorDirection")]
    FromElevatorDirection,
    #[serde(rename = "InRelationToClosestAlivePlayer")]
    InRelationToClosestAlivePlayer,
    #[serde(rename = "InSuppliedCourseNode")]
    InSuppliedCourseNode,
    #[serde(rename = "InSuppliedCourseNode_OnPosition")]
    InSuppliedCourseNodeOnPosition,
    #[serde(rename = "InSuppliedCourseNodeZone")]
    InSuppliedCourseNodeZone,
    #[serde(rename = "OnSpawnPoints")]
    OnSpawnPoints,
}

impl NamedData for SurvivalWaveSettings {
    fn get_name() -> &'static str {
        "SurvivalWaveSettings"
    }
}
