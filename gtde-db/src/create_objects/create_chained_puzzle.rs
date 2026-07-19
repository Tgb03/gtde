use gtde_error::error::Error;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    create_objects::{
        generic_constructor::GenericConstructor,
        targetted_constructor::{TargettedConstructor, TargettedConstructorWithoutName},
    },
    datablocks::{block_wrapper::BlockWrapper, reference::Reference},
    generated::{
        EnumWrapper,
        chained_puzzle::{ChainedPuzzle, ChainedPuzzleComponent},
        enums::{EEnemyName, EEnemyType},
        survival_wave_population::SurvivalWavePopulation,
        survival_wave_settings::{EEnemyFilterType, SurvivalWaveSettings, SurvivalWaveSpawnType},
    },
};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateChainedPuzzle {
    chained_puzzle_data: ChainedPuzzleIntermediary,
    survival_wave_population: [EnumWrapper<EEnemyName>; 5],
    survival_wave_settings: SurvivalWaveSettings,

    name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ChainedPuzzleIntermediary {
    pub alarm_sound_start: u32,
    pub alarm_sound_stop: u32,
    pub chained_puzzle: Vec<ChainedPuzzleComponent>,
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
            chained_puzzle: self.chained_puzzle,
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

impl Default for CreateChainedPuzzle {
    fn default() -> Self {
        Self {
            chained_puzzle_data: ChainedPuzzleIntermediary {
                public_alarm_name: "Class II Alarm".into(),
                alarm_sound_start: 3339129407,
                alarm_sound_stop: 42633153,
                chained_puzzle: vec![
                    ChainedPuzzleComponent { puzzle_type: 6 },
                    ChainedPuzzleComponent { puzzle_type: 2 },
                ],
                disable_survival_wave_on_complete: true,
                only_show_hud_when_player_is_close: false,
                survival_wave_area_distance: 2,
                trigger_alarm_on_activate: true,
                use_random_positions: true,
                wanted_distance_between_puzzle_components: 10f32,
                wanted_distance_from_start_pos: 0f32,
            },
            survival_wave_population: Default::default(),
            survival_wave_settings: SurvivalWaveSettings {
                m_chance_to_randomize_spawn_direction_per_group: 0.1f32,
                m_chance_to_randomize_spawn_direction_per_wave: 1.0f32,
                m_filter_type: EEnemyFilterType::Exclude.into(),
                m_override_wave_spawn_type: false,
                m_pause_before_start: 3f32,
                m_pause_between_groups: 5f32,
                m_population_filter: vec![EEnemyType::Weakling.into()],
                m_population_points_min_per_group: 5f32,
                m_population_points_per_group_start: 5f32,
                m_population_points_per_group_end: 10f32,
                m_population_points_per_wave_start: 17f32,
                m_population_points_per_wave_end: 25f32,
                m_population_points_total: -1f32,
                m_population_ramp_over_time: 200f32,
                m_survival_wave_spawn_type: SurvivalWaveSpawnType::InRelationToClosestAlivePlayer
                    .into(),
                m_wave_pause_min: 3f32,
                m_wave_pause_max: 30f32,
                m_wave_pause_max_at_cost: 3f32,
                m_wave_pause_min_at_cost: 10f32,
            },
            name: Default::default(),
        }
    }
}

fn convert_enemy(wrapper: &EnumWrapper<EEnemyName>) -> u32 {
    match wrapper {
        EnumWrapper::Data(d) => *d as u32,
        EnumWrapper::Int(i) => *i as u32,
    }
}

impl GenericConstructor for CreateChainedPuzzle {
    fn construct_all(self, env_path: impl AsRef<std::path::Path>) -> Result<(), Error> {
        let survival_population = SurvivalWavePopulation {
            wave_role_weakling: convert_enemy(&self.survival_wave_population[0]).into(),
            wave_role_standard: convert_enemy(&self.survival_wave_population[1]).into(),
            wave_role_special: convert_enemy(&self.survival_wave_population[2]).into(),
            wave_role_mini_boss: convert_enemy(&self.survival_wave_population[3]).into(),
            wave_role_boss: convert_enemy(&self.survival_wave_population[4]).into(),
        };

        let wave_population = survival_population.construct(
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
