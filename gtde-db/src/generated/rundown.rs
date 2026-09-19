use serde::{Deserialize, Serialize};

use crate::{
    datablocks::reference::Reference, generated::{EnumWrapper, UndefinedReferenceItem, enemy_population::EnemyPopulation, survival_wave_population::SurvivalWavePopulation, survival_wave_settings::SurvivalWaveSettings, text::LocalizedText, vector::Vector2},
};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rundown {
    pub req_to_reach_tier_b: RundownTierProgressionData,
    pub req_to_reach_tier_c: RundownTierProgressionData,
    pub req_to_reach_tier_d: RundownTierProgressionData,
    pub req_to_reach_tier_e: RundownTierProgressionData,
    pub storytelling_data: RundownStorytellingData,
    pub tier_a: Vec<ExpeditionInTierData>,
    pub tier_b: Vec<ExpeditionInTierData>,
    pub tier_c: Vec<ExpeditionInTierData>,
    pub tier_d: Vec<ExpeditionInTierData>,
    pub tier_e: Vec<ExpeditionInTierData>,
    pub use_tier_unlock_requirements: bool,
    pub vanity_item_layer_drop_data_block: Reference<UndefinedReferenceItem>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownTierProgressionData {
    pub all_cleared_sectors: i64,
    pub main_sectors: i64,
    pub secondary_sectors: i64,
    pub third_sectors: i64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownStorytellingData {
    pub external_exp_title: LocalizedText,
    pub surface_description: LocalizedText,
    pub surface_icon_position: Vector2<f64>,
    pub text_log: LocalizedText,
    pub text_log_pos: Vector2<f64>,
    pub title: LocalizedText,
    pub visuals: RundownStorytellingVisualData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownStorytellingVisualData {
    pub color_background: Color,
    pub tier_a_visuals: TierVisualData,
    pub tier_b_visuals: TierVisualData,
    pub tier_c_visuals: TierVisualData,
    pub tier_d_visuals: TierVisualData,
    pub tier_e_visuals: TierVisualData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TierVisualData {
    pub color: Color,
    pub scale: f64,
    pub scale_y_modifier: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionInTierData {
    pub accessibility: EnumWrapper<ExpeditionAccessibility>,
    pub build_secondary_from: BuildLayerFromData,
    pub build_third_from: BuildLayerFromData,
    pub custom_progression_lock: RundownTierProgressionData,
    pub descriptive: DescriptiveData,
    pub dimension_datas: Vec<DimensionInExpeditionData>,
    pub disable_player_voicelines: bool,
    pub enabled: bool,
    pub exclude_from_progression: bool,
    pub expedition: ExpeditionData,
    pub gear_picker: GearPicker,
    pub has_external_style: bool,
    pub has_story_style: bool,
    pub is_single_player: bool,
    pub level_layout_data: Reference<UndefinedReferenceItem>,
    pub main_layer_data: LayerData,
    pub put_icon_above_tier: bool,
    pub secondary_layer_data: LayerData,
    pub secondary_layer_enabled: bool,
    pub secondary_layout: Reference<UndefinedReferenceItem>,
    pub seeds: BuildSeedData,
    pub skip_lobby: bool,
    pub sound_event_on_warp_to_reality: i64,
    pub special_override_data: SpecialExpeditionOverridesData,
    pub third_layer_data: LayerData,
    pub third_layer_enabled: bool,
    pub third_layout: Reference<UndefinedReferenceItem>,
    pub unlocked_by_expedition: ExpeditionIndex,
    pub use_gear_picker: bool,
    pub vanity_items_drop_data: VanityItemsDropData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialExpeditionOverridesData {
    pub health_level_at_expedition_start: f64,
    pub infection_level_at_expedition_start: f64,
    pub special_ammo_at_expedition_start: f64,
    pub standard_ammo_at_expedition_start: f64,
    pub tool_ammo_at_expedition_start: f64,
    pub weak_resource_container_with_pack_chance_for_locked: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VanityItemsDropData {
    pub groups: Vec<Reference<UndefinedReferenceItem>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildSeedData {
    pub build_seed: i32,
    pub function_marker_offset: i32,
    pub light_job_seed_offset: i32,
    pub standard_marker_offset: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPicker {
    pub melee_gear: Reference<UndefinedReferenceItem>,
    pub special_gear: Reference<UndefinedReferenceItem>,
    pub standard_gear: Reference<UndefinedReferenceItem>,
    pub tool_gear: Reference<UndefinedReferenceItem>,
}

#[derive(Serialize, Deserialize)]
pub enum ExpeditionAccessibility {
    AlwayBlock,
    AlwaysAllow,
    BlockedAndScrambled,
    Normal,
    UnlockedByExpedition,
    UseCustomProgressionLock,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildLayerFromData {
    pub layer_type: EnumWrapper<LgLayerType>,
    pub zone: EnumWrapper<LocalZoneIndex>,
}

#[derive(Serialize, Deserialize, Default)]
pub enum LgLayerType {
    #[default] MainLayer,
    SecondaryLayer,
    ThirdLayer,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescriptiveData {
    pub custom_matchmaking_tier: Tier,
    pub dev_info: String,
    pub estimated_duration: LocalizedText,
    pub expedition_depth: i64,
    pub expedition_description: LocalizedText,
    pub is_extra_expedition: bool,
    pub prefix: String,
    pub progression_visual_style: EnumWrapper<ProgressionVisualStyle>,
    pub public_name: String,
    pub roleplayed_warden_intel: LocalizedText,
    pub skip_exp_number_in_name: bool,
    pub use_custom_matchmaking_tier: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Tier {
    TierA = 1,
    TierB = 2,
    TierC = 3,
    TierD = 4,
    TierE = 5,
    Surface = 99,
}

#[derive(Serialize, Deserialize)]
pub enum ProgressionVisualStyle {
    Normal,
    Story,
    External,
    ExternalStory,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionInExpeditionData {
    pub dimension_data: Reference<UndefinedReferenceItem>,
    pub dimension_index: EnumWrapper<DimensionIndex>,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DimensionIndex {
    #[serde(rename = "Reality")]
    #[default] Reality,
    #[serde(rename = "Dimension_1")]
    Dimension1,
    #[serde(rename = "Dimension_2")]
    Dimension2,
    #[serde(rename = "Dimension_20")]
    Dimension20,
    #[serde(rename = "Dimension_3")]
    Dimension3,
    #[serde(rename = "Dimension_4")]
    Dimension4,
    #[serde(rename = "Dimension_5")]
    Dimension5,
    #[serde(rename = "Dimension_6")]
    Dimension6,
    #[serde(rename = "Dimension_7")]
    Dimension7,
    #[serde(rename = "Dimension_8")]
    Dimension8,
    #[serde(rename = "Dimension_9")]
    Dimension9,
    #[serde(rename = "Dimension_10")]
    Dimension10,
    #[serde(rename = "Dimension_11")]
    Dimension11,
    #[serde(rename = "Dimension_12")]
    Dimension12,
    #[serde(rename = "Dimension_13")]
    Dimension13,
    #[serde(rename = "Dimension_14")]
    Dimension14,
    #[serde(rename = "Dimension_15")]
    Dimension15,
    #[serde(rename = "Dimension_16")]
    Dimension16,
    #[serde(rename = "Dimension_17")]
    Dimension17,
    #[serde(rename = "Dimension_18")]
    Dimension18,
    #[serde(rename = "Dimension_19")]
    Dimension19,
    #[serde(rename = "MAX_COUNT")]
    MaxCount,
    #[serde(rename = "ARENA_DIMENSION")]
    ArenaDimension,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionData {
    pub complex_resource_data: Reference<UndefinedReferenceItem>,
    pub dust_color: Color,
    pub dust_turbulence: f64,
    pub enemy_population: Reference<EnemyPopulation>,
    pub environment_wetness: f64,
    pub expedition_balance: Reference<UndefinedReferenceItem>,
    pub fog_settings: Reference<UndefinedReferenceItem>,
    pub light_settings: Reference<UndefinedReferenceItem>,
    #[serde(rename = "MLSLevelKit")]
    pub mls_level_kit: Reference<UndefinedReferenceItem>,
    pub scout_wave_population: Reference<SurvivalWavePopulation>,
    pub scout_wave_settings: Reference<SurvivalWaveSettings>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionIndex {
    pub exp: EnumWrapper<BuildAutoExpeditionNumber>,
    pub tier: EnumWrapper<Tier>,
}

#[derive(Serialize, Deserialize)]
pub enum BuildAutoExpeditionNumber {
    Exp1,
    Exp2,
    Exp3,
    Exp4,
    Exp5,
    Exp6,
    Exp7,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LayerData {
    pub artifact_data: ArtifactLayerData,
    pub bulkhead_door_controller_placements: Vec<BulkheadDoorPlacementData>,
    pub bulkhead_key_placements: Vec<Vec<BulkheadKeyPlacementElement>>,
    pub chained_objective_data: Vec<WardenObjectiveLayerData>,
    pub objective_data: WardenObjectiveLayerData,
    pub zones_with_bulkhead_entrance: Vec<EnumWrapper<LocalZoneIndex>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkheadDoorPlacementData {
    pub area_seed_offset: i64,
    pub marker_seed_offset: i64,
    pub placement_weights: ZonePlacementWeights,
    pub zone_index: EnumWrapper<LocalZoneIndex>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactLayerData {
    pub artifact_amount_multi: f64,
    #[serde(rename = "ArtifactLayerDistributionDataID")]
    pub artifact_layer_distribution_data_id: Reference<UndefinedReferenceItem>,
    pub artifact_zone_distributions: Vec<ArtifactZoneDistribution>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactZoneDistribution {
    pub advanced_artifact_weight: f64,
    pub basic_artifact_weight: f64,
    pub specialized_artifact_weight: f64,
    pub zone: EnumWrapper<LocalZoneIndex>,
}

#[derive(Serialize, Deserialize, Default)]
pub enum LocalZoneIndex {
    #[serde(rename = "Zone_0")]
    #[default] Zone0,
    #[serde(rename = "Zone_1")]
    Zone1,
    #[serde(rename = "Zone_2")]
    Zone2,
    #[serde(rename = "Zone_20")]
    Zone20,
    #[serde(rename = "Zone_3")]
    Zone3,
    #[serde(rename = "Zone_4")]
    Zone4,
    #[serde(rename = "Zone_5")]
    Zone5,
    #[serde(rename = "Zone_6")]
    Zone6,
    #[serde(rename = "Zone_7")]
    Zone7,
    #[serde(rename = "Zone_8")]
    Zone8,
    #[serde(rename = "Zone_9")]
    Zone9,
    #[serde(rename = "Zone_10")]
    Zone10,
    #[serde(rename = "Zone_11")]
    Zone11,
    #[serde(rename = "Zone_12")]
    Zone12,
    #[serde(rename = "Zone_13")]
    Zone13,
    #[serde(rename = "Zone_14")]
    Zone14,
    #[serde(rename = "Zone_15")]
    Zone15,
    #[serde(rename = "Zone_16")]
    Zone16,
    #[serde(rename = "Zone_17")]
    Zone17,
    #[serde(rename = "Zone_18")]
    Zone18,
    #[serde(rename = "Zone_19")]
    Zone19,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WardenObjectiveLayerData {
    pub data_block_id: Reference<UndefinedReferenceItem>,
    pub win_condition: EnumWrapper<WardenObjectiveWinCondition>,
    pub zone_placement_datas: Vec<Vec<BulkheadKeyPlacementElement>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkheadKeyPlacementElement {
    pub dimension_index: DimensionIndex,
    pub local_index: EnumWrapper<LocalZoneIndex>,
    pub weights: ZonePlacementWeights,
}

#[derive(Serialize, Deserialize)]
pub enum WardenObjectiveWinCondition {
    GoToElevator,
    GoToExitGeo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ZonePlacementWeights {
    pub start: f64,
    pub middle: f64,
    pub end: f64,
}
