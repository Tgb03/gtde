use serde::{Deserialize, Serialize};

use crate::{datablocks::reference::Reference, generated::{EnumWrapper, UndefinedReferenceItem, chained_puzzle::ChainedPuzzle, rundown::{LocalZoneIndex, ZonePlacementWeights}, vector::Vector2, warden_objective_event::WardenObjectiveEvent}};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelLayoutDataBlock {
    pub zone_alias_start: i32,

    pub zones: Vec<ExpeditionZoneData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionZoneData {
    pub active_enemy_wave: Option<ActiveEnemyWaveData>,
    pub alias_override: Option<i64>,
    pub alias_prefix_override: Option<String>,
    pub alias_prefix_short_override: Option<String>,
    pub allow_resource_container_allocation: Option<bool>,
    pub allow_small_pickups_allocation: Option<bool>,
    pub altitude_data: Option<AltitudeData>,
    pub big_pickup_distribution_in_zone: Reference<UndefinedReferenceItem>,
    pub build_from_local_index: EnumWrapper<LocalZoneIndex>,
    #[serde(rename = "BulkheadDCScanSeed")]
    pub bulkhead_dc_scan_seed: Option<i64>,
    pub chained_puzzle_to_enter: Reference<ChainedPuzzle>,
    pub consumable_distribution_in_zone: Reference<UndefinedReferenceItem>,
    pub corpse_clusters_in_zone: Reference<UndefinedReferenceItem>,
    pub corpses_in_zone: Option<StaticGroundSpawnersInZone>,
    pub coverage_min_max: Vector2<f32>,
    #[serde(default)] pub custom_geomorph: String,
    pub decon_units_in_zone: Option<StaticGroundSpawnersInZone>,
    pub disinfection_station_placements: Option<Vec<FunctionPlacementData>>,
    pub enemy_respawn_count_multiplier: f32,
    pub enemy_respawn_exclude_list: Option<Vec<i64>>,
    #[serde(default)] pub enemy_respawning: bool,
    pub enemy_respawn_require_other_zone: bool,
    pub enemy_respawn_room_distance: i32,
    pub enemy_respawn_time_interval: Option<f64>,
    pub enemy_spawning_in_zone: Option<Vec<EnemySpawningInZoneElement>>,
    #[serde(default)] pub events_on_approach_door: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_boss_death: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_door_scan_done: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_door_scan_start: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_open_door: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_portal_warp: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_terminal_deactivate_alarm: Vec<WardenObjectiveEvent>,
    #[serde(default)] pub events_on_unlock_door: Vec<WardenObjectiveEvent>,
    pub events_on_enter: Option<Vec<LevelEventData>>,
    pub events_on_trigger: Option<Vec<WorldEventFromSourceData>>,
    #[serde(default)] pub forbid_terminals_in_zone: bool,
    pub force_big_pickups_allocation: bool,
    pub generator_clusters_in_zone: Option<i64>,
    pub ground_spawners_in_zone: Option<StaticGroundSpawnersInZone>,
    #[serde(rename = "HSUClustersInZone")]
    pub hsu_clusters_in_zone: Option<i64>,
    #[serde(rename = "HSUsInZone")]
    pub hs_us_in_zone: Option<StaticGroundSpawnersInZone>,
    pub ignore_random_geomorph_rotation: bool,
    pub is_checkpoint_door: bool,
    pub light_settings: Reference<UndefinedReferenceItem>,
    pub lights_sub_seed: i32,
    pub local_index: EnumWrapper<LocalZoneIndex>,
    pub marker_sub_seed: i32,
    #[serde(default)] pub override_alias_prefix: bool,
    pub play_scanner_voice_audio: bool,
    pub power_generator_placements: Option<Vec<FunctionPlacementData>>,
    pub progression_puzzle_to_enter: Option<ProgressionPuzzleData>,
    pub resource_container_clusters_in_zone: Option<i64>,
    pub security_gate_to_enter: Option<SecurityGateToEnter>,
    pub skip_automatic_progression_objective: Option<bool>,
    pub specific_pickup_spawning_datas: Option<Vec<SpecificPickupSpawnData>>,
    pub start_expansion: Option<StartExpansion>,
    pub start_position: Option<StartPosition>,
    #[serde(rename = "StartPosition_IndexWeight")]
    pub start_position_index_weight: Option<f64>,
    pub static_spawn_data_containers: Option<Vec<StaticSpawnDataContainer>>,
    pub sub_complex: Option<PrimareSubComplexUsedUnion>,
    pub sub_seed: Option<i64>,
    pub terminal_placements: Option<Vec<TerminalPlacementElement>>,
    pub terminal_puzzle_zone: Option<TerminalZoneSelectionData>,
    #[serde(default)] pub turn_off_alarm_on_terminal: bool,
    pub use_static_bioscan_points_in_zone: bool,
    pub health_multi: f32,
    pub health_placement: ZonePlacementWeights,
    pub disinfection_multi: f32,
    pub disinfection_placement: ZonePlacementWeights,
    pub weapon_ammo_multi: f32,
    pub weapon_ammo_placement: ZonePlacementWeights,
    pub tool_ammo_multi: f32,
    pub tool_ammo_placement: ZonePlacementWeights,
    pub world_event_chained_puzzle_datas: Option<Vec<SpecificChainPuzzleSpawnData>>,
    pub zone_expansion: Option<ZoneExpansion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AltitudeData {
    pub allowed_zone_altitude: EnumWrapper<WantedZoneHeighs>,
    #[serde(default)] pub chance_to_change: f32,
}

#[derive(Serialize, Deserialize)]
pub enum WantedZoneHeighs
{
    LowMidHigh,
    OnlyLow,
    OnlyHigh,
    OnlyMid,
    LowMid,
    MidHigh,
    LowHigh,
    Ascending,
    Descending,
    Unchanged
}
