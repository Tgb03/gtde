// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::GameDataBlockWrapperOfArchetypeDataBlock;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: GameDataBlockWrapperOfArchetypeDataBlock = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfArchetypeDataBlock {
    pub blocks: Option<Vec<ArchetypeDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArchetypeDataBlock {
    pub aim_sequence: Option<Vec<AimSequenceElement>>,

    pub aim_spread: Option<f64>,

    pub aim_transition_time: Option<f64>,

    pub burst_delay: Option<f64>,

    pub burst_shot_count: Option<i64>,

    pub cost_of_bullet: Option<f64>,

    pub damage: Option<f64>,

    pub damage_booster_effect: Option<Effect>,

    pub damage_falloff: Option<PurpleVector2>,

    pub default_clip_size: Option<i64>,

    pub default_reload_time: Option<f64>,

    pub description: Option<PurpleDescription>,

    pub equip_sequence: Option<Vec<AimSequenceElement>>,

    pub equip_transition_time: Option<f64>,

    pub fire_mode: Option<FireMode>,

    pub hip_fire_spread: Option<f64>,

    pub piercing_bullets: Option<bool>,

    pub piercing_damage_count_limit: Option<i64>,

    pub precision_damage_multi: Option<f64>,

    pub public_name: Option<PurpleDescription>,

    #[serde(rename = "RecoilDataID")]
    pub recoil_data_id: Option<i64>,

    #[serde(rename = "Sentry_CostOfBulletTagMulti")]
    pub sentry_cost_of_bullet_tag_multi: Option<f64>,

    #[serde(rename = "Sentry_DamageTagMulti")]
    pub sentry_damage_tag_multi: Option<f64>,

    #[serde(rename = "Sentry_DetectionMaxAngle")]
    pub sentry_detection_max_angle: Option<f64>,

    #[serde(rename = "Sentry_DetectionMaxRange")]
    pub sentry_detection_max_range: Option<f64>,

    #[serde(rename = "Sentry_FireTagOnly")]
    pub sentry_fire_tag_only: Option<bool>,

    #[serde(rename = "Sentry_FireTowardsTargetInsteadOfForward")]
    pub sentry_fire_towards_target_instead_of_forward: Option<bool>,

    #[serde(rename = "Sentry_LegacyEnemyDetection")]
    pub sentry_legacy_enemy_detection: Option<bool>,

    #[serde(rename = "Sentry_LongRangeThreshold")]
    pub sentry_long_range_threshold: Option<f64>,

    #[serde(rename = "Sentry_PrioTag")]
    pub sentry_prio_tag: Option<bool>,

    #[serde(rename = "Sentry_RotationSpeed")]
    pub sentry_rotation_speed: Option<f64>,

    #[serde(rename = "Sentry_RotationSpeedTagMulti")]
    pub sentry_rotation_speed_tag_multi: Option<f64>,

    #[serde(rename = "Sentry_ShortRangeThreshold")]
    pub sentry_short_range_threshold: Option<f64>,

    #[serde(rename = "Sentry_ShotDelayTagMulti")]
    pub sentry_shot_delay_tag_multi: Option<f64>,

    #[serde(rename = "Sentry_StaggerDamageTagMulti")]
    pub sentry_stagger_damage_tag_multi: Option<f64>,

    #[serde(rename = "Sentry_StartFireDelay")]
    pub sentry_start_fire_delay: Option<f64>,

    #[serde(rename = "Sentry_StartFireDelayTagMulti")]
    pub sentry_start_fire_delay_tag_multi: Option<f64>,

    pub shell_casing_size: Option<f64>,

    pub shell_casing_speed_range: Option<PurpleVector2>,

    pub shot_delay: Option<f64>,

    pub shotgun_bullet_count: Option<i64>,

    pub shotgun_bullet_spread: Option<i64>,

    pub shotgun_cone_size: Option<i64>,

    pub special_chargetup_time: Option<f64>,

    pub special_cooldown_time: Option<f64>,

    pub special_semi_burst_count_timeout: Option<f64>,

    pub stagger_damage_multi: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AimSequenceElement {
    pub string_data: Option<String>,

    pub trigger_time: Option<f64>,

    #[serde(rename = "Type")]
    pub weapon_anim_sequence_item_type: Option<AimSequenceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AimSequenceType {
    Enum(EWeaponAnimSequenceItemType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWeaponAnimSequenceItemType {
    #[serde(rename = "DoUpdateAmmo")]
    DoUpdateAmmo,

    #[serde(rename = "EjectCasing")]
    EjectCasing,

    Empty,

    #[serde(rename = "FrontPartAnim")]
    FrontPartAnim,

    #[serde(rename = "LeftHandAnim")]
    LeftHandAnim,

    #[serde(rename = "LeftHandMagAnim")]
    LeftHandMagAnim,

    #[serde(rename = "MagazineAction")]
    MagazineAction,

    #[serde(rename = "ReceiverAnim")]
    ReceiverAnim,

    #[serde(rename = "RightHandAnim")]
    RightHandAnim,

    Sound,

    #[serde(rename = "StockAnim")]
    StockAnim,

    #[serde(rename = "WeaponMovementAnim")]
    WeaponMovementAnim,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Effect {
    Enum(AgentModifier),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AgentModifier {
    #[serde(rename = "AmmoSupport")]
    AmmoSupport,

    #[serde(rename = "AssaultRifleDamage")]
    AssaultRifleDamage,

    #[serde(rename = "AutoPistolDamage")]
    AutoPistolDamage,

    #[serde(rename = "BioscanSpeed")]
    BioscanSpeed,

    #[serde(rename = "BullpupRifleDamage")]
    BullpupRifleDamage,

    #[serde(rename = "BurstCannonDamage")]
    BurstCannonDamage,

    #[serde(rename = "BurstRifleDamage")]
    BurstRifleDamage,

    #[serde(rename = "CarbineDamage")]
    CarbineDamage,

    #[serde(rename = "ChokeModShotgunDamage")]
    ChokeModShotgunDamage,

    #[serde(rename = "CombatShotgunDamage")]
    CombatShotgunDamage,

    #[serde(rename = "ComputerProcessingSpeed")]
    ComputerProcessingSpeed,

    #[serde(rename = "DamageOverTime")]
    DamageOverTime,

    #[serde(rename = "DMRDamage")]
    DmrDamage,

    #[serde(rename = "DoubleTapRifle")]
    DoubleTapRifle,

    #[serde(rename = "ExplosionResistance")]
    ExplosionResistance,

    #[serde(rename = "FogRepellerEffect")]
    FogRepellerEffect,

    #[serde(rename = "GlowstickEffect")]
    GlowstickEffect,

    #[serde(rename = "GlueEfficiency")]
    GlueEfficiency,

    #[serde(rename = "GlueStrength")]
    GlueStrength,

    #[serde(rename = "HackingProficiency")]
    HackingProficiency,

    #[serde(rename = "HealSupport")]
    HealSupport,

    #[serde(rename = "HELDamage")]
    HelDamage,

    #[serde(rename = "InfectionResistance")]
    InfectionResistance,

    #[serde(rename = "InitialAmmoSpecial")]
    InitialAmmoSpecial,

    #[serde(rename = "InitialAmmoStandard")]
    InitialAmmoStandard,

    #[serde(rename = "InitialAmmoTool")]
    InitialAmmoTool,

    #[serde(rename = "MachineGunDamage")]
    MachineGunDamage,

    #[serde(rename = "MachinePistolDamage")]
    MachinePistolDamage,

    #[serde(rename = "MeleeDamage")]
    MeleeDamage,

    #[serde(rename = "MeleeResistance")]
    MeleeResistance,

    #[serde(rename = "MovementAcceleration")]
    MovementAcceleration,

    #[serde(rename = "MovementSpeed")]
    MovementSpeed,

    #[serde(rename = "Nanoswarm_Shield")]
    NanoswarmShield,

    #[serde(rename = "Nanoswarm_Weakness")]
    NanoswarmWeakness,

    None,

    #[serde(rename = "PistolDamage")]
    PistolDamage,

    #[serde(rename = "ProjectileResistance")]
    ProjectileResistance,

    #[serde(rename = "RegenerationCap")]
    RegenerationCap,

    #[serde(rename = "RegenerationSpeed")]
    RegenerationSpeed,

    #[serde(rename = "ReviveSpeedSupport")]
    ReviveSpeedSupport,

    #[serde(rename = "ReviveStartHealthSupport")]
    ReviveStartHealthSupport,

    #[serde(rename = "RevolverDamage")]
    RevolverDamage,

    #[serde(rename = "RifleDamage")]
    RifleDamage,

    #[serde(rename = "ScannerRechargeSpeed")]
    ScannerRechargeSpeed,

    #[serde(rename = "SentryGunDamage")]
    SentryGunDamage,

    #[serde(rename = "SentryGunLongRangeDamage")]
    SentryGunLongRangeDamage,

    #[serde(rename = "SentryGunShortRangeDamage")]
    SentryGunShortRangeDamage,

    #[serde(rename = "SentryGunSpeed")]
    SentryGunSpeed,

    #[serde(rename = "ShotgunDamage")]
    ShotgunDamage,

    #[serde(rename = "SMGDamage")]
    SmgDamage,

    #[serde(rename = "SniperDamage")]
    SniperDamage,

    #[serde(rename = "SpecialWeaponDamage")]
    SpecialWeaponDamage,

    #[serde(rename = "StandardWeaponDamage")]
    StandardWeaponDamage,

    #[serde(rename = "TripMineDamage")]
    TripMineDamage,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleDescription {
    Integer(i64),

    PurpleLocalizedText(PurpleLocalizedText),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FireMode {
    Enum(EWeaponFireMode),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWeaponFireMode {
    Auto,

    Burst,

    Semi,

    #[serde(rename = "SemiBurst")]
    SemiBurst,

    #[serde(rename = "SentryGunAuto")]
    SentryGunAuto,

    #[serde(rename = "SentryGunBurst")]
    SentryGunBurst,

    #[serde(rename = "SentryGunSemi")]
    SentryGunSemi,

    #[serde(rename = "SentryGunShotgunSemi")]
    SentryGunShotgunSemi,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfArtifactDataBlock {
    pub blocks: Option<Vec<ArtifactDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactDataBlock {
    pub artifact_category: Option<ArtifactCategoryUnion>,

    pub prefab_path: Option<String>,

    pub prefab_paths: Option<Vec<String>>,

    pub public_name: Option<String>,

    #[serde(rename = "TagsID")]
    pub tags_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ArtifactCategoryUnion {
    Enum(ArtifactCategory),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ArtifactCategory {
    Common,

    #[serde(rename = "_COUNT")]
    Count,

    Rare,

    Uncommon,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfArtifactDistributionDataBlock {
    pub blocks: Option<Vec<ArtifactDistributionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactDistributionDataBlock {
    pub advanced_weight: Option<f64>,

    pub basic_weight: Option<f64>,

    pub specialized_weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfArtifactTagDataBlock {
    pub blocks: Option<Vec<ArtifactTagDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactTagDataBlock {
    pub tags: Option<Vec<Tag>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tag {
    Enum(ArtifactTags),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ArtifactTags {
    #[serde(rename = "AllComplex")]
    AllComplex,

    Any,

    Corpse,

    #[serde(rename = "DataCenter")]
    DataCenter,

    #[serde(rename = "DigSite")]
    DigSite,

    Floodways,

    Ground,

    Lab,

    None,

    Refinery,

    Shelf,

    Storage,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfAtmosphereDataBlock {
    pub blocks: Option<Vec<AtmosphereDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AtmosphereDataBlock {
    pub light_color: Option<PurpleColor>,

    pub planet_albedo: Option<PurpleColor>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfBigPickupDistributionDataBlock {
    pub blocks: Option<Vec<BigPickupDistributionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BigPickupDistributionDataBlock {
    pub spawn_data: Option<Vec<BigPickupSpawnData>>,

    pub spawns_per_zone: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BigPickupSpawnData {
    #[serde(rename = "ItemID")]
    pub item_id: Option<i64>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfBoosterImplantConditionDataBlock {
    pub blocks: Option<Vec<BoosterImplantConditionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoosterImplantConditionDataBlock {
    pub condition: Option<Condition>,

    pub description: Option<FluffyDescription>,

    pub icon_path: Option<String>,

    pub public_name: Option<FluffyDescription>,

    pub public_short_name: Option<FluffyDescription>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Condition {
    Enum(BoosterCondition),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum BoosterCondition {
    #[serde(rename = "HasFlashlightOff")]
    HasFlashlightOff,

    #[serde(rename = "HasFlashlightOn")]
    HasFlashlightOn,

    #[serde(rename = "HasFullHealth")]
    HasFullHealth,

    #[serde(rename = "HasHighHealth")]
    HasHighHealth,

    #[serde(rename = "HasLessThanAmmoSmall")]
    HasLessThanAmmoSmall,

    #[serde(rename = "HasLowHealth")]
    HasLowHealth,

    #[serde(rename = "HasMoreThanAmmoLarge")]
    HasMoreThanAmmoLarge,

    #[serde(rename = "IsCloseToEnemy")]
    IsCloseToEnemy,

    #[serde(rename = "IsFarAwayFromEnemy")]
    IsFarAwayFromEnemy,

    #[serde(rename = "IsInBioscan")]
    IsInBioscan,

    #[serde(rename = "IsInFogRepellerRadius")]
    IsInFogRepellerRadius,

    #[serde(rename = "IsLastManStanding")]
    IsLastManStanding,

    #[serde(rename = "IsNextToGlowstick")]
    IsNextToGlowstick,

    #[serde(rename = "IsNextToTeammate")]
    IsNextToTeammate,

    #[serde(rename = "IsOutsideBioscan")]
    IsOutsideBioscan,

    #[serde(rename = "IsTeamAliveFour")]
    IsTeamAliveFour,

    #[serde(rename = "IsTeamAliveThree")]
    IsTeamAliveThree,

    #[serde(rename = "IsTeamAliveTwo")]
    IsTeamAliveTwo,

    #[serde(rename = "IsWieldingCarryItem")]
    IsWieldingCarryItem,

    #[serde(rename = "IsWieldingConsumable")]
    IsWieldingConsumable,

    #[serde(rename = "IsWieldingHackingTool")]
    IsWieldingHackingTool,

    #[serde(rename = "IsWieldingMelee")]
    IsWieldingMelee,

    #[serde(rename = "IsWieldingResourcePack")]
    IsWieldingResourcePack,

    #[serde(rename = "IsWieldingSpecialWeapon")]
    IsWieldingSpecialWeapon,

    #[serde(rename = "IsWieldingStandardWeapon")]
    IsWieldingStandardWeapon,

    #[serde(rename = "IsWieldingTool")]
    IsWieldingTool,

    None,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyDescription {
    FluffyLocalizedText(FluffyLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfBoosterImplantEffectDataBlock {
    pub blocks: Option<Vec<BoosterImplantEffectDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoosterImplantEffectDataBlock {
    pub booster_effect_category: Option<BoosterEffectCategoryUnion>,

    pub description: Option<DescriptionNegativeUnion>,

    pub description_negative: Option<DescriptionNegativeUnion>,

    pub effect: Option<Effect>,

    pub public_name: Option<DescriptionNegativeUnion>,

    pub public_short_name: Option<DescriptionNegativeUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoosterEffectCategoryUnion {
    Enum(BoosterEffectCategory),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum BoosterEffectCategory {
    Damage,

    Health,

    #[serde(rename = "InitialState")]
    InitialState,

    #[serde(rename = "ProcessingSpeed")]
    ProcessingSpeed,

    Tool,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DescriptionNegativeUnion {
    DescriptionNegativeLocalizedText(DescriptionNegativeLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescriptionNegativeLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfBoosterImplantTemplateDataBlock {
    pub blocks: Option<Vec<BoosterImplantTemplateDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoosterImplantTemplateDataBlock {
    pub conditions: Option<Vec<i64>>,

    pub deprecated: Option<bool>,

    pub description: Option<TentacledDescription>,

    pub drop_weight: Option<f64>,

    pub duration_range: Option<FluffyVector2>,

    pub effects: Option<Vec<BoosterImplantEffectInstance>>,

    pub implant_category: Option<ImplantCategory>,

    pub main_effect_type: Option<BoosterEffectCategoryUnion>,

    pub public_name: Option<TentacledDescription>,

    pub random_conditions: Option<Vec<i64>>,

    pub random_effects: Option<Vec<Vec<BoosterImplantEffectInstance>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledDescription {
    Integer(i64),

    String(String),

    TentacledLocalizedText(TentacledLocalizedText),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoosterImplantEffectInstance {
    pub booster_implant_effect: Option<i64>,

    pub max_value: Option<f64>,

    pub min_value: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImplantCategory {
    Enum(BoosterImplantCategory),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum BoosterImplantCategory {
    Aggressive,

    Bold,

    #[serde(rename = "_COUNT")]
    Count,

    Muted,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfChainedPuzzleDataBlock {
    pub blocks: Option<Vec<ChainedPuzzleDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainedPuzzleDataBlock {
    pub alarm_sound_start: Option<i64>,

    pub alarm_sound_stop: Option<i64>,

    pub chained_puzzle: Option<Vec<ChainedPuzzleComponent>>,

    pub disable_survival_wave_on_complete: Option<bool>,

    #[serde(rename = "OnlyShowHUDWhenPlayerIsClose")]
    pub only_show_hud_when_player_is_close: Option<bool>,

    pub public_alarm_name: Option<String>,

    pub survival_wave_area_distance: Option<i64>,

    pub survival_wave_population: Option<i64>,

    pub survival_wave_settings: Option<i64>,

    pub trigger_alarm_on_activate: Option<bool>,

    pub use_random_positions: Option<bool>,

    pub wanted_distance_between_puzzle_components: Option<f64>,

    pub wanted_distance_from_start_pos: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainedPuzzleComponent {
    pub puzzle_type: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfChainedPuzzleTypeDataBlock {
    pub blocks: Option<Vec<ChainedPuzzleTypeDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChainedPuzzleTypeDataBlock {
    pub prefab: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfCloudsDataBlock {
    pub blocks: Option<Vec<CloudsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CloudsDataBlock {
    pub altitude: Option<f64>,

    pub ambient_occlusion: Option<f64>,

    pub anvil: Option<f64>,

    pub belly: Option<f64>,

    pub center: Option<f64>,

    pub coverage_size: Option<f64>,

    pub coverage_texture: Option<String>,

    pub crown: Option<f64>,

    pub height: Option<f64>,

    pub noise_intensity1: Option<f64>,

    pub noise_intensity2: Option<f64>,

    pub noise_scale1: Option<f64>,

    pub noise_scale2: Option<f64>,

    pub noise_texture: Option<String>,

    pub softness: Option<f64>,

    #[serde(rename = "startupShard")]
    pub startup_shard: Option<Shard>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Shard {
    Enum(AssetBundleShard),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AssetBundleShard {
    S1,

    S10,

    S11,

    S12,

    S13,

    S14,

    S15,

    S16,

    S17,

    S18,

    S19,

    S2,

    S20,

    S3,

    S4,

    S5,

    S6,

    S7,

    S8,

    S9,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfCommodityDataBlock {
    pub blocks: Option<Vec<CommodityDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommodityDataBlock {
    pub item: Option<i64>,

    pub pack_size: Option<PackSize>,

    pub playfab_currency: Option<PlayfabCurrency>,

    pub playfab_currency_value: Option<i64>,

    pub public_name: Option<String>,

    #[serde(rename = "Type")]
    pub commodity_data_block_type: Option<PurpleType>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleType {
    Enum(ECommodityType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ECommodityType {
    Artifact,

    Data,

    #[serde(rename = "LiquidSample")]
    LiquidSample,

    #[serde(rename = "PhysicalSample")]
    PhysicalSample,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackSize {
    Enum(ECommodityPackSize),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ECommodityPackSize {
    Large,

    Medium,

    Small,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayfabCurrency {
    Enum(EPlayfabCurrency),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EPlayfabCurrency {
    #[serde(rename = "CurrA")]
    CurrA,

    #[serde(rename = "CurrB")]
    CurrB,

    #[serde(rename = "CurrC")]
    CurrC,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfComplexResourceSetDataBlock {
    pub blocks: Option<Vec<ComplexResourceSetDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComplexResourceSetDataBlock {
    pub bundle_name: Option<BundleName>,

    pub complex_type: Option<ComplexType>,

    #[serde(rename = "CustomGeomorphs_Challenge_1x1")]
    pub custom_geomorphs_challenge_1_x1: Option<Vec<ResourceData>>,

    #[serde(rename = "CustomGeomorphs_Exit_1x1")]
    pub custom_geomorphs_exit_1_x1: Option<Vec<ResourceData>>,

    #[serde(rename = "CustomGeomorphs_Objective_1x1")]
    pub custom_geomorphs_objective_1_x1: Option<Vec<ResourceData>>,

    pub double_drop_plugs_no_gates: Option<Vec<ResourceData>>,

    pub double_drop_plugs_with_gates: Option<Vec<ResourceData>>,

    #[serde(rename = "ElevatorShafts_1x1")]
    pub elevator_shafts_1_x1: Option<Vec<ResourceData>>,

    #[serde(rename = "GeomorphTiles_1x1")]
    pub geomorph_tiles_1_x1: Option<Vec<ResourceData>>,

    #[serde(rename = "GeomorphTiles_2x1")]
    pub geomorph_tiles_2_x1: Option<Vec<ResourceData>>,

    #[serde(rename = "GeomorphTiles_2x2")]
    pub geomorph_tiles_2_x2: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_05m")]
    pub ladders_05_m: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_1m")]
    pub ladders_1_m: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_2m")]
    pub ladders_2_m: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_4m")]
    pub ladders_4_m: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_Bottom")]
    pub ladders_bottom: Option<Vec<ResourceData>>,

    #[serde(rename = "Ladders_Top")]
    pub ladders_top: Option<Vec<ResourceData>>,

    pub large_apex_gates: Option<Vec<ResourceData>>,

    pub large_bulkhead_gates: Option<Vec<ResourceData>>,

    pub large_destroyed_caps: Option<Vec<ResourceData>>,

    pub large_main_path_bulkhead_gates: Option<Vec<ResourceData>>,

    pub large_security_gates: Option<Vec<ResourceData>>,

    pub large_wall_and_destroyed_caps: Option<Vec<ResourceData>>,

    pub large_wall_caps: Option<Vec<ResourceData>>,

    pub large_weak_gates: Option<Vec<ResourceData>>,

    pub level_gen_config: Option<LevelGenConfig>,

    pub medium_apex_gates: Option<Vec<ResourceData>>,

    pub medium_bulkhead_gates: Option<Vec<ResourceData>>,

    pub medium_destroyed_caps: Option<Vec<ResourceData>>,

    pub medium_main_path_bulkhead_gates: Option<Vec<ResourceData>>,

    pub medium_security_gates: Option<Vec<ResourceData>>,

    pub medium_wall_and_destroyed_caps: Option<Vec<ResourceData>>,

    pub medium_wall_caps: Option<Vec<ResourceData>>,

    pub medium_weak_gates: Option<Vec<ResourceData>>,

    pub plug_caps: Option<Vec<ResourceData>>,

    pub primare_sub_complex_used: Option<PrimareSubComplexUsedUnion>,

    pub randomize_geomorph_order: Option<bool>,

    pub single_drop_plugs_no_gates: Option<Vec<ResourceData>>,

    pub single_drop_plugs_with_gates: Option<Vec<ResourceData>>,

    pub small_apex_gates: Option<Vec<ResourceData>>,

    pub small_bulkhead_gates: Option<Vec<ResourceData>>,

    pub small_destroyed_caps: Option<Vec<ResourceData>>,

    pub small_main_path_bulkhead_gates: Option<Vec<ResourceData>>,

    pub small_security_gates: Option<Vec<ResourceData>>,

    pub small_wall_and_destroyed_caps: Option<Vec<ResourceData>>,

    pub small_wall_caps: Option<Vec<ResourceData>>,

    pub small_weak_gates: Option<Vec<ResourceData>>,

    pub straight_plugs_no_gates: Option<Vec<ResourceData>>,

    pub straight_plugs_with_gates: Option<Vec<ResourceData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BundleName {
    Enum(AssetBundleName),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AssetBundleName {
    #[serde(rename = "Complex_Mining")]
    ComplexMining,

    #[serde(rename = "Complex_Service")]
    ComplexService,

    #[serde(rename = "Complex_Shared")]
    ComplexShared,

    #[serde(rename = "Complex_Tech")]
    ComplexTech,

    Enemies,

    #[serde(rename = "Gear_Material")]
    GearMaterial,

    #[serde(rename = "Gear_Melee_Handle")]
    GearMeleeHandle,

    #[serde(rename = "Gear_Melee_Head")]
    GearMeleeHead,

    #[serde(rename = "Gear_Melee_Neck")]
    GearMeleeNeck,

    #[serde(rename = "Gear_Melee_Pommel")]
    GearMeleePommel,

    #[serde(rename = "Gear_Tool_Delivery")]
    GearToolDelivery,

    #[serde(rename = "Gear_Tool_Grip")]
    GearToolGrip,

    #[serde(rename = "Gear_Tool_Main")]
    GearToolMain,

    #[serde(rename = "Gear_Tool_Payload")]
    GearToolPayload,

    #[serde(rename = "Gear_Tool_Screen")]
    GearToolScreen,

    #[serde(rename = "Gear_Tool_Targeting")]
    GearToolTargeting,

    #[serde(rename = "Gear_Weapon_Flashlight")]
    GearWeaponFlashlight,

    #[serde(rename = "Gear_Weapon_Front")]
    GearWeaponFront,

    #[serde(rename = "Gear_Weapon_Mag")]
    GearWeaponMag,

    #[serde(rename = "Gear_Weapon_Receiver")]
    GearWeaponReceiver,

    #[serde(rename = "Gear_Weapon_Sight")]
    GearWeaponSight,

    #[serde(rename = "Gear_Weapon_Stock")]
    GearWeaponStock,

    None,

    Startup,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ComplexType {
    Enum(Complex),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum Complex {
    Mining,

    Service,

    Tech,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceData {
    pub prefab: Option<String>,

    pub shard: Option<Shard>,

    pub sub_complex: Option<PrimareSubComplexUsedUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrimareSubComplexUsedUnion {
    Enum(SubComplex),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum SubComplex {
    All,

    #[serde(rename = "DataCenter")]
    DataCenter,

    #[serde(rename = "DigSite")]
    DigSite,

    Floodways,

    Gardens,

    Lab,

    #[serde(rename = "Mining_Portal")]
    MiningPortal,

    #[serde(rename = "Mining_Reactor")]
    MiningReactor,

    #[serde(rename = "Plug_SubComplex_Transition")]
    PlugSubComplexTransition,

    Refinery,

    Storage,

    #[serde(rename = "Tech_Portal")]
    TechPortal,

    #[serde(rename = "Tech_Reactor")]
    TechReactor,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelGenConfig {
    pub altitude_offset: Option<f64>,

    pub cell_dimension: Option<f64>,

    pub grid_size: Option<i64>,

    pub level_progression: Option<LevelProgression>,

    pub transition_direction: Option<TransitionDirection>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LevelProgression {
    Enum(LgLevelProgression),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgLevelProgression {
    #[serde(rename = "EndLevel")]
    EndLevel,

    #[serde(rename = "MidLevel")]
    MidLevel,

    #[serde(rename = "StartLevel")]
    StartLevel,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransitionDirection {
    Enum(LgFloorTransitionDirection),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgFloorTransitionDirection {
    #[serde(rename = "FloorDown")]
    FloorDown,

    #[serde(rename = "FloorUp")]
    FloorUp,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfConsumableDistributionDataBlock {
    pub blocks: Option<Vec<ConsumableDistributionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumableDistributionDataBlock {
    pub chance_to_spawn_in_resource_container: Option<f64>,

    pub spawn_data: Option<Vec<ConsumableSpawnData>>,

    pub spawns_per_zone: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConsumableSpawnData {
    #[serde(rename = "ItemID")]
    pub item_id: Option<i64>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfCustomAssetShardDataBlock {
    pub blocks: Option<Vec<CustomAssetShardDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomAssetShardDataBlock {
    pub assets: Option<Vec<AssetData>>,

    pub bundle_name: Option<BundleName>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AssetData {
    pub asset_path: Option<String>,

    pub shard: Option<Shard>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfDimensionDataBlock {
    pub blocks: Option<Vec<DimensionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionDataBlock {
    pub dimension_data: Option<DimensionData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionData {
    pub aerial_scale: Option<f64>,

    pub ambient_intensity: Option<f64>,

    pub atmosphere_data: Option<i64>,

    pub atmosphere_density: Option<f64>,

    pub clouds_coverage: Option<f64>,

    pub clouds_crawling: Option<f64>,

    pub clouds_data: Option<i64>,

    pub clouds_density: Option<f64>,

    pub clouds_fade: Option<f64>,

    pub clouds_shadow_opacity: Option<f64>,

    pub clouds_sharpness: Option<f64>,

    pub clouds_timescale: Option<f64>,

    pub dimension_fog_data: Option<i64>,

    pub dimension_geomorph: Option<String>,

    #[serde(rename = "DimensionResourceSetID")]
    pub dimension_resource_set_id: Option<i64>,

    #[serde(rename = "DisableVFXEventOnWarp")]
    pub disable_vfx_event_on_warp: Option<bool>,

    pub dust_alpha_boost: Option<f64>,

    pub dust_color: Option<DimensionDataColor>,

    pub dust_turbulence: Option<f64>,

    pub environment_wetness: Option<f64>,

    pub events_on_boss_death: Option<Vec<DimensionDataEventsOnBossDeath>>,

    pub exposure: Option<f64>,

    pub forbid_carry_item_warps: Option<bool>,

    pub forbid_terminals_in_dimension: Option<bool>,

    pub forbid_wave_spawning: Option<bool>,

    pub godray_exponent: Option<f64>,

    pub godray_range: Option<f64>,

    pub is_outside: Option<bool>,

    pub is_static_dimension: Option<bool>,

    pub leave_deployables_on_warp: Option<bool>,

    pub level_layout_data: Option<i64>,

    pub light_azimuth: Option<f64>,

    pub light_elevation: Option<f64>,

    pub light_intensity: Option<f64>,

    pub linked_to_layer: Option<Layer>,

    pub mie_g: Option<f64>,

    pub mie_scattering: Option<f64>,

    pub multiple_scattering: Option<f64>,

    pub objective_type: Option<ObjectiveType>,

    pub reflections_intensity: Option<f64>,

    pub sandstorm: Option<bool>,

    pub sandstorm_edge_a: Option<f64>,

    pub sandstorm_edge_b: Option<f64>,

    pub sandstorm_min_fog: Option<f64>,

    pub sound_event_on_warp_to: Option<i64>,

    pub static_alias_override: Option<i64>,

    pub static_alias_prefix_override: Option<String>,

    pub static_alias_prefix_short_override: Option<String>,

    pub static_allow_resource_container_allocation: Option<bool>,

    pub static_allow_small_pickups_allocation: Option<bool>,

    pub static_big_pickup_distribution_in_zone: Option<i64>,

    pub static_consumable_distribution_in_zone: Option<i64>,

    pub static_disinfection_multi: Option<f64>,

    pub static_enemy_spawning_in_zone: Option<Vec<StaticEnemySpawningInZoneElement>>,

    pub static_force_big_pickups_allocation: Option<bool>,

    pub static_ground_spawners_in_zone: Option<StaticGroundSpawnersInZone>,

    pub static_health_multi: Option<f64>,

    pub static_light_seed: Option<i64>,

    pub static_light_settings: Option<i64>,

    pub static_marker_seed: Option<i64>,

    pub static_terminal_placements: Option<Vec<StaticTerminalPlacementElement>>,

    pub static_tool_ammo_multi: Option<f64>,

    pub static_weapon_ammo_multi: Option<f64>,

    pub static_zone_seed: Option<i64>,

    pub use_local_dimension_seeds: Option<bool>,

    pub vertical_extents_down: Option<f64>,

    pub vertical_extents_up: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct DimensionDataColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionDataEventsOnBossDeath {
    pub chain_puzzle: Option<i64>,

    pub clear_dimension: Option<bool>,

    pub condition: Option<PurpleWorldEventConditionPair>,

    pub count: Option<i64>,

    pub custom_sub_objective: Option<PurpleCustomSubObjective>,

    pub custom_sub_objective_header: Option<PurpleCustomSubObjective>,

    pub delay: Option<f64>,

    #[serde(rename = "DialogueID")]
    pub dialogue_id: Option<i64>,

    pub dimension_index: Option<DimensionIndex>,

    pub duration: Option<f64>,

    pub enabled: Option<bool>,

    #[serde(rename = "EnemyID")]
    pub enemy_id: Option<i64>,

    pub enemy_wave_data: Option<PurpleGenericEnemyWaveData>,

    pub fog_setting: Option<i64>,

    pub fog_transition_duration: Option<f64>,

    pub layer: Option<Layer>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub position: Option<PurpleVector3>,

    #[serde(rename = "SoundID")]
    pub sound_id: Option<i64>,

    pub sound_subtitle: Option<PurpleCustomSubObjective>,

    pub terminal_command: Option<TerminalCommand>,

    pub terminal_command_rule: Option<AlCommandRule>,

    pub trigger: Option<EventsOnBossDeathTrigger>,

    #[serde(rename = "Type")]
    pub warden_objective_event_data_type: Option<EventsOnBossDeathType>,

    pub use_static_bioscan_points: Option<bool>,

    pub warden_intel: Option<PurpleCustomSubObjective>,

    pub world_event_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleWorldEventConditionPair {
    pub condition_index: Option<i64>,

    pub is_true: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PurpleCustomSubObjective {
    Integer(i64),

    StickyLocalizedText(StickyLocalizedText),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionIndex {
    Enum(EDimensionIndex),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EDimensionIndex {
    #[serde(rename = "ARENA_DIMENSION")]
    ArenaDimension,

    #[serde(rename = "Dimension_1")]
    Dimension1,

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

    #[serde(rename = "MAX_COUNT")]
    MaxCount,

    #[serde(rename = "Reality")]
    Reality,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleGenericEnemyWaveData {
    pub area_distance: Option<i64>,

    pub intel_message: Option<PurpleCustomSubObjective>,

    pub spawn_delay: Option<f64>,

    pub trigger_alarm: Option<bool>,

    pub wave_population: Option<i64>,

    pub wave_settings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Layer {
    Enum(LgLayerType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgLayerType {
    #[serde(rename = "MainLayer")]
    MainLayer,

    #[serde(rename = "SecondaryLayer")]
    SecondaryLayer,

    #[serde(rename = "ThirdLayer")]
    ThirdLayer,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildFromLocalIndexUnion {
    Enum(ELocalZoneIndex),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ELocalZoneIndex {
    #[serde(rename = "Zone_0")]
    Zone0,

    #[serde(rename = "Zone_1")]
    Zone1,

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
}

#[derive(Serialize, Deserialize)]
pub struct PurpleVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TerminalCommand {
    Enum(TermCommand),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum TermCommand {
    Activate,

    #[serde(rename = "ActivateBeacon")]
    ActivateBeacon,

    Close,

    Cls,

    Commands,

    Deactivate,

    #[serde(rename = "DisableAlarm")]
    DisableAlarm,

    #[serde(rename = "DownloadData")]
    DownloadData,

    #[serde(rename = "EmptyLine")]
    EmptyLine,

    Exit,

    Find,

    Help,

    Info,

    #[serde(rename = "InvalidCommand")]
    InvalidCommand,

    #[serde(rename = "ListLogs")]
    ListLogs,

    Locate,

    #[serde(rename = "MAX_COUNT")]
    MaxCount,

    None,

    Open,

    Override,

    Ping,

    Query,

    #[serde(rename = "ReactorShutdown")]
    ReactorShutdown,

    #[serde(rename = "ReactorStartup")]
    ReactorStartup,

    #[serde(rename = "ReactorVerify")]
    ReactorVerify,

    #[serde(rename = "ReadLog")]
    ReadLog,

    #[serde(rename = "ShowList")]
    ShowList,

    Start,

    #[serde(rename = "TerminalCorruptedUplinkConnect")]
    TerminalCorruptedUplinkConnect,

    #[serde(rename = "TerminalCorruptedUplinkVerify")]
    TerminalCorruptedUplinkVerify,

    #[serde(rename = "TerminalUplinkConfirm")]
    TerminalUplinkConfirm,

    #[serde(rename = "TerminalUplinkConnect")]
    TerminalUplinkConnect,

    #[serde(rename = "TerminalUplinkVerify")]
    TerminalUplinkVerify,

    #[serde(rename = "TimedConnectionSend")]
    TimedConnectionSend,

    #[serde(rename = "TimedConnectionVerify")]
    TimedConnectionVerify,

    #[serde(rename = "TryUnlockingTerminal")]
    TryUnlockingTerminal,

    #[serde(rename = "UniqueCommand1")]
    UniqueCommand1,

    #[serde(rename = "UniqueCommand2")]
    UniqueCommand2,

    #[serde(rename = "UniqueCommand3")]
    UniqueCommand3,

    #[serde(rename = "UniqueCommand4")]
    UniqueCommand4,

    #[serde(rename = "UniqueCommand5")]
    UniqueCommand5,

    #[serde(rename = "UsedCommand")]
    UsedCommand,

    #[serde(rename = "ViewSecurityLog")]
    ViewSecurityLog,

    #[serde(rename = "WardenObjectiveGatherCommand")]
    WardenObjectiveGatherCommand,

    #[serde(rename = "WardenObjectiveSpecialCommand")]
    WardenObjectiveSpecialCommand,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlCommandRule {
    Enum(TermCommandRule),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum TermCommandRule {
    Normal,

    #[serde(rename = "OnlyOnce")]
    OnlyOnce,

    #[serde(rename = "OnlyOnceDelete")]
    OnlyOnceDelete,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventsOnBossDeathTrigger {
    Enum(EWardenObjectiveEventTrigger),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWardenObjectiveEventTrigger {
    None,

    #[serde(rename = "OnEnd")]
    OnEnd,

    #[serde(rename = "OnMid")]
    OnMid,

    #[serde(rename = "OnStart")]
    OnStart,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventsOnBossDeathType {
    Enum(EWardenObjectiveEventType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWardenObjectiveEventType {
    #[serde(rename = "ActivateChainedPuzzle")]
    ActivateChainedPuzzle,

    #[serde(rename = "AllLightsOff")]
    AllLightsOff,

    #[serde(rename = "AllLightsOn")]
    AllLightsOn,

    #[serde(rename = "AnimationTrigger")]
    AnimationTrigger,

    #[serde(rename = "DimensionFlashTeam")]
    DimensionFlashTeam,

    #[serde(rename = "DimensionWarpTeam")]
    DimensionWarpTeam,

    #[serde(rename = "EventBreak")]
    EventBreak,

    #[serde(rename = "ForceCompleteObjective")]
    ForceCompleteObjective,

    #[serde(rename = "LightsInZone")]
    LightsInZone,

    #[serde(rename = "LightsInZoneToggle")]
    LightsInZoneToggle,

    #[serde(rename = "LockSecurityDoor")]
    LockSecurityDoor,

    None,

    #[serde(rename = "OpenSecurityDoor")]
    OpenSecurityDoor,

    #[serde(rename = "PlaySound")]
    PlaySound,

    #[serde(rename = "SetFogSetting")]
    SetFogSetting,

    #[serde(rename = "SetNavMarker")]
    SetNavMarker,

    #[serde(rename = "SetTerminalCommand")]
    SetTerminalCommand,

    #[serde(rename = "SetWorldEventCondition")]
    SetWorldEventCondition,

    #[serde(rename = "SpawnEnemyOnPoint")]
    SpawnEnemyOnPoint,

    #[serde(rename = "SpawnEnemyWave")]
    SpawnEnemyWave,

    #[serde(rename = "StepProgressionObjective")]
    StepProgressionObjective,

    #[serde(rename = "StopEnemyWaves")]
    StopEnemyWaves,

    #[serde(rename = "UnlockSecurityDoor")]
    UnlockSecurityDoor,

    #[serde(rename = "UpdateCustomSubObjective")]
    UpdateCustomSubObjective,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ObjectiveType {
    Enum(DimensionObjectiveType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DimensionObjectiveType {
    Ignore,

    Independent,

    #[serde(rename = "LinkedToLayer")]
    LinkedToLayer,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticEnemySpawningInZoneElement {
    pub difficulty: Option<Difficulty>,

    pub distribution: Option<StaticEnemySpawningInZoneDistribution>,

    pub distribution_value: Option<f64>,

    pub group_type: Option<GroupTypeUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Difficulty {
    Enum(EEnemyRoleDifficulty),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyRoleDifficulty {
    Biss,

    Boss,

    Buss,

    Easy,

    Hard,

    Medium,

    #[serde(rename = "MegaBoss")]
    MegaBoss,

    #[serde(rename = "MiniBoss")]
    MiniBoss,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StaticEnemySpawningInZoneDistribution {
    Enum(EEnemyZoneDistribution),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyZoneDistribution {
    #[serde(rename = "Force_One")]
    ForceOne,

    None,

    #[serde(rename = "Rel_Value")]
    RelValue,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GroupTypeUnion {
    Enum(EEnemyGroupType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyGroupType {
    Awake,

    Detect,

    Hibernate,

    Hunter,

    Patrol,

    #[serde(rename = "PureDetect")]
    PureDetect,

    #[serde(rename = "PureSneak")]
    PureSneak,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StaticGroundSpawnersInZone {
    Enum(EZoneDistributionAmount),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EZoneDistributionAmount {
    #[serde(rename = "Alot_30")]
    Alot30,

    #[serde(rename = "Few_5")]
    Few5,

    #[serde(rename = "Many_20")]
    Many20,

    #[serde(rename = "None_0")]
    None0,

    #[serde(rename = "Pair_2")]
    Pair2,

    #[serde(rename = "Some_10")]
    Some10,

    #[serde(rename = "SomeMore_15")]
    SomeMore15,

    #[serde(rename = "Tons_50")]
    Tons50,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticTerminalPlacementElement {
    pub area_seed_offset: Option<i64>,

    pub local_log_files: Option<Vec<StaticTerminalPlacementLocalLogFile>>,

    pub marker_seed_offset: Option<i64>,

    pub placement_weights: Option<StaticTerminalPlacementZonePlacementWeights>,

    pub starting_state_data: Option<StaticTerminalPlacementTerminalStartStateData>,

    pub unique_commands: Option<Vec<StaticTerminalPlacementUniqueCommand>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticTerminalPlacementLocalLogFile {
    pub attached_audio_byte_size: Option<i64>,

    pub attached_audio_file: Option<i64>,

    pub file_content: Option<PurpleCustomSubObjective>,

    pub file_content_original_language: Option<FileContentOriginalLanguage>,

    pub file_name: Option<String>,

    pub player_dialog_to_trigger_after_audio: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FileContentOriginalLanguage {
    Enum(Language),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "Chinese_Simplified")]
    ChineseSimplified,

    #[serde(rename = "Chinese_Traditional")]
    ChineseTraditional,

    English,

    French,

    German,

    Italian,

    Japanese,

    Korean,

    Polish,

    #[serde(rename = "Portuguese_Brazil")]
    PortugueseBrazil,

    Russian,

    Spanish,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticTerminalPlacementZonePlacementWeights {
    pub end: Option<f64>,

    pub middle: Option<f64>,

    pub start: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticTerminalPlacementTerminalStartStateData {
    pub audio_event_enter: Option<i64>,

    pub audio_event_exit: Option<i64>,

    pub custom_info_text: Option<PurpleCustomSubObjective>,

    pub generate_password: Option<bool>,

    pub keep_showing_local_log_count: Option<bool>,

    pub password: Option<String>,

    pub password_hint_text: Option<String>,

    pub password_part_count: Option<i64>,

    pub password_protected: Option<bool>,

    pub show_password_length: Option<bool>,

    pub show_password_part_positions: Option<bool>,

    pub starting_state: Option<StartingState>,

    pub terminal_zone_selection_datas: Option<Vec<Vec<TerminalZoneSelectionDatumElement>>>,

    pub use_custom_info_text: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartingState {
    Enum(TermState),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum TermState {
    #[serde(rename = "AskToPlayLogAudio")]
    AskToPlayLogAudio,

    #[serde(rename = "AudioLoopError")]
    AudioLoopError,

    Awake,

    #[serde(rename = "CodePuzzle")]
    CodePuzzle,

    #[serde(rename = "DataMining")]
    DataMining,

    #[serde(rename = "DoPlayAudioFile")]
    DoPlayAudioFile,

    #[serde(rename = "EnterPassword")]
    EnterPassword,

    Hacked,

    #[serde(rename = "InputTest")]
    InputTest,

    #[serde(rename = "PasswordProtected")]
    PasswordProtected,

    Ping,

    #[serde(rename = "PlayerInteracting")]
    PlayerInteracting,

    #[serde(rename = "ReactorError")]
    ReactorError,

    Sleeping,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalZoneSelectionDatumElement {
    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub seed_type: Option<SeedType>,

    pub static_seed: Option<i64>,

    pub terminal_index: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SeedType {
    Enum(ESeedType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ESeedType {
    #[serde(rename = "BuildSeed")]
    BuildSeed,

    None,

    #[serde(rename = "SessionSeed")]
    SessionSeed,

    #[serde(rename = "StaticSeed")]
    StaticSeed,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticTerminalPlacementUniqueCommand {
    pub command: Option<String>,

    pub command_desc: Option<PurpleCustomSubObjective>,

    pub command_events: Option<Vec<DimensionDataEventsOnBossDeath>>,

    pub post_command_outputs: Option<Vec<PurpleTerminalOutput>>,

    pub special_command_rule: Option<AlCommandRule>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleTerminalOutput {
    pub line_type: Option<LineType>,

    pub output: Option<PurpleCustomSubObjective>,

    pub time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LineType {
    Enum(TerminalLineType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum TerminalLineType {
    Fail,

    Normal,

    #[serde(rename = "ProgressWait")]
    ProgressWait,

    #[serde(rename = "SpinningWaitDone")]
    SpinningWaitDone,

    #[serde(rename = "SpinningWaitNoDone")]
    SpinningWaitNoDone,

    Warning,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEffectNodeDataBlock {
    pub blocks: Option<Vec<EffectNodeDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectNodeDataBlock {
    pub effect_to_spawn: Option<EffectNodePrefabData>,

    pub play_all: Option<Vec<EffectNodeListData>>,

    pub play_one: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectNodePrefabData {
    pub attach_to_root: Option<bool>,

    pub prefab_pointer: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectNodeListData {
    pub chance: Option<f64>,

    pub effect_node: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyBalancingDataBlock {
    pub blocks: Option<Vec<EnemyBalancingDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyBalancingDataBlock {
    pub allow_damge_bonus_from_behind: Option<bool>,

    pub can_be_pushed: Option<bool>,

    pub enemy_collision_minimum_move_speed_modifier: Option<f64>,

    pub enemy_collision_player_movement_reduction: Option<f64>,

    pub enemy_collision_radius: Option<f64>,

    pub forbid_twitch_hit: Option<bool>,

    pub glue_fade_out_time: Option<f64>,

    pub glue_tolerance: Option<f64>,

    pub health: Option<HealthData>,

    pub melee_attack_damage: Option<f64>,

    pub melee_attack_damage_check_radius: Option<f64>,

    pub tag_time: Option<f64>,

    pub tentacle_attack_damage: Option<f64>,

    pub tentacle_attack_damage_radius_if_no_tunnel_check: Option<f64>,

    pub use_tentacle_tunnel_check: Option<bool>,

    pub use_visibility_raycast_during_tentacle_attack: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthData {
    pub armor_damage_multi: Option<f64>,

    pub bodypart_health: Option<f64>,

    pub damage_until_hitreact: Option<f64>,

    pub health_max: Option<f64>,

    pub hitreact_on_limb_destruction: Option<HitreactOnLimbDestruction>,

    pub try_force_hitreact_on_limb_destruction: Option<bool>,

    pub weakspot_damage_multi: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum HitreactOnLimbDestruction {
    Enum(EsHitreactType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EsHitreactType {
    Heavy,

    #[serde(rename = "InstantRagdollDeath")]
    InstantRagdollDeath,

    Light,

    Micro,

    None,

    #[serde(rename = "ToDeath")]
    ToDeath,

    Unspecified,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyBehaviorDataBlock {
    pub blocks: Option<Vec<EnemyBehaviorDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyBehaviorDataBlock {
    pub chance_to_propagate_on_death: Option<f64>,

    pub chance_to_propagate_on_detection: Option<f64>,

    pub chance_to_propagate_to_mastermind: Option<f64>,

    pub closest_target_distance: Option<PurpleMinMaxValue>,

    pub disable_target_preference_weight: Option<bool>,

    #[serde(rename = "DistanceToStrafeInTargetsLOS")]
    pub distance_to_strafe_in_targets_los: Option<PurpleMinMaxValue>,

    #[serde(rename = "DistanceToStrafeWithoutLOS")]
    pub distance_to_strafe_without_los: Option<PurpleMinMaxValue>,

    #[serde(rename = "DistanceToStrafeWithoutTargetsLOS")]
    pub distance_to_strafe_without_targets_los: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_Acceleration")]
    pub flyer_acceleration: Option<f64>,

    #[serde(rename = "Flyer_AttackChargeDuration")]
    pub flyer_attack_charge_duration: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_AttackDuration")]
    pub flyer_attack_duration: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_BackToMovementAfterAttack")]
    pub flyer_back_to_movement_after_attack: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_BossEnemyID")]
    pub flyer_boss_enemy_id: Option<i64>,

    #[serde(rename = "Flyer_BossSpawnWaveID")]
    pub flyer_boss_spawn_wave_id: Option<i64>,

    #[serde(rename = "Flyer_DeathDealy")]
    pub flyer_death_dealy: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_Deceleration")]
    pub flyer_deceleration: Option<f64>,

    #[serde(rename = "Flyer_EstimatedDistanceFromBoss")]
    pub flyer_estimated_distance_from_boss: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_HeavyDamageForce")]
    pub flyer_heavy_damage_force: Option<f64>,

    #[serde(rename = "Flyer_HibernateRemap")]
    pub flyer_hibernate_remap: Option<FlyerHibernateRemap>,

    #[serde(rename = "Flyer_LightDamageForce")]
    pub flyer_light_damage_force: Option<f64>,

    #[serde(rename = "Flyer_PathingNodeMargin")]
    pub flyer_pathing_node_margin: Option<i64>,

    #[serde(rename = "Flyer_PitchThreshold")]
    pub flyer_pitch_threshold: Option<f64>,

    #[serde(rename = "Flyer_RecoilForce")]
    pub flyer_recoil_force: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_RecoilTorque")]
    pub flyer_recoil_torque: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_RollThreshold")]
    pub flyer_roll_threshold: Option<f64>,

    #[serde(rename = "Flyer_SpawnOutOfBossStateDuration")]
    pub flyer_spawn_out_of_boss_state_duration: Option<PurpleMinMaxValue>,

    #[serde(rename = "Flyer_SpeedMultipler")]
    pub flyer_speed_multipler: Option<f64>,

    #[serde(rename = "Flyer_YawThreshold")]
    pub flyer_yaw_threshold: Option<f64>,

    pub how_often_to_strafe: Option<PurpleMinMaxValue>,

    pub ignore_bots_targeting: Option<bool>,

    pub is_flyer: Option<bool>,

    pub melee_attack_distance: Option<PurpleMinMaxValue>,

    pub move_closer_delay_after_attack: Option<PurpleMinMaxValue>,

    pub move_towards_target_duration: Option<PurpleMinMaxValue>,

    pub pathing_target_distance: Option<PurpleMinMaxValue>,

    pub pathing_target_offset_y: Option<PurpleMinMaxValue>,

    pub propagation_distance: Option<f64>,

    pub ranged_attack_distance: Option<PurpleMinMaxValue>,

    pub rush_target_distance: Option<f64>,

    pub scream_state_delay: Option<PurpleMinMaxValue>,

    pub start_walk_duration: Option<PurpleMinMaxValue>,

    pub start_walk_timeout_until_next: Option<PurpleMinMaxValue>,

    pub strafe_distance: Option<PurpleMinMaxValue>,

    pub strafe_vertical_distance: Option<PurpleMinMaxValue>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleMinMaxValue {
    pub max: Option<f64>,

    pub min: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FlyerHibernateRemap {
    Enum(AgentMode),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AgentMode {
    Agressive,

    Hibernate,

    Off,

    Patrolling,

    Scout,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyDataBlock {
    pub blocks: Option<Vec<EnemyDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyDataBlock {
    #[serde(rename = "AI_Abilities")]
    pub ai_abilities: Option<Vec<AbilityData>>,

    pub arena_dimensions: Option<Vec<i64>>,

    pub asset_bundle: Option<BundleName>,

    pub balancing_data_id: Option<i64>,

    pub base_prefabs: Option<Vec<String>>,

    pub behavior_data_id: Option<i64>,

    pub bundle_shard: Option<Shard>,

    pub detection_data_id: Option<i64>,

    pub enemy_spotted_dialog_id: Option<i64>,

    pub enemy_type: Option<EnemyTypeUnion>,

    pub internal_material: Option<InternalMaterial>,

    #[serde(rename = "isCoccoon")]
    pub is_coccoon: Option<bool>,

    pub linked_slave_models: Option<Vec<LinkedSlaveModelData>>,

    pub model_datas: Option<Vec<ModelData>>,

    pub movement_data_id: Option<i64>,

    #[serde(rename = "SFXDataId")]
    pub sfx_data_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AbilityData {
    pub ability_conditions: Option<Vec<AbilityConditionElement>>,

    pub ability_prefab: Option<String>,

    pub ability_type: Option<AbilityType>,

    pub cooldown: Option<f64>,

    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AbilityConditionElement {
    Enum(AbilityCondition),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AbilityCondition {
    #[serde(rename = "BossPhase0")]
    BossPhase0,

    #[serde(rename = "BossPhase1")]
    BossPhase1,

    #[serde(rename = "BossPhase2")]
    BossPhase2,

    #[serde(rename = "BossPhase3")]
    BossPhase3,

    #[serde(rename = "BossPhase4")]
    BossPhase4,

    #[serde(rename = "BossPhase5")]
    BossPhase5,

    #[serde(rename = "BossStateCombat")]
    BossStateCombat,

    #[serde(rename = "BossStateIntro")]
    BossStateIntro,

    #[serde(rename = "BossStateRage")]
    BossStateRage,

    #[serde(rename = "BossStateSpawn")]
    BossStateSpawn,

    None,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AbilityType {
    Enum(AgentAbility),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum AgentAbility {
    Alarm,

    Defensive,

    Detection,

    #[serde(rename = "DoorBreaker")]
    DoorBreaker,

    #[serde(rename = "GroupEnhance")]
    GroupEnhance,

    Healing,

    Melee,

    None,

    Ranged,

    #[serde(rename = "SpawnChildren")]
    SpawnChildren,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnemyTypeUnion {
    Enum(EEnemyType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyType {
    Boss,

    #[serde(rename = "MiniBoss")]
    MiniBoss,

    Special,

    Standard,

    Weakling,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InternalMaterial {
    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LinkedSlaveModelData {
    pub animation_set: Option<InternalMaterial>,

    pub arm_scale: Option<LinkedSlaveModelVector3>,

    pub chest_scale: Option<LinkedSlaveModelVector3>,

    pub head_scale: Option<LinkedSlaveModelVector3>,

    pub leg_scale: Option<LinkedSlaveModelVector3>,

    pub link_bone: Option<String>,

    pub model_customization: Option<String>,

    pub model_file: Option<String>,

    pub neck_scale: Option<LinkedSlaveModelVector3>,

    pub position_offset: Option<LinkedSlaveModelVector3>,

    pub rotation_offset: Option<LinkedSlaveModelVector3>,

    pub size_range: Option<LinkedSlaveModelVector2>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedSlaveModelVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct LinkedSlaveModelVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModelData {
    pub arm_scale: Option<LinkedSlaveModelVector3>,

    pub chest_scale: Option<LinkedSlaveModelVector3>,

    pub head_scale: Option<LinkedSlaveModelVector3>,

    pub leg_scale: Option<LinkedSlaveModelVector3>,

    pub link_bone: Option<String>,

    pub model_customization: Option<String>,

    pub model_file: Option<String>,

    pub neck_scale: Option<LinkedSlaveModelVector3>,

    pub position_offset: Option<LinkedSlaveModelVector3>,

    pub rotation_offset: Option<LinkedSlaveModelVector3>,

    pub size_range: Option<LinkedSlaveModelVector2>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyDetectionDataBlock {
    pub blocks: Option<Vec<EnemyDetectionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnemyDetectionDataBlock {
    pub detection_buildup_speed: Option<f64>,

    pub detection_cooldown_speed: Option<f64>,

    pub detection_node_distance_max: Option<f64>,

    pub movement_detection_distance: Option<f64>,

    pub weapon_detection_distance_max: Option<f64>,

    pub weapon_detection_distance_min: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyGroupDataBlock {
    pub blocks: Option<Vec<EnemyGroupDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyGroupDataBlock {
    pub difficulty: Option<Difficulty>,

    pub max_score: Option<f64>,

    pub relative_weight: Option<f64>,

    pub roles: Option<Vec<EnemyGroupCompositionData>>,

    pub score_in_area_padding_multi: Option<f64>,

    pub spawn_placement_type: Option<SpawnPlacementType>,

    #[serde(rename = "Type")]
    pub enemy_group_data_block_type: Option<GroupTypeUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyGroupCompositionData {
    pub distribution: Option<RoleDistribution>,

    pub role: Option<Role>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RoleDistribution {
    Enum(EEnemyRoleDistribution),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyRoleDistribution {
    #[serde(rename = "Force_One")]
    ForceOne,

    None,

    #[serde(rename = "Rel_05")]
    Rel05,

    #[serde(rename = "Rel_10")]
    Rel10,

    #[serde(rename = "Rel_100")]
    Rel100,

    #[serde(rename = "Rel_15")]
    Rel15,

    #[serde(rename = "Rel_25")]
    Rel25,

    #[serde(rename = "Rel_50")]
    Rel50,

    #[serde(rename = "Rel_75")]
    Rel75,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Role {
    Enum(EEnemyRole),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyRole {
    #[serde(rename = "BirtherChild")]
    BirtherChild,

    Boss,

    Hunter,

    Lurker,

    Melee,

    #[serde(rename = "MiniBoss")]
    MiniBoss,

    Patroller,

    #[serde(rename = "PureSneak")]
    PureSneak,

    Ranged,

    Scout,

    Tank,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpawnPlacementType {
    Enum(ESpawnPlacementType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ESpawnPlacementType {
    #[serde(rename = "Align_0")]
    Align0,

    #[serde(rename = "Align_1")]
    Align1,

    #[serde(rename = "Align_2")]
    Align2,

    #[serde(rename = "Align_3")]
    Align3,

    #[serde(rename = "Align_4")]
    Align4,

    #[serde(rename = "Align_5")]
    Align5,

    #[serde(rename = "CycleAllAligns")]
    CycleAllAligns,

    Default,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyMovementDataBlock {
    pub blocks: Option<Vec<EnemyMovementDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyMovementDataBlock {
    pub allow_climb_down_ladders: Option<bool>,

    pub allow_movment_blend_out_of_path_move: Option<bool>,

    pub animation_controllers: Option<InternalMaterial>,

    pub blend_into_attack_anim: Option<f64>,

    pub blend_into_scream_anim: Option<f64>,

    pub force_disable_animator: Option<bool>,

    pub global_anim_speed_multi: Option<f64>,

    pub locomotion_dead: Option<Locomotion>,

    pub locomotion_hit_react: Option<Locomotion>,

    pub locomotion_path_move: Option<Locomotion>,

    pub locomotion_scream: Option<Locomotion>,

    pub locomotion_shooter_attack: Option<Locomotion>,

    pub move_slower_when_in_player_room: Option<bool>,

    pub move_slower_within_target_distance: Option<f64>,

    pub path_move_anim_damp_time: Option<f64>,

    pub path_move_anim_speed_multi: Option<f64>,

    pub rotation_lerp: Option<f64>,

    pub rotation_lerp_when_not_moving: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Locomotion {
    Enum(EsStateEnum),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EsStateEnum {
    #[serde(rename = "BirtherGiveBirth")]
    BirtherGiveBirth,

    #[serde(rename = "ClimbLadder")]
    ClimbLadder,

    Dead,

    #[serde(rename = "DeadFlyer")]
    DeadFlyer,

    #[serde(rename = "DeadSquidBoss")]
    DeadSquidBoss,

    #[serde(rename = "FloaterFly")]
    FloaterFly,

    #[serde(rename = "FloaterHitReact")]
    FloaterHitReact,

    Hibernate,

    #[serde(rename = "HibernateWakeUp")]
    HibernateWakeUp,

    #[serde(rename = "HitReactFlyer")]
    HitReactFlyer,

    Hitreact,

    Jump,

    #[serde(rename = "JumpDissolve")]
    JumpDissolve,

    Knockdown,

    #[serde(rename = "KnockdownRecover")]
    KnockdownRecover,

    #[serde(rename = "LiquidSnake")]
    LiquidSnake,

    None,

    #[serde(rename = "PathMove")]
    PathMove,

    #[serde(rename = "PathMoveFlyer")]
    PathMoveFlyer,

    #[serde(rename = "ScoutDetection")]
    ScoutDetection,

    #[serde(rename = "ScoutScream")]
    ScoutScream,

    Scream,

    #[serde(rename = "ScreamFlyer")]
    ScreamFlyer,

    #[serde(rename = "ShooterAttack")]
    ShooterAttack,

    #[serde(rename = "ShooterAttackFlyer")]
    ShooterAttackFlyer,

    #[serde(rename = "ShortcutJump")]
    ShortcutJump,

    #[serde(rename = "StandStill")]
    StandStill,

    #[serde(rename = "StrikerAttack")]
    StrikerAttack,

    #[serde(rename = "StrikerMelee")]
    StrikerMelee,

    #[serde(rename = "StuckInGlue")]
    StuckInGlue,

    #[serde(rename = "TankAttack")]
    TankAttack,

    #[serde(rename = "TankMultiTargetAttack")]
    TankMultiTargetAttack,

    #[serde(rename = "TentacleDragMove")]
    TentacleDragMove,

    #[serde(rename = "TriggerFogSphere")]
    TriggerFogSphere,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemyPopulationDataBlock {
    pub blocks: Option<Vec<EnemyPopulationDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyPopulationDataBlock {
    pub role_datas: Option<Vec<EnemyRoleData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyRoleData {
    pub cost: Option<f64>,

    pub difficulty: Option<Difficulty>,

    pub enemy: Option<i64>,

    pub role: Option<Role>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnemySfxDataBlock {
    pub blocks: Option<Vec<EnemySfxDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnemySfxDataBlock {
    pub hibernate_idle_interval: Option<TentacledVector2>,

    #[serde(rename = "SFX_ID_attackFire")]
    pub sfx_id_attack_fire: Option<i64>,

    #[serde(rename = "SFX_ID_attackWindUp")]
    pub sfx_id_attack_wind_up: Option<i64>,

    #[serde(rename = "SFX_ID_attackWindUp_NotLocalTarget")]
    pub sfx_id_attack_wind_up_not_local_target: Option<i64>,

    #[serde(rename = "SFX_ID_BigTentacleEnd")]
    pub sfx_id_big_tentacle_end: Option<i64>,

    #[serde(rename = "SFX_ID_BigTentacleStart")]
    pub sfx_id_big_tentacle_start: Option<i64>,

    #[serde(rename = "SFX_ID_BigTentacleTipLoop")]
    pub sfx_id_big_tentacle_tip_loop: Option<i64>,

    #[serde(rename = "SFX_ID_climbLadder")]
    pub sfx_id_climb_ladder: Option<i64>,

    #[serde(rename = "SFX_ID_die")]
    pub sfx_id_die: Option<i64>,

    #[serde(rename = "SFX_ID_heartbeatPulse")]
    pub sfx_id_heartbeat_pulse: Option<i64>,

    #[serde(rename = "SFX_ID_hibernateDetectionStart")]
    pub sfx_id_hibernate_detection_start: Option<i64>,

    #[serde(rename = "SFX_ID_hibernateDie")]
    pub sfx_id_hibernate_die: Option<i64>,

    #[serde(rename = "SFX_ID_hibernateIdle")]
    pub sfx_id_hibernate_idle: Option<i64>,

    #[serde(rename = "SFX_ID_hibernateWakeUp")]
    pub sfx_id_hibernate_wake_up: Option<i64>,

    #[serde(rename = "SFX_ID_hurtBig")]
    pub sfx_id_hurt_big: Option<i64>,

    #[serde(rename = "SFX_ID_hurtSmall")]
    pub sfx_id_hurt_small: Option<i64>,

    #[serde(rename = "SFX_ID_JumpInAir")]
    pub sfx_id_jump_in_air: Option<i64>,

    #[serde(rename = "SFX_ID_JumpLand")]
    pub sfx_id_jump_land: Option<i64>,

    #[serde(rename = "SFX_ID_JumpStart")]
    pub sfx_id_jump_start: Option<i64>,

    #[serde(rename = "SFX_ID_releaseFromGlue")]
    pub sfx_id_release_from_glue: Option<i64>,

    #[serde(rename = "SFX_ID_run")]
    pub sfx_id_run: Option<i64>,

    #[serde(rename = "SFX_ID_scream")]
    pub sfx_id_scream: Option<i64>,

    #[serde(rename = "SFX_ID_stuckInGlue")]
    pub sfx_id_stuck_in_glue: Option<i64>,

    #[serde(rename = "SFX_ID_walk")]
    pub sfx_id_walk: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEnvironmentFeedbackDataBlock {
    pub blocks: Option<Vec<EnvironmentFeedbackDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentFeedbackDataBlock {
    pub audio_data: Option<Vec<PurpleFeedbackAudioCompData>>,

    pub effect_data: Option<Vec<PurpleFeedbackEffectCompData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleFeedbackAudioCompData {
    pub audio_event: Option<String>,

    pub delay: Option<f64>,

    pub enabled: Option<bool>,

    pub material: Option<InternalMaterial>,

    pub net_mode: Option<NetMode>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetMode {
    Enum(NetworkMode),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum NetworkMode {
    Both,

    Local,

    Sync,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleFeedbackEffectCompData {
    pub attach_to_root: Option<bool>,

    pub chance: Option<f64>,

    pub delay: Option<f64>,

    pub do_no_kill: Option<bool>,

    pub effect_prefab: Option<String>,

    pub enabled: Option<bool>,

    pub material: Option<InternalMaterial>,

    pub net_mode: Option<NetMode>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEventSequenceActionDataBlock {
    pub blocks: Option<Vec<EventSequenceActionDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventSequenceActionDataBlock {
    pub enemy_wave: Option<EsaEnemyWave>,

    pub fog_settings: Option<EsaFogSettings>,

    pub lights: Option<EsaLights>,

    pub post_effect: Option<EsaPostEffect>,

    pub sound: Option<EsaSound>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EsaEnemyWave {
    pub on: Option<bool>,

    pub value_a: Option<f64>,

    pub value_b: Option<f64>,

    pub value_c: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EsaFogSettings {
    pub on: Option<bool>,

    pub value_a: Option<f64>,

    pub value_b: Option<String>,

    pub value_c: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EsaLights {
    #[serde(rename = "LightSettingsID")]
    pub light_settings_id: Option<i64>,

    pub on: Option<bool>,

    pub scope: Option<Scope>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Scope {
    Enum(EesaLightsEffectScope),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EesaLightsEffectScope {
    #[serde(rename = "CurrentArea")]
    CurrentArea,

    #[serde(rename = "CurrentZone")]
    CurrentZone,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EsaPostEffect {
    pub on: Option<bool>,

    pub value_a: Option<f64>,

    pub value_b: Option<f64>,

    pub value_c: Option<f64>,

    pub value_d: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EsaSound {
    pub on: Option<bool>,

    #[serde(rename = "SoundID")]
    pub sound_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfEventSequenceDataBlock {
    pub blocks: Option<Vec<EventSequenceDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventSequenceDataBlock {
    pub actions: Option<Vec<EventSequenceActionListComponent>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventSequenceActionListComponent {
    #[serde(rename = "ActionID")]
    pub action_id: Option<i64>,

    pub on: Option<bool>,

    pub time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfExpeditionBalanceDataBlock {
    pub blocks: Option<Vec<ExpeditionBalanceDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionBalanceDataBlock {
    pub air_per_zone: Option<f64>,

    pub air_per_zone_in_no_air: Option<f64>,

    pub artifacts_per_layer: Option<i64>,

    pub artifacts_per_segment: Option<i64>,

    pub chance_to_put_artifact_in_resource_container: Option<f64>,

    pub chance_to_put_commodity_in_resource_container: Option<f64>,

    pub chance_to_re_use_resource_container: Option<f64>,

    pub chance_to_spawn_commodity_large_pack: Option<f64>,

    pub chance_to_spawn_commodity_medium_pack: Option<f64>,

    pub commodity_value_per_zone: Option<f64>,

    pub disinfection_per_zone: Option<f64>,

    pub empty_secure_resource_containers_per_zone: Option<f64>,

    pub empty_weak_resource_containers_per_zone: Option<f64>,

    pub enemy_patrol_groups_per_zone: Option<i64>,

    pub enemy_population_per_zone: Option<f64>,

    pub glue_volume_for_door_glue_max_state: Option<f64>,

    pub glue_volume_to_door_health_conversion: Option<f64>,

    pub health_per_zone: Option<f64>,

    pub loot_per_zone: Option<f64>,

    pub max_packs_per_resource_container: Option<i64>,

    pub parasite_nests: Option<StaticEnemyData>,

    pub resource_pack_sizes: Option<Vec<f64>>,

    pub static_enemies_max_huge_area: Option<i64>,

    pub static_enemies_max_large_area: Option<i64>,

    pub static_enemies_max_medium_area: Option<i64>,

    pub static_enemies_max_per_zone: Option<i64>,

    pub static_enemies_max_small_area: Option<i64>,

    pub tentacle_traps: Option<StaticEnemyData>,

    pub terminals_per_zone: Option<f64>,

    pub tool_ammo_per_zone: Option<f64>,

    pub voxel_coverage_area_multiplier: Option<f64>,

    pub voxel_coverage_area_scoring_random_multiplier: Option<f64>,

    #[serde(rename = "WeakDoor4x4Health")]
    pub weak_door4_x4_health: Option<f64>,

    #[serde(rename = "WeakDoor8x4Health")]
    pub weak_door8_x4_health: Option<f64>,

    pub weak_door_chance_lock_weight_hackable_lock: Option<f64>,

    pub weak_door_chance_lock_weight_melee_lock: Option<f64>,

    pub weak_door_chance_lock_weight_no_lock: Option<f64>,

    pub weak_door_lock_health: Option<f64>,

    pub weak_door_open_chance_for_wall_remover_used: Option<f64>,

    pub weak_door_unlocked_chance_for_open: Option<f64>,

    pub weak_resource_container_with_pack_chance_for_locked: Option<f64>,

    pub weapon_ammo_per_zone: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticEnemyData {
    pub attack_damage: Option<f64>,

    pub health: Option<f64>,

    pub max_huge_area: Option<i64>,

    pub max_large_area: Option<i64>,

    pub max_medium_area: Option<i64>,

    pub max_per_zone: Option<i64>,

    pub max_small_area: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfExtractionEventDataBlock {
    pub blocks: Option<Vec<ExtractionEventDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtractionEventDataBlock {
    pub events_to_trigger: Option<Vec<EventsToTrigger>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventsToTrigger {
    Enum(EExtractionEventType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EExtractionEventType {
    #[serde(rename = "ElevatorBioscan")]
    ElevatorBioscan,

    #[serde(rename = "EnemyWaveSpawn")]
    EnemyWaveSpawn,

    #[serde(rename = "LightsOff")]
    LightsOff,

    #[serde(rename = "LightsOn")]
    LightsOn,

    #[serde(rename = "RemoveAir")]
    RemoveAir,

    #[serde(rename = "ThickFog")]
    ThickFog,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfFeedbackDataBlock {
    pub blocks: Option<Vec<FeedbackDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackDataBlock {
    pub audio_data: Option<Vec<FluffyFeedbackAudioCompData>>,

    pub effect_data: Option<Vec<FluffyFeedbackEffectCompData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyFeedbackAudioCompData {
    pub audio_event: Option<String>,

    pub delay: Option<f64>,

    pub enabled: Option<bool>,

    pub material: Option<InternalMaterial>,

    pub net_mode: Option<NetMode>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyFeedbackEffectCompData {
    pub attach_to_root: Option<bool>,

    pub chance: Option<f64>,

    pub delay: Option<f64>,

    pub do_no_kill: Option<bool>,

    pub effect_prefab: Option<String>,

    pub enabled: Option<bool>,

    pub material: Option<InternalMaterial>,

    pub net_mode: Option<NetMode>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfFlashlightSettingsDataBlock {
    pub blocks: Option<Vec<FlashlightSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashlightSettingsDataBlock {
    pub angle: Option<f64>,

    pub color: Option<FluffyColor>,

    pub cookie: Option<String>,

    pub intensity: Option<f64>,

    pub range: Option<f64>,

    pub startup_shard: Option<Shard>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfFogScenarioDataBlock {
    pub blocks: Option<Vec<FogScenarioDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FogScenarioDataBlock {
    pub fog: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfFogSettingsDataBlock {
    pub blocks: Option<Vec<FogSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FogSettingsDataBlock {
    pub density_height_altitude: Option<f64>,

    pub density_height_max_boost: Option<f64>,

    pub density_height_range: Option<f64>,

    pub density_noise_direction: Option<FluffyVector3>,

    pub density_noise_scale: Option<f64>,

    pub density_noise_speed: Option<f64>,

    pub enabled: Option<bool>,

    pub fog_ambience: Option<f64>,

    pub fog_color: Option<TentacledColor>,

    pub fog_density: Option<f64>,

    pub infection: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGameplayTrailerDataBlock {
    pub blocks: Option<Vec<GameplayTrailerDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayTrailerDataBlock {
    pub actions: Option<Vec<GameplayTrailerDirectorAction>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameplayTrailerDirectorAction {
    pub action: Option<Action>,

    #[serde(rename = "GOPath")]
    pub go_path: Option<String>,

    pub position: Option<ActionVector3>,

    pub trigger: Option<ActionTrigger>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Action {
    Enum(GameplayTrailerActionName),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum GameplayTrailerActionName {
    Disable,

    Move,

    None,

    Rotate,
}

#[derive(Serialize, Deserialize)]
pub struct ActionVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionTrigger {
    Enum(GameplayTrailerActionTrigger),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum GameplayTrailerActionTrigger {
    #[serde(rename = "OnButtonA")]
    OnButtonA,

    #[serde(rename = "OnButtonB")]
    OnButtonB,

    #[serde(rename = "OnButtonX")]
    OnButtonX,

    #[serde(rename = "OnButtonY")]
    OnButtonY,

    #[serde(rename = "OnStart")]
    OnStart,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGameSetupDataBlock {
    pub blocks: Option<Vec<GameSetupDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameSetupDataBlock {
    pub rundown_id_to_load: Option<i64>,

    pub startup_screen_to_load: Option<StartupScreenToLoad>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartupScreenToLoad {
    Enum(EStartupScreenKey),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EStartupScreenKey {
    None,

    #[serde(rename = "StartupScreenData_1")]
    StartupScreenData1,

    #[serde(rename = "StartupScreenData_2")]
    StartupScreenData2,

    #[serde(rename = "StartupScreenData_3")]
    StartupScreenData3,

    #[serde(rename = "StartupScreenData_4")]
    StartupScreenData4,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearCategoryDataBlock {
    pub blocks: Option<Vec<GearCategoryDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearCategoryDataBlock {
    pub auto_archetype: Option<i64>,

    pub base_item: Option<i64>,

    pub burst_archetype: Option<i64>,

    pub description: Option<StickyDescription>,

    #[serde(rename = "FPSArmPoseName")]
    pub fps_arm_pose_name: Option<String>,

    #[serde(rename = "HUDIcon")]
    pub hud_icon: Option<String>,

    pub icon_rotation_offset: Option<f64>,

    pub icon_zoom_offset: Option<f64>,

    pub melee_archetype: Option<i64>,

    pub part_align_priority: Option<Vec<GearPartAlignPriority>>,

    pub public_name: Option<StickyDescription>,

    pub semi_archetype: Option<i64>,

    pub semi_burst_archetype: Option<i64>,

    pub third_person_fullbody_movement: Option<ThirdPersonFullbodyMovement>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StickyDescription {
    IndigoLocalizedText(IndigoLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndigoLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAlignPriority {
    pub align_type: Option<AlignType>,

    pub part_prio: Option<Vec<PartPrio>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlignType {
    Enum(EGearPartAlign),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EGearPartAlign {
    Flashlight,

    Front,

    #[serde(rename = "GroundPlacement")]
    GroundPlacement,

    #[serde(rename = "LeftHand")]
    LeftHand,

    Magazine,

    #[serde(rename = "MeleeHead")]
    MeleeHead,

    Muzzle,

    Receiver,

    #[serde(rename = "RightHand")]
    RightHand,

    #[serde(rename = "RotationPivot")]
    RotationPivot,

    #[serde(rename = "ShellEject")]
    ShellEject,

    Sight,

    #[serde(rename = "SightLook")]
    SightLook,

    #[serde(rename = "ToolDelivery")]
    ToolDelivery,

    #[serde(rename = "ToolDetection")]
    ToolDetection,

    #[serde(rename = "ToolGrip")]
    ToolGrip,

    #[serde(rename = "ToolPayload")]
    ToolPayload,

    #[serde(rename = "ToolScanning")]
    ToolScanning,

    #[serde(rename = "ToolScreen")]
    ToolScreen,

    #[serde(rename = "ToolTargeting")]
    ToolTargeting,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PartPrio {
    Enum(EGearComponent),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EGearComponent {
    #[serde(rename = "AudioSetting")]
    AudioSetting,

    #[serde(rename = "BaseItem")]
    BaseItem,

    Category,

    #[serde(rename = "DecalA")]
    DecalA,

    #[serde(rename = "DecalB")]
    DecalB,

    #[serde(rename = "FireMode")]
    FireMode,

    #[serde(rename = "FlashlightPart")]
    FlashlightPart,

    #[serde(rename = "FlashlightPartPerk")]
    FlashlightPartPerk,

    #[serde(rename = "FrontPart")]
    FrontPart,

    #[serde(rename = "FrontPartAttachmentA")]
    FrontPartAttachmentA,

    #[serde(rename = "FrontPartAttachmentB")]
    FrontPartAttachmentB,

    #[serde(rename = "FrontPartPerk")]
    FrontPartPerk,

    #[serde(rename = "ItemFPSSettings")]
    ItemFpsSettings,

    #[serde(rename = "MagPart")]
    MagPart,

    #[serde(rename = "MagPartPerk")]
    MagPartPerk,

    #[serde(rename = "MeleeHandlePart")]
    MeleeHandlePart,

    #[serde(rename = "MeleeHandlePartPerk")]
    MeleeHandlePartPerk,

    #[serde(rename = "MeleeHeadPart")]
    MeleeHeadPart,

    #[serde(rename = "MeleeHeadPartPerk")]
    MeleeHeadPartPerk,

    #[serde(rename = "MeleeNeckPart")]
    MeleeNeckPart,

    #[serde(rename = "MeleeNeckPartPerk")]
    MeleeNeckPartPerk,

    #[serde(rename = "MeleePommelPart")]
    MeleePommelPart,

    #[serde(rename = "MeleePommelPartPerk")]
    MeleePommelPartPerk,

    #[serde(rename = "MuzzleFlash")]
    MuzzleFlash,

    None,

    Palette,

    Pattern,

    #[serde(rename = "PlatformPerkAID")]
    PlatformPerkAid,

    #[serde(rename = "PlatformPerkBID")]
    PlatformPerkBid,

    #[serde(rename = "PlatformPerkCID")]
    PlatformPerkCid,

    #[serde(rename = "ReceiverPart")]
    ReceiverPart,

    #[serde(rename = "ReceiverPartAttachment")]
    ReceiverPartAttachment,

    #[serde(rename = "ReceiverPartPerk")]
    ReceiverPartPerk,

    #[serde(rename = "ShellCasing")]
    ShellCasing,

    #[serde(rename = "SightPart")]
    SightPart,

    #[serde(rename = "SightPartPerk")]
    SightPartPerk,

    #[serde(rename = "StockPart")]
    StockPart,

    #[serde(rename = "StockPartPerk")]
    StockPartPerk,

    #[serde(rename = "ToolDeliveryPart")]
    ToolDeliveryPart,

    #[serde(rename = "ToolDeliveryPartAttachment")]
    ToolDeliveryPartAttachment,

    #[serde(rename = "ToolDeliveryPartPerk")]
    ToolDeliveryPartPerk,

    #[serde(rename = "ToolDeliveryType")]
    ToolDeliveryType,

    #[serde(rename = "ToolGripPart")]
    ToolGripPart,

    #[serde(rename = "ToolGripPartPerk")]
    ToolGripPartPerk,

    #[serde(rename = "ToolMainPart")]
    ToolMainPart,

    #[serde(rename = "ToolMainPartAttachment")]
    ToolMainPartAttachment,

    #[serde(rename = "ToolMainPartPerk")]
    ToolMainPartPerk,

    #[serde(rename = "ToolPayloadPart")]
    ToolPayloadPart,

    #[serde(rename = "ToolPayloadPartPerk")]
    ToolPayloadPartPerk,

    #[serde(rename = "ToolPayloadType")]
    ToolPayloadType,

    #[serde(rename = "ToolScreenPart")]
    ToolScreenPart,

    #[serde(rename = "ToolScreenPartPerk")]
    ToolScreenPartPerk,

    #[serde(rename = "ToolTargetingPart")]
    ToolTargetingPart,

    #[serde(rename = "ToolTargetingPartPerk")]
    ToolTargetingPartPerk,

    #[serde(rename = "ToolTargetingType")]
    ToolTargetingType,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ThirdPersonFullbodyMovement {
    Enum(EFullbodyPlayerMovementSet),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EFullbodyPlayerMovementSet {
    #[serde(rename = "CarryHeavy")]
    CarryHeavy,

    Knife,

    Melee,

    Pistol,

    Rifle,

    Spear,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearCategoryFilterDataBlock {
    pub blocks: Option<Vec<GearCategoryFilterDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearCategoryFilterDataBlock {
    pub allowed_categories: Option<Vec<GearCategoryFilterData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearCategoryFilterData {
    #[serde(rename = "CategoryID")]
    pub category_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearDataBlock {
    pub blocks: Option<Vec<GearDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearDataBlock {
    pub allow_empty_grip_slot: Option<bool>,

    pub allow_empty_magazine_slot: Option<bool>,

    pub allow_empty_muzzle_slot: Option<bool>,

    pub allow_empty_sight_slot: Option<bool>,

    pub allow_empty_structual_mod_slot: Option<bool>,

    pub allow_empty_tool_slot: Option<bool>,

    pub allow_empty_visual_mod_slot: Option<bool>,

    pub archetypes_allowed: Option<Vec<GearArchetypeData>>,

    #[serde(rename = "baseItemID")]
    pub base_item_id: Option<i64>,

    #[serde(rename = "ClassAmmoCostFactor")]
    pub class_ammo_cost_factor: Option<f64>,

    pub drop_period_range: Option<Vec<GearDropPeriodData>>,

    pub grips_allowed: Option<Vec<ItemPartData>>,

    pub magazines_allowed: Option<Vec<ItemPartData>>,

    pub muzzles_allowed: Option<Vec<ItemPartData>>,

    pub official_for_release: Option<bool>,

    #[serde(rename = "PublicGearInfo")]
    pub public_gear_info: Option<String>,

    pub public_name: Option<String>,

    #[serde(rename = "showAmmoInGUI")]
    pub show_ammo_in_gui: Option<bool>,

    pub sights_allowed: Option<Vec<ItemPartData>>,

    pub structual_mods_allowed_barrel: Option<Vec<ItemPartData>>,

    pub structual_mods_allowed_grip: Option<Vec<ItemPartData>>,

    pub structual_mods_allowed_receiver: Option<Vec<ItemPartData>>,

    pub structual_mods_allowed_stock: Option<Vec<ItemPartData>>,

    pub tools_allowed: Option<Vec<ItemPartData>>,

    pub visual_mods_allowed: Option<Vec<ItemPartData>>,

    #[serde(rename = "weaponPersonalityID")]
    pub weapon_personality_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearArchetypeData {
    pub allowed: Option<bool>,

    #[serde(rename = "archetypeID")]
    pub archetype_id: Option<i64>,

    pub macro_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GearDropPeriodData {
    pub macro_text: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPartData {
    pub allowed: Option<bool>,

    #[serde(rename = "ID")]
    pub id: Option<i64>,

    pub macro_text: Option<String>,

    pub part_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearDecalDataBlock {
    pub blocks: Option<Vec<GearDecalDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearDecalDataBlock {
    pub asset_bundle_name: Option<BundleName>,

    pub asset_bundle_shard: Option<Shard>,

    pub decal: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearFlashlightPartDataBlock {
    pub blocks: Option<Vec<GearFlashlightPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearFlashlightPartDataBlock {
    pub aligns: Option<Vec<PurpleGearPartAlignData>>,

    pub general: Option<PurpleGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearFrontPartDataBlock {
    pub blocks: Option<Vec<GearFrontPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearFrontPartDataBlock {
    pub aligns: Option<Vec<FluffyGearPartAlignData>>,

    pub fire_sequence: Option<Vec<PurpleWeaponAnimSequenceItem>>,

    pub general: Option<FluffyGearPartGeneralData>,

    pub reload_sequence: Option<Vec<PurpleWeaponAnimSequenceItem>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleWeaponAnimSequenceItem {
    pub string_data: Option<String>,

    pub trigger_time: Option<f64>,

    #[serde(rename = "Type")]
    pub weapon_anim_sequence_item_type: Option<AimSequenceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearMagPartDataBlock {
    pub blocks: Option<Vec<GearMagPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearMagPartDataBlock {
    pub aligns: Option<Vec<TentacledGearPartAlignData>>,

    pub clip_size_multiplier: Option<f64>,

    pub drop_sound_type: Option<DropSoundType>,

    pub general: Option<TentacledGearPartGeneralData>,

    pub reload_left_hand_grip_anim: Option<String>,

    pub reload_time_multiplier: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropSoundType {
    Enum(MagazineDropSoundType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum MagazineDropSoundType {
    Large,

    None,

    Small,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearMeleeHandlePartDataBlock {
    pub blocks: Option<Vec<GearMeleeHandlePartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearMeleeHandlePartDataBlock {
    pub aligns: Option<Vec<StickyGearPartAlignData>>,

    pub general: Option<StickyGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StickyGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearMeleeHeadPartDataBlock {
    pub blocks: Option<Vec<GearMeleeHeadPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearMeleeHeadPartDataBlock {
    pub aligns: Option<Vec<IndigoGearPartAlignData>>,

    pub general: Option<IndigoGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndigoGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndigoGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearMeleeNeckPartDataBlock {
    pub blocks: Option<Vec<GearMeleeNeckPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearMeleeNeckPartDataBlock {
    pub aligns: Option<Vec<IndecentGearPartAlignData>>,

    pub general: Option<IndecentGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndecentGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndecentGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearMeleePommelPartDataBlock {
    pub blocks: Option<Vec<GearMeleePommelPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearMeleePommelPartDataBlock {
    pub aligns: Option<Vec<HilariousGearPartAlignData>>,

    pub general: Option<HilariousGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HilariousGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HilariousGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearPaletteDataBlock {
    pub blocks: Option<Vec<GearPaletteDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPaletteDataBlock {
    pub decal_a: Option<StickyColor>,

    pub decal_b: Option<StickyColor>,

    pub main: Option<StickyColor>,

    pub second: Option<StickyColor>,

    pub third: Option<StickyColor>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearPartAttachmentDataBlock {
    pub blocks: Option<Vec<GearPartAttachmentDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAttachmentDataBlock {
    pub aligns: Option<Vec<AmbitiousGearPartAlignData>>,

    pub attach_to_align: Option<AlignType>,

    pub general: Option<AmbitiousGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmbitiousGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AmbitiousGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearPatternDataBlock {
    pub blocks: Option<Vec<GearPatternDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPatternDataBlock {
    pub asset_bundle_name: Option<BundleName>,

    pub asset_bundle_shard: Option<Shard>,

    pub pattern: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearPerkDataBlock {
    pub blocks: Option<Vec<GearPerkDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPerkDataBlock {
    pub mod_damage: Option<GearPerkModifierData>,

    pub mod_firerate: Option<GearPerkModifierData>,

    pub mod_handling: Option<GearPerkModifierData>,

    pub mod_range: Option<GearPerkModifierData>,

    pub mod_recoil_reduction: Option<GearPerkModifierData>,

    pub mod_reload_time: Option<GearPerkModifierData>,

    pub perk_specials: Option<Vec<GearSpecialPerkData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPerkModifierData {
    pub active: Option<bool>,

    pub cost: Option<f64>,

    pub max: Option<f64>,

    pub min: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSpecialPerkData {
    #[serde(rename = "Type")]
    pub gear_special_perk_data_type: Option<PerkSpecialType>,

    pub type_value1: Option<f64>,

    pub type_value2: Option<f64>,

    pub type_value3: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PerkSpecialType {
    Enum(EGearPerkSpecialType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EGearPerkSpecialType {
    #[serde(rename = "FirstShotInMagMulti")]
    FirstShotInMagMulti,

    #[serde(rename = "HeadshotMulti")]
    HeadshotMulti,

    #[serde(rename = "LastShotInMagMulti")]
    LastShotInMagMulti,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearReceiverPartDataBlock {
    pub blocks: Option<Vec<GearReceiverPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearReceiverPartDataBlock {
    pub aligns: Option<Vec<CunningGearPartAlignData>>,

    pub fire_sequence: Option<Vec<FluffyWeaponAnimSequenceItem>>,

    pub general: Option<CunningGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CunningGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyWeaponAnimSequenceItem {
    pub string_data: Option<String>,

    pub trigger_time: Option<f64>,

    #[serde(rename = "Type")]
    pub weapon_anim_sequence_item_type: Option<AimSequenceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CunningGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearSightPartDataBlock {
    pub blocks: Option<Vec<GearSightPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSightPartDataBlock {
    pub aligns: Option<Vec<MagentaGearPartAlignData>>,

    pub general: Option<MagentaGearPartGeneralData>,

    pub sight_properties: Option<GearSightPartProperties>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MagentaGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MagentaGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearSightPartProperties {
    pub gear_fov_zoom: Option<i64>,

    pub world_fov_zoom: Option<i64>,

    pub zoom_sensitivity_modifier: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearStockPartDataBlock {
    pub blocks: Option<Vec<GearStockPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearStockPartDataBlock {
    pub aligns: Option<Vec<FriskyGearPartAlignData>>,

    pub fire_sequence: Option<Vec<TentacledWeaponAnimSequenceItem>>,

    pub general: Option<FriskyGearPartGeneralData>,

    pub reload_sequence: Option<Vec<TentacledWeaponAnimSequenceItem>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriskyGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledWeaponAnimSequenceItem {
    pub string_data: Option<String>,

    pub trigger_time: Option<f64>,

    #[serde(rename = "Type")]
    pub weapon_anim_sequence_item_type: Option<AimSequenceType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FriskyGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolDeliveryPartDataBlock {
    pub blocks: Option<Vec<GearToolDeliveryPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolDeliveryPartDataBlock {
    pub aligns: Option<Vec<MischievousGearPartAlignData>>,

    pub general_datas: Option<Vec<PurpleTypeAndGeneralData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MischievousGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleTypeAndGeneralData {
    pub data: Option<MischievousGearPartGeneralData>,

    #[serde(rename = "Type")]
    pub type_and_general_data_type: Option<GeneralDataType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MischievousGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GeneralDataType {
    Enum(EGearToolPartDeliveryType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EGearToolPartDeliveryType {
    #[serde(rename = "DirectPlacement")]
    DirectPlacement,

    #[serde(rename = "DirectPlacementWithRemoteTrigger")]
    DirectPlacementWithRemoteTrigger,

    Drone,

    #[serde(rename = "LongRangeBurst")]
    LongRangeBurst,

    #[serde(rename = "LongRangeSemi")]
    LongRangeSemi,

    #[serde(rename = "LongRangeSemiRemoteTrigger")]
    LongRangeSemiRemoteTrigger,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolGripPartDataBlock {
    pub blocks: Option<Vec<GearToolGripPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolGripPartDataBlock {
    pub aligns: Option<Vec<BraggadociousGearPartAlignData>>,

    pub general: Option<BraggadociousGearPartGeneralData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BraggadociousGearPartAlignData {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BraggadociousGearPartGeneralData {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolMainPartDataBlock {
    pub blocks: Option<Vec<GearToolMainPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolMainPartDataBlock {
    pub aligns: Option<Vec<GearPartAlignData1>>,

    pub general: Option<GearPartGeneralData1>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAlignData1 {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartGeneralData1 {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolPayloadPartDataBlock {
    pub blocks: Option<Vec<GearToolPayloadPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolPayloadPartDataBlock {
    pub aligns: Option<Vec<GearPartAlignData2>>,

    pub general_datas: Option<Vec<FluffyTypeAndGeneralData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAlignData2 {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyTypeAndGeneralData {
    pub data: Option<GearPartGeneralData2>,

    #[serde(rename = "Type")]
    pub type_and_general_data_type: Option<GeneralDataType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartGeneralData2 {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolScreenPartDataBlock {
    pub blocks: Option<Vec<GearToolScreenPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolScreenPartDataBlock {
    pub aligns: Option<Vec<GearPartAlignData3>>,

    pub general: Option<GearPartGeneralData3>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAlignData3 {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartGeneralData3 {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfGearToolTargetingPartDataBlock {
    pub blocks: Option<Vec<GearToolTargetingPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearToolTargetingPartDataBlock {
    pub aligns: Option<Vec<GearPartAlignData4>>,

    pub general_datas: Option<Vec<TentacledTypeAndGeneralData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartAlignData4 {
    pub align_name: Option<String>,

    pub align_type: Option<AlignType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledTypeAndGeneralData {
    pub data: Option<GearPartGeneralData4>,

    #[serde(rename = "Type")]
    pub type_and_general_data_type: Option<GeneralDataType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPartGeneralData4 {
    pub asset_bundle: Option<BundleName>,

    pub bundle_shard: Option<Shard>,

    pub children: Option<Vec<String>>,

    pub gear_category_filter: Option<i64>,

    pub left_hand_grip_anim: Option<String>,

    pub model: Option<String>,

    pub public_name: Option<String>,

    pub right_hand_grip_anim: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfItemDataBlock {
    pub blocks: Option<Vec<ItemDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDataBlock {
    #[serde(rename = "addSerialNumberToName")]
    pub add_serial_number_to_name: Option<bool>,

    pub aim_transition_time: Option<f64>,

    #[serde(rename = "audioEventEquip")]
    pub audio_event_equip: Option<String>,

    pub backpack_align: Option<String>,

    pub block_tool_ammo_refill: Option<bool>,

    #[serde(rename = "canMoveQuick")]
    pub can_move_quick: Option<bool>,

    pub class_ammo_cost_factor: Option<f64>,

    pub consumable_ammo_max: Option<i64>,

    pub consumable_ammo_min: Option<i64>,

    #[serde(rename = "crosshair")]
    pub crosshair: Option<CrosshairUnion>,

    pub dimension_warp_type: Option<DimensionWarpType>,

    pub equip_transition_time: Option<f64>,

    pub first_person_prefabs: Option<Vec<String>>,

    #[serde(rename = "FPSArmsAnim")]
    pub fps_arms_anim: Option<String>,

    #[serde(rename = "FPSSettings")]
    pub fps_settings: Option<i64>,

    #[serde(rename = "GUIShowAmmoClip")]
    pub gui_show_ammo_clip: Option<bool>,

    #[serde(rename = "GUIShowAmmoInfinite")]
    pub gui_show_ammo_infinite: Option<bool>,

    #[serde(rename = "GUIShowAmmoPack")]
    pub gui_show_ammo_pack: Option<bool>,

    #[serde(rename = "GUIShowAmmoTotalRel")]
    pub gui_show_ammo_total_rel: Option<bool>,

    #[serde(rename = "HUDIcon")]
    pub hud_icon: Option<String>,

    pub instance_prefabs: Option<Vec<String>>,

    #[serde(rename = "inventorySlot")]
    pub inventory_slot: Option<InventorySlotUnion>,

    pub left_hand_grip_align: Option<String>,

    pub left_hand_grip_anim: Option<String>,

    pub localized_name: Option<Name>,

    pub muzzle_align: Option<String>,

    pub pickup_prefabs: Option<Vec<String>>,

    #[serde(rename = "publicName")]
    pub public_name: Option<String>,

    #[serde(rename = "registerInTerminalSystem")]
    pub register_in_terminal_system: Option<bool>,

    pub right_hand_grip_align: Option<String>,

    pub right_hand_grip_anim: Option<String>,

    pub shard: Option<Shard>,

    pub show_crosshair_when_aiming: Option<bool>,

    pub sight_look_align: Option<String>,

    #[serde(rename = "terminalItemLongName")]
    pub terminal_item_long_name: Option<Name>,

    #[serde(rename = "terminalItemShortName")]
    pub terminal_item_short_name: Option<String>,

    pub third_person_fullbody_movement_set: Option<ThirdPersonFullbodyMovement>,

    pub third_person_prefabs: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CrosshairUnion {
    Enum(Crosshair),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum Crosshair {
    #[serde(rename = "AssaultRifle")]
    AssaultRifle,

    Circle,

    Keypad,

    Neutral,

    None,

    Shotgun,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionWarpType {
    Enum(ItemWarpType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ItemWarpType {
    Always,

    Never,

    #[serde(rename = "OnlyWarpToOtherDimension")]
    OnlyWarpToOtherDimension,

    #[serde(rename = "OnlyWarpToSpawnDimension")]
    OnlyWarpToSpawnDimension,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum InventorySlotUnion {
    Enum(InventorySlot),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum InventorySlot {
    Consumable,

    #[serde(rename = "ConsumableHeavy")]
    ConsumableHeavy,

    #[serde(rename = "GearClass")]
    GearClass,

    #[serde(rename = "GearMelee")]
    GearMelee,

    #[serde(rename = "GearSpecial")]
    GearSpecial,

    #[serde(rename = "GearStandard")]
    GearStandard,

    #[serde(rename = "HackingTool")]
    HackingTool,

    #[serde(rename = "InLevelCarry")]
    InLevelCarry,

    #[serde(rename = "InPocket")]
    InPocket,

    None,

    Pickup,

    #[serde(rename = "ResourcePack")]
    ResourcePack,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Name {
    Integer(i64),

    LocalizedNameLocalizedText(LocalizedNameLocalizedText),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LocalizedNameLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfItemFpsSettingsDataBlock {
    pub blocks: Option<Vec<ItemFpsSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemFpsSettingsDataBlock {
    pub allow_rot_to_aim_pos: Option<bool>,

    pub body_offset_local: Option<TentacledVector3>,

    pub body_rotation_offset_local: Option<TentacledVector3>,

    pub can_aim: Option<bool>,

    pub can_relax: Option<bool>,

    #[serde(rename = "ChargeCancelAnimation")]
    pub charge_cancel_animation: Option<i64>,

    pub crouch_tilt_angle: Option<f64>,

    pub custom_delay_until_relax: Option<f64>,

    #[serde(rename = "DofAim")]
    pub dof_aim: Option<PurpleDofSettingsData>,

    #[serde(rename = "DofDefault")]
    pub dof_default: Option<PurpleDofSettingsData>,

    #[serde(rename = "IdleAnimation")]
    pub idle_animation: Option<i64>,

    #[serde(rename = "ItemCameraFOVDefault")]
    pub item_camera_fov_default: Option<i64>,

    #[serde(rename = "ItemCameraFOVZoom")]
    pub item_camera_fov_zoom: Option<i64>,

    #[serde(rename = "JumpAnimation")]
    pub jump_animation: Option<i64>,

    #[serde(rename = "LandAnimation")]
    pub land_animation: Option<i64>,

    pub local_pos_hip: Option<TentacledVector3>,

    pub local_pos_relaxed: Option<TentacledVector3>,

    pub local_pos_zoom: Option<TentacledVector3>,

    pub local_rot_hip: Option<TentacledVector3>,

    pub local_rot_relaxed: Option<TentacledVector3>,

    pub local_rot_zoom: Option<TentacledVector3>,

    #[serde(rename = "LookCameraFOVZoom")]
    pub look_camera_fov_zoom: Option<i64>,

    pub only_start_aim_on_pressed: Option<bool>,

    #[serde(rename = "RecoilAnimation")]
    pub recoil_animation: Option<i64>,

    pub rot_to_aim_pos_min_dis: Option<f64>,

    #[serde(rename = "RunAnimation")]
    pub run_animation: Option<i64>,

    #[serde(rename = "SwayAmount")]
    pub sway_amount: Option<f64>,

    pub transition_to_aim: Option<TransitionToAim>,

    #[serde(rename = "WalkAnimation")]
    pub walk_animation: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleDofSettingsData {
    pub aperture: Option<f64>,

    pub enabled: Option<bool>,

    pub focal_length: Option<f64>,

    pub focus_distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransitionToAim {
    Enum(EfpisTransitionTime),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EfpisTransitionTime {
    Quick,

    Slow,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfItemMovementAnimationDataBlock {
    pub blocks: Option<Vec<ItemMovementAnimationDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemMovementAnimationDataBlock {
    pub camera_position: Option<Vector3AnimationCurve>,

    pub camera_rotation: Option<Vector3AnimationCurve>,

    pub item_position: Option<Vector3AnimationCurve>,

    pub item_rotation: Option<Vector3AnimationCurve>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vector3AnimationCurve {
    pub variance_x: Option<Vector3AnimationCurveVector2>,

    pub variance_y: Option<Vector3AnimationCurveVector2>,

    pub variance_z: Option<Vector3AnimationCurveVector2>,

    pub x: Option<HashMap<String, Option<serde_json::Value>>>,

    pub y: Option<HashMap<String, Option<serde_json::Value>>>,

    pub z: Option<HashMap<String, Option<serde_json::Value>>>,
}

#[derive(Serialize, Deserialize)]
pub struct Vector3AnimationCurveVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfItemPartDataBlock {
    pub blocks: Option<Vec<ItemPartDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemPartDataBlock {
    pub part_prefab: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfLevelGenSettingsDataBlock {
    pub blocks: Option<Vec<LevelGenSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelGenSettingsDataBlock {
    pub public_estimated_area_coverage: Option<f64>,

    pub public_name: Option<String>,

    pub segment_zone_count: Option<i64>,

    pub start_zone_size: Option<f64>,

    pub zone_max_size: Option<f64>,

    pub zone_min_size: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfLevelLayoutDataBlock {
    pub blocks: Option<Vec<LevelLayoutDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelLayoutDataBlock {
    pub zone_alias_start: Option<i64>,

    pub zones: Option<Vec<ExpeditionZoneData>>,
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

    pub big_pickup_distribution_in_zone: Option<i64>,

    pub build_from_local_index: Option<BuildFromLocalIndexUnion>,

    #[serde(rename = "BulkheadDCScanSeed")]
    pub bulkhead_dc_scan_seed: Option<i64>,

    pub chained_puzzle_to_enter: Option<i64>,

    pub consumable_distribution_in_zone: Option<i64>,

    pub corpse_clusters_in_zone: Option<i64>,

    pub corpses_in_zone: Option<StaticGroundSpawnersInZone>,

    pub coverage_min_max: Option<ZoneVector2>,

    pub custom_geomorph: Option<String>,

    pub decon_units_in_zone: Option<StaticGroundSpawnersInZone>,

    pub disinfection_multi: Option<f64>,

    pub disinfection_placement: Option<ZoneZonePlacementWeights>,

    pub disinfection_station_placements: Option<Vec<FunctionPlacementData>>,

    pub enemy_respawn_count_multiplier: Option<f64>,

    pub enemy_respawn_exclude_list: Option<Vec<i64>>,

    pub enemy_respawning: Option<bool>,

    pub enemy_respawn_require_other_zone: Option<bool>,

    pub enemy_respawn_room_distance: Option<i64>,

    pub enemy_respawn_time_interval: Option<f64>,

    pub enemy_spawning_in_zone: Option<Vec<EnemySpawningInZoneElement>>,

    pub events_on_approach_door: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_boss_death: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_door_scan_done: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_door_scan_start: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_enter: Option<Vec<LevelEventData>>,

    pub events_on_open_door: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_portal_warp: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_terminal_deactivate_alarm: Option<Vec<EventsOnApproachDoorElement>>,

    pub events_on_trigger: Option<Vec<WorldEventFromSourceData>>,

    pub events_on_unlock_door: Option<Vec<EventsOnApproachDoorElement>>,

    pub forbid_terminals_in_zone: Option<bool>,

    pub force_big_pickups_allocation: Option<bool>,

    pub generator_clusters_in_zone: Option<i64>,

    pub ground_spawners_in_zone: Option<StaticGroundSpawnersInZone>,

    pub health_multi: Option<f64>,

    pub health_placement: Option<ZoneZonePlacementWeights>,

    #[serde(rename = "HSUClustersInZone")]
    pub hsu_clusters_in_zone: Option<i64>,

    #[serde(rename = "HSUsInZone")]
    pub hs_us_in_zone: Option<StaticGroundSpawnersInZone>,

    pub ignore_random_geomorph_rotation: Option<bool>,

    pub is_checkpoint_door: Option<bool>,

    pub light_settings: Option<i64>,

    pub lights_sub_seed: Option<i64>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub marker_sub_seed: Option<i64>,

    pub override_alias_prefix: Option<bool>,

    pub play_scanner_voice_audio: Option<bool>,

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

    pub tool_ammo_multi: Option<f64>,

    pub tool_ammo_placement: Option<ZoneZonePlacementWeights>,

    pub turn_off_alarm_on_terminal: Option<bool>,

    pub use_static_bioscan_points_in_zone: Option<bool>,

    pub weapon_ammo_multi: Option<f64>,

    pub weapon_ammo_placement: Option<ZoneZonePlacementWeights>,

    pub world_event_chained_puzzle_datas: Option<Vec<SpecificChainPuzzleSpawnData>>,

    pub zone_expansion: Option<ZoneExpansion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActiveEnemyWaveData {
    pub enemy_group_in_area: Option<i64>,

    pub enemy_group_infront_of_door: Option<i64>,

    pub enemy_groups_in_area: Option<i64>,

    pub has_active_enemy_wave: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AltitudeData {
    pub allowed_zone_altitude: Option<AllowedZoneAltitude>,

    pub chance_to_change: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AllowedZoneAltitude {
    Enum(EWantedZoneHeighs),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWantedZoneHeighs {
    Ascending,

    Descending,

    #[serde(rename = "LowHigh")]
    LowHigh,

    #[serde(rename = "LowMid")]
    LowMid,

    #[serde(rename = "LowMidHigh")]
    LowMidHigh,

    #[serde(rename = "MidHigh")]
    MidHigh,

    #[serde(rename = "OnlyHigh")]
    OnlyHigh,

    #[serde(rename = "OnlyLow")]
    OnlyLow,

    #[serde(rename = "OnlyMid")]
    OnlyMid,

    Unchanged,
}

#[derive(Serialize, Deserialize)]
pub struct ZoneVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ZoneZonePlacementWeights {
    pub end: Option<f64>,

    pub middle: Option<f64>,

    pub start: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FunctionPlacementData {
    pub area_seed_offset: Option<i64>,

    pub marker_seed_offset: Option<i64>,

    pub placement_weights: Option<ZoneZonePlacementWeights>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemySpawningInZoneElement {
    pub difficulty: Option<Difficulty>,

    pub distribution: Option<StaticEnemySpawningInZoneDistribution>,

    pub distribution_value: Option<f64>,

    pub group_type: Option<GroupTypeUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventsOnApproachDoorElement {
    pub chain_puzzle: Option<i64>,

    pub clear_dimension: Option<bool>,

    pub condition: Option<EventsOnApproachDoorWorldEventConditionPair>,

    pub count: Option<i64>,

    pub custom_sub_objective: Option<CustomTextUnion>,

    pub custom_sub_objective_header: Option<CustomTextUnion>,

    pub delay: Option<f64>,

    #[serde(rename = "DialogueID")]
    pub dialogue_id: Option<i64>,

    pub dimension_index: Option<DimensionIndex>,

    pub duration: Option<f64>,

    pub enabled: Option<bool>,

    #[serde(rename = "EnemyID")]
    pub enemy_id: Option<i64>,

    pub enemy_wave_data: Option<EventsOnApproachDoorGenericEnemyWaveData>,

    pub fog_setting: Option<i64>,

    pub fog_transition_duration: Option<f64>,

    pub layer: Option<Layer>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub position: Option<EventsOnApproachDoorVector3>,

    #[serde(rename = "SoundID")]
    pub sound_id: Option<i64>,

    pub sound_subtitle: Option<CustomTextUnion>,

    pub terminal_command: Option<TerminalCommand>,

    pub terminal_command_rule: Option<AlCommandRule>,

    pub trigger: Option<EventsOnBossDeathTrigger>,

    #[serde(rename = "Type")]
    pub warden_objective_event_data_type: Option<EventsOnBossDeathType>,

    pub use_static_bioscan_points: Option<bool>,

    pub warden_intel: Option<CustomTextUnion>,

    pub world_event_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventsOnApproachDoorWorldEventConditionPair {
    pub condition_index: Option<i64>,

    pub is_true: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CustomTextUnion {
    CustomTextLocalizedText(CustomTextLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomTextLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventsOnApproachDoorGenericEnemyWaveData {
    pub area_distance: Option<i64>,

    pub intel_message: Option<CustomTextUnion>,

    pub spawn_delay: Option<f64>,

    pub trigger_alarm: Option<bool>,

    pub wave_population: Option<i64>,

    pub wave_settings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct EventsOnApproachDoorVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelEventData {
    pub delay: Option<f64>,

    pub intel: Option<LevelEventWardenIntelData>,

    pub noise: Option<LevelEventNoiseData>,

    pub sound: Option<LevelEventSoundData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelEventWardenIntelData {
    pub enabled: Option<bool>,

    pub intel_message: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelEventNoiseData {
    pub enabled: Option<bool>,

    pub radius_max: Option<f64>,

    pub radius_min: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LevelEventSoundData {
    pub enabled: Option<bool>,

    pub sound_event: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WorldEventFromSourceData {
    pub chain_puzzle: Option<i64>,

    pub clear_dimension: Option<bool>,

    pub condition: Option<EventsOnApproachDoorWorldEventConditionPair>,

    pub count: Option<i64>,

    pub custom_sub_objective: Option<CustomTextUnion>,

    pub custom_sub_objective_header: Option<CustomTextUnion>,

    pub delay: Option<f64>,

    #[serde(rename = "DialogueID")]
    pub dialogue_id: Option<i64>,

    pub dimension_index: Option<DimensionIndex>,

    pub duration: Option<f64>,

    pub enabled: Option<bool>,

    #[serde(rename = "EnemyID")]
    pub enemy_id: Option<i64>,

    pub enemy_wave_data: Option<EventsOnApproachDoorGenericEnemyWaveData>,

    pub fog_setting: Option<i64>,

    pub fog_transition_duration: Option<f64>,

    pub layer: Option<Layer>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub position: Option<EventsOnApproachDoorVector3>,

    #[serde(rename = "SoundID")]
    pub sound_id: Option<i64>,

    pub sound_subtitle: Option<CustomTextUnion>,

    pub terminal_command: Option<TerminalCommand>,

    pub terminal_command_rule: Option<AlCommandRule>,

    pub trigger: Option<EventsOnBossDeathTrigger>,

    #[serde(rename = "Type")]
    pub world_event_from_source_data_type: Option<EventsOnBossDeathType>,

    pub use_static_bioscan_points: Option<bool>,

    pub warden_intel: Option<CustomTextUnion>,

    pub world_event_object_filter: Option<String>,

    pub world_event_trigger_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressionPuzzleData {
    pub custom_text: Option<CustomTextUnion>,

    pub placement_count: Option<i64>,

    pub puzzle_type: Option<PuzzleType>,

    pub zone_placement_data: Option<Vec<ProgressionPuzzleDataZonePlacementDatum>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PuzzleType {
    Enum(EProgressionPuzzleType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EProgressionPuzzleType {
    #[serde(rename = "Keycard_SecurityBox")]
    KeycardSecurityBox,

    #[serde(rename = "Locked_No_Key")]
    LockedNoKey,

    None,

    #[serde(rename = "PowerGenerator_And_PowerCell")]
    PowerGeneratorAndPowerCell,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProgressionPuzzleDataZonePlacementDatum {
    pub dimension_index: Option<DimensionIndex>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub weights: Option<ZoneZonePlacementWeights>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecurityGateToEnter {
    Enum(ESecurityGateType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ESecurityGateType {
    Apex,

    Security,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecificPickupSpawnData {
    pub pickup_to_spawn: Option<i64>,

    pub world_event_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartExpansion {
    Enum(EZoneBuildFromExpansionType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EZoneBuildFromExpansionType {
    #[serde(rename = "Towards_Backward")]
    TowardsBackward,

    #[serde(rename = "Towards_Forward")]
    TowardsForward,

    #[serde(rename = "Towards_Left")]
    TowardsLeft,

    #[serde(rename = "Towards_Random")]
    TowardsRandom,

    #[serde(rename = "Towards_Right")]
    TowardsRight,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum StartPosition {
    Enum(EZoneBuildFromType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EZoneBuildFromType {
    #[serde(rename = "From_AverageCenter")]
    FromAverageCenter,

    #[serde(rename = "From_BetweenStartAndFurthest")]
    FromBetweenStartAndFurthest,

    #[serde(rename = "From_Furthest")]
    FromFurthest,

    #[serde(rename = "From_IndexWeight")]
    FromIndexWeight,

    #[serde(rename = "From_Random")]
    FromRandom,

    #[serde(rename = "From_Start")]
    FromStart,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticSpawnDataContainer {
    pub count: Option<i64>,

    pub distribution_random_blend: Option<f64>,

    pub distribution_result_pow: Option<f64>,

    pub distribution_weight: Option<f64>,

    pub distribution_weight_type: Option<DistributionWeightType>,

    pub fixed_seed: Option<i64>,

    pub static_spawn_data_id: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistributionWeightType {
    Enum(LgStaticDistributionWeightType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgStaticDistributionWeightType {
    #[serde(rename = "Weight_is_exact_node_index")]
    WeightIsExactNodeIndex,

    #[serde(rename = "Weight_is_zeroToOne_startToEnd")]
    WeightIsZeroToOneStartToEnd,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalPlacementElement {
    pub area_seed_offset: Option<i64>,

    pub local_log_files: Option<Vec<TerminalPlacementLocalLogFile>>,

    pub marker_seed_offset: Option<i64>,

    pub placement_weights: Option<ZoneZonePlacementWeights>,

    pub starting_state_data: Option<TerminalPlacementTerminalStartStateData>,

    pub unique_commands: Option<Vec<TerminalPlacementUniqueCommand>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalPlacementLocalLogFile {
    pub attached_audio_byte_size: Option<i64>,

    pub attached_audio_file: Option<i64>,

    pub file_content: Option<CustomTextUnion>,

    pub file_content_original_language: Option<FileContentOriginalLanguage>,

    pub file_name: Option<String>,

    pub player_dialog_to_trigger_after_audio: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalPlacementTerminalStartStateData {
    pub audio_event_enter: Option<i64>,

    pub audio_event_exit: Option<i64>,

    pub custom_info_text: Option<CustomTextUnion>,

    pub generate_password: Option<bool>,

    pub keep_showing_local_log_count: Option<bool>,

    pub password: Option<String>,

    pub password_hint_text: Option<String>,

    pub password_part_count: Option<i64>,

    pub password_protected: Option<bool>,

    pub show_password_length: Option<bool>,

    pub show_password_part_positions: Option<bool>,

    pub starting_state: Option<StartingState>,

    pub terminal_zone_selection_datas: Option<Vec<Vec<TerminalZoneSelectionData>>>,

    pub use_custom_info_text: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalZoneSelectionData {
    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub seed_type: Option<SeedType>,

    pub static_seed: Option<i64>,

    pub terminal_index: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TerminalPlacementUniqueCommand {
    pub command: Option<String>,

    pub command_desc: Option<CustomTextUnion>,

    pub command_events: Option<Vec<EventsOnApproachDoorElement>>,

    pub post_command_outputs: Option<Vec<FluffyTerminalOutput>>,

    pub special_command_rule: Option<AlCommandRule>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyTerminalOutput {
    pub line_type: Option<LineType>,

    pub output: Option<CustomTextUnion>,

    pub time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecificChainPuzzleSpawnData {
    pub chained_puzzle: Option<i64>,

    pub events_on_scan_done: Option<Vec<EventsOnApproachDoorElement>>,

    pub world_event_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZoneExpansion {
    Enum(EZoneExpansionType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EZoneExpansionType {
    Collapsed,

    #[serde(rename = "Directional_Backward")]
    DirectionalBackward,

    #[serde(rename = "Directional_Forward")]
    DirectionalForward,

    #[serde(rename = "Directional_Left")]
    DirectionalLeft,

    #[serde(rename = "Directional_Random")]
    DirectionalRandom,

    #[serde(rename = "Directional_Right")]
    DirectionalRight,

    Expansional,

    Random,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfLightSettingsDataBlock {
    pub blocks: Option<Vec<LightSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LightSettingsDataBlock {
    pub light_category_settings: Option<Vec<LightCategorySetting>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LightCategorySetting {
    pub category: Option<LightCategorySettingCategory>,

    pub chance: Option<f64>,

    pub chance_broken: Option<f64>,

    pub color: Option<LightCategorySettingColor>,

    pub intensity_mul: Option<f64>,

    pub on: Option<bool>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LightCategorySettingCategory {
    Enum(LightCategory),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LightCategory {
    Door,

    #[serde(rename = "DoorImportant")]
    DoorImportant,

    Emergency,

    General,

    Independent,

    Sign,

    Special,
}

#[derive(Serialize, Deserialize)]
pub struct LightCategorySettingColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfLootDataBlock {
    pub blocks: Option<Vec<LootDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LootDataBlock {
    pub category: Option<BlockCategory>,

    pub loot_collection: Option<Vec<Loot>>,

    pub public_name: Option<String>,

    pub weight: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum BlockCategory {
    Enum(LootCategory),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LootCategory {
    #[serde(rename = "ArmoryBig")]
    ArmoryBig,

    #[serde(rename = "ArmorySmall")]
    ArmorySmall,

    #[serde(rename = "ArtifactBig")]
    ArtifactBig,

    #[serde(rename = "ArtifactSmall")]
    ArtifactSmall,

    #[serde(rename = "LootBig")]
    LootBig,

    #[serde(rename = "LootSmall")]
    LootSmall,

    #[serde(rename = "MedicBig")]
    MedicBig,

    #[serde(rename = "MedicSmall")]
    MedicSmall,

    #[serde(rename = "ObjectiveItem")]
    ObjectiveItem,

    #[serde(rename = "ResourcesBig")]
    ResourcesBig,

    #[serde(rename = "ResourcesSmall")]
    ResourcesSmall,
}

#[derive(Serialize, Deserialize)]
pub struct Loot {
    #[serde(rename = "m_amountAbs")]
    pub m_amount_abs: Option<i64>,

    #[serde(rename = "m_amountRel")]
    pub m_amount_rel: Option<f64>,

    pub m_type: Option<MType>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MType {
    Enum(LootType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LootType {
    Ammo,

    #[serde(rename = "ArtifactBasic")]
    ArtifactBasic,

    #[serde(rename = "ArtifactExotic")]
    ArtifactExotic,

    #[serde(rename = "ArtifactLegendary")]
    ArtifactLegendary,

    #[serde(rename = "ArtifactRare")]
    ArtifactRare,

    #[serde(rename = "ArtifactUncommon")]
    ArtifactUncommon,

    Battery,

    Health,

    #[serde(rename = "ObjectiveScavangeItem")]
    ObjectiveScavangeItem,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMarkerGroupDataBlock {
    pub blocks: Option<Vec<MarkerGroupDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MarkerGroupDataBlock {
    pub mesh_color: Option<IndigoColor>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMeleeAnimationSetDataBlock {
    pub blocks: Option<Vec<MeleeAnimationSetDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MeleeAnimationSetDataBlock {
    pub auto_attack_time: Option<f64>,

    pub auto_attack_warning_time: Option<f64>,

    #[serde(rename = "FPAttackChargeUpHitLeft")]
    pub fp_attack_charge_up_hit_left: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackChargeUpHitRight")]
    pub fp_attack_charge_up_hit_right: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackChargeUpLeft")]
    pub fp_attack_charge_up_left: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackChargeUpReleaseLeft")]
    pub fp_attack_charge_up_release_left: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackChargeUpReleaseRight")]
    pub fp_attack_charge_up_release_right: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackChargeUpRight")]
    pub fp_attack_charge_up_right: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackHitLeft")]
    pub fp_attack_hit_left: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackHitRight")]
    pub fp_attack_hit_right: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackMissLeft")]
    pub fp_attack_miss_left: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackMissRight")]
    pub fp_attack_miss_right: Option<MeleeAttackData>,

    #[serde(rename = "FPAttackPush")]
    pub fp_attack_push: Option<MeleeAttackData>,

    #[serde(rename = "FPChargeCancelAnimLeft")]
    pub fp_charge_cancel_anim_left: Option<AnimHash>,

    #[serde(rename = "FPChargeCancelAnimRight")]
    pub fp_charge_cancel_anim_right: Option<AnimHash>,

    #[serde(rename = "FPIdleAnim")]
    pub fp_idle_anim: Option<AnimHash>,

    #[serde(rename = "FPJumpAnim")]
    pub fp_jump_anim: Option<AnimHash>,

    #[serde(rename = "FPLandAnim")]
    pub fp_land_anim: Option<AnimHash>,

    #[serde(rename = "FPRunAnim")]
    pub fp_run_anim: Option<AnimHash>,

    #[serde(rename = "FPSettleAnim")]
    pub fp_settle_anim: Option<AnimHash>,

    #[serde(rename = "FPWalkAnim")]
    pub fp_walk_anim: Option<AnimHash>,

    pub hold_to_charge_time: Option<f64>,

    pub max_damage_charge_time: Option<f64>,

    #[serde(rename = "TPAnimHashAttackLeft")]
    pub tp_anim_hash_attack_left: Option<AnimHash>,

    #[serde(rename = "TPAnimHashAttackLeftCharge")]
    pub tp_anim_hash_attack_left_charge: Option<AnimHash>,

    #[serde(rename = "TPAnimHashAttackLeftChargeCrouch")]
    pub tp_anim_hash_attack_left_charge_crouch: Option<AnimHash>,

    #[serde(rename = "TPAnimHashAttackLeftCrouch")]
    pub tp_anim_hash_attack_left_crouch: Option<AnimHash>,

    #[serde(rename = "TPAnimHashAttackLeftRelease")]
    pub tp_anim_hash_attack_left_release: Option<AnimHash>,

    #[serde(rename = "TPAnimHashAttackLeftReleaseCrouch")]
    pub tp_anim_hash_attack_left_release_crouch: Option<AnimHash>,

    #[serde(rename = "TPAnimHashIdle")]
    pub tp_anim_hash_idle: Option<AnimHash>,

    #[serde(rename = "TPAnimHashIdleCrouch")]
    pub tp_anim_hash_idle_crouch: Option<AnimHash>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MeleeAttackData {
    pub anim: Option<AnimHash>,

    pub anim_blend_in: Option<f64>,

    pub attack_cam_fwd_hit_frame_time: Option<f64>,

    pub attack_hit_frame_time: Option<f64>,

    pub attack_length_frame_time: Option<f64>,

    pub combo_early_frame_time: Option<f64>,

    pub damage_end_frame_time: Option<f64>,

    pub damage_start_frame_time: Option<f64>,

    pub side: Option<Side>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AnimHash {
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Side {
    Enum(EMeleeAttackSide),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EMeleeAttackSide {
    Left,

    Middle,

    None,

    Right,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMeleeArchetypeDataBlock {
    pub blocks: Option<Vec<MeleeArchetypeDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MeleeArchetypeDataBlock {
    pub allow_running_when_charging: Option<bool>,

    pub attack_sphere_radius: Option<f64>,

    pub camera_damage_ray_length: Option<f64>,

    pub can_hit_multiple_enemies: Option<bool>,

    pub charged_attack_damage: Option<f64>,

    pub charged_attack_stamina_cost: Option<PurpleActionCost>,

    pub charged_backstabber_multi: Option<f64>,

    pub charged_environment_multi: Option<f64>,

    pub charged_precision_multi: Option<f64>,

    pub charged_sleeper_multi: Option<f64>,

    pub charged_stagger_multi: Option<f64>,

    pub evaluate_hold_before_attack: Option<bool>,

    pub light_attack_damage: Option<f64>,

    pub light_attack_stamina_cost: Option<PurpleActionCost>,

    pub light_backstabber_multi: Option<f64>,

    pub light_environment_multi: Option<f64>,

    pub light_precision_multi: Option<f64>,

    pub light_sleeper_multi: Option<f64>,

    pub light_stagger_multi: Option<f64>,

    pub melee_animation_set: Option<i64>,

    #[serde(rename = "MeleeSFXSet")]
    pub melee_sfx_set: Option<i64>,

    pub noise_level: Option<NoiseLevel>,

    pub player_run_speed_multi_while_charging: Option<f64>,

    pub play_impact_effect: Option<bool>,

    pub public_name: Option<String>,

    pub push_damage_sphere_radius: Option<f64>,

    pub push_stamina_cost: Option<PurpleActionCost>,

    pub skip_limb_destruction: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleActionCost {
    pub base_stamina_cost_in_combat: Option<f64>,

    pub base_stamina_cost_out_of_combat: Option<f64>,

    pub reset_resting_timer_in_combat: Option<bool>,

    pub reset_resting_timer_out_of_combat: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum NoiseLevel {
    Enum(DamageNoiseLevel),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DamageNoiseLevel {
    Low,

    Normal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMeleeSfxDataBlock {
    pub blocks: Option<Vec<MeleeSfxDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MeleeSfxDataBlock {
    pub charge: Option<i64>,

    pub charged: Option<i64>,

    pub charged_cancel_warning: Option<i64>,

    pub feedback_melee_hit_hard: Option<i64>,

    pub feedback_melee_hit_normal: Option<i64>,

    pub initial_foley: Option<i64>,

    pub player_cancel_charge: Option<i64>,

    pub shove_hit: Option<i64>,

    pub swish_heavy: Option<i64>,

    pub swish_regular_delay: Option<f64>,

    pub swish_regular_left: Option<i64>,

    pub swish_regular_right: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMiningMarkerDataBlock {
    pub blocks: Option<Vec<MiningMarkerDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MiningMarkerDataBlock {
    pub common_data: Option<PurpleMarkerDataCommon>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PurpleMarkerDataCommon {
    pub asset_bundle_name: Option<BundleName>,

    pub bounding_volume: Option<f64>,

    pub compositions: Option<Vec<PurpleMarkerComposition>>,

    pub editor_mesh: Option<String>,

    pub function_potential: Option<f64>,

    pub group: Option<i64>,

    pub rotation_noise: Option<f64>,

    pub rotation_snap: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleMarkerComposition {
    pub function: Option<Function>,

    pub prefab: Option<String>,

    #[serde(rename = "Shard")]
    pub shard: Option<Shard>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Function {
    Enum(ExpeditionFunction),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ExpeditionFunction {
    #[serde(rename = "AirSupply")]
    AirSupply,

    #[serde(rename = "BigPickupItem")]
    BigPickupItem,

    #[serde(rename = "BulkheadDoorController")]
    BulkheadDoorController,

    Corpse,

    #[serde(rename = "CorpseCluster")]
    CorpseCluster,

    #[serde(rename = "DisinfectionStation")]
    DisinfectionStation,

    #[serde(rename = "GeneratorCluster")]
    GeneratorCluster,

    #[serde(rename = "GroundSpawn")]
    GroundSpawn,

    #[serde(rename = "HydroStatisUnit")]
    HydroStatisUnit,

    #[serde(rename = "HydroStatisUnitCluster")]
    HydroStatisUnitCluster,

    None,

    #[serde(rename = "PowerGenerator")]
    PowerGenerator,

    #[serde(rename = "ResourceContainerCluster")]
    ResourceContainerCluster,

    #[serde(rename = "ResourceContainerSecure")]
    ResourceContainerSecure,

    #[serde(rename = "ResourceContainerWeak")]
    ResourceContainerWeak,

    Security,

    #[serde(rename = "SecurityTurret")]
    SecurityTurret,

    Sign,

    #[serde(rename = "SmallPickupItem")]
    SmallPickupItem,

    Strongbox,

    Terminal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMlsArrayDescriptorReferenceDataBlock {
    pub blocks: Option<Vec<MlsArrayDescriptorReferenceDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MlsArrayDescriptorReferenceDataBlock {
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfMusicStateDataBlock {
    pub blocks: Option<Vec<MusicStateDataBlock>>,
}

#[derive(Serialize, Deserialize)]
pub struct MusicStateDataBlock {
    #[serde(rename = "m_EventOnBigScare")]
    pub m_event_on_big_scare: Option<String>,

    #[serde(rename = "m_EventOnSmallScare")]
    pub m_event_on_small_scare: Option<String>,

    #[serde(rename = "m_EventsOnEnter")]
    pub m_events_on_enter: Option<Vec<MusicEventData>>,

    #[serde(rename = "m_EventsOnExit")]
    pub m_events_on_exit: Option<Vec<MusicEventDataConditional>>,

    #[serde(rename = "m_EventsOnTime")]
    pub m_events_on_time: Option<Vec<MusicEventDataTimed>>,

    #[serde(rename = "m_EventsOnTimeInactive")]
    pub m_events_on_time_inactive: Option<Vec<MusicEventDataTimed>>,

    #[serde(rename = "m_gameEventSounds")]
    pub m_game_event_sounds: Option<Vec<GameEventSound>>,

    #[serde(rename = "m_RTPC_1")]
    pub m_rtpc_1: Option<String>,

    #[serde(rename = "m_RTPC_2")]
    pub m_rtpc_2: Option<String>,

    #[serde(rename = "m_RTPC_3")]
    pub m_rtpc_3: Option<String>,

    #[serde(rename = "m_SwitchesOnCustomCue")]
    pub m_switches_on_custom_cue: Option<Vec<MusicSwitchDataOnCustomCue>>,

    #[serde(rename = "m_SwitchesOnEnter")]
    pub m_switches_on_enter: Option<Vec<MusicSwitchData>>,

    #[serde(rename = "m_SwitchesOnTime")]
    pub m_switches_on_time: Option<Vec<MusicSwitchDataTimed>>,

    #[serde(rename = "m_SwitchesOnTimeInactive")]
    pub m_switches_on_time_inactive: Option<Vec<MusicSwitchDataTimed>>,

    pub m_timing: Option<TimingData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicEventData {
    pub event_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicEventDataConditional {
    pub event_name: Option<String>,

    pub only_when_exiting_to: Option<OnlyWhenExitingTo>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnlyWhenExitingTo {
    Enum(MusState),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum MusState {
    #[serde(rename = "CombatHidden")]
    CombatHidden,

    #[serde(rename = "CombatRegular")]
    CombatRegular,

    #[serde(rename = "ElevatorGoingDown")]
    ElevatorGoingDown,

    #[serde(rename = "ElevatorIdle")]
    ElevatorIdle,

    #[serde(rename = "EncounterHidden")]
    EncounterHidden,

    #[serde(rename = "EncounterRegular")]
    EncounterRegular,

    Exploration,

    #[serde(rename = "IntentionalCombatHidden")]
    IntentionalCombatHidden,

    #[serde(rename = "IntentionalCombatRegular")]
    IntentionalCombatRegular,

    #[serde(rename = "MainMenu")]
    MainMenu,

    None,

    Silence,

    #[serde(rename = "StartUp")]
    StartUp,

    #[serde(rename = "SurvivalEpicMoment")]
    SurvivalEpicMoment,

    #[serde(rename = "SurvivalExtreme")]
    SurvivalExtreme,

    #[serde(rename = "SurvivalHidden")]
    SurvivalHidden,

    #[serde(rename = "SurvivalRegular")]
    SurvivalRegular,

    Tension,

    #[serde(rename = "TensionMax")]
    TensionMax,

    Testing,

    Theme,
}

#[derive(Serialize, Deserialize)]
pub struct MusicEventDataTimed {
    pub events: Option<Vec<MusicEventData>>,

    pub repeat: Option<bool>,

    pub time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameEventSound {
    pub cooldown: Option<f64>,

    pub game_event: Option<GameEvent>,

    pub sound_event: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum GameEvent {
    Enum(EGameEvent),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EGameEvent {
    #[serde(rename = "bioscan_end")]
    BioscanEnd,

    #[serde(rename = "bioscan_start")]
    BioscanStart,

    #[serde(rename = "checkpoint_reload")]
    CheckpointReload,

    #[serde(rename = "enemy_big_attack")]
    EnemyBigAttack,

    #[serde(rename = "enemy_dead")]
    EnemyDead,

    #[serde(rename = "enemy_dead_from_melee")]
    EnemyDeadFromMelee,

    #[serde(rename = "enemy_scout_dead_from_melee")]
    EnemyScoutDeadFromMelee,

    #[serde(rename = "enemy_shooter_attack")]
    EnemyShooterAttack,

    #[serde(rename = "enemy_spotted")]
    EnemySpotted,

    #[serde(rename = "enemy_striker_attack")]
    EnemyStrikerAttack,

    #[serde(rename = "enemy_wave_spawned")]
    EnemyWaveSpawned,

    #[serde(rename = "game_quit")]
    GameQuit,

    #[serde(rename = "gs_AfterLevel")]
    GsAfterLevel,

    #[serde(rename = "gs_CaptureRecall")]
    GsCaptureRecall,

    #[serde(rename = "gs_ExpeditionAbort")]
    GsExpeditionAbort,

    #[serde(rename = "gs_ExpeditionFail")]
    GsExpeditionFail,

    #[serde(rename = "gs_ExpeditionSuccess")]
    GsExpeditionSuccess,

    #[serde(rename = "gs_FakeLobby")]
    GsFakeLobby,

    #[serde(rename = "gs_Generating")]
    GsGenerating,

    #[serde(rename = "gs_InLevel")]
    GsInLevel,

    #[serde(rename = "gs_Lobby")]
    GsLobby,

    #[serde(rename = "gs_NoLobby")]
    GsNoLobby,

    #[serde(rename = "gs_Offline")]
    GsOffline,

    #[serde(rename = "gs_ReadyToStartLevel")]
    GsReadyToStartLevel,

    #[serde(rename = "gs_ReadyToStopElevatorRide")]
    GsReadyToStopElevatorRide,

    #[serde(rename = "gs_Slim")]
    GsSlim,

    #[serde(rename = "gs_Startup")]
    GsStartup,

    #[serde(rename = "gs_StopElevatorRide")]
    GsStopElevatorRide,

    #[serde(rename = "hibernating_enemy_dead")]
    HibernatingEnemyDead,

    #[serde(rename = "hibernating_enemy_heartbeat")]
    HibernatingEnemyHeartbeat,

    #[serde(rename = "hibernating_enemy_spotted")]
    HibernatingEnemySpotted,

    #[serde(rename = "hibernating_enemy_wakeup")]
    HibernatingEnemyWakeup,

    #[serde(rename = "None")]
    None,

    #[serde(rename = "player_apply_ammokit")]
    PlayerApplyAmmokit,

    #[serde(rename = "player_apply_disinfection")]
    PlayerApplyDisinfection,

    #[serde(rename = "player_apply_medikit")]
    PlayerApplyMedikit,

    #[serde(rename = "player_apply_toolRefill")]
    PlayerApplyToolRefill,

    #[serde(rename = "player_downed")]
    PlayerDowned,

    #[serde(rename = "player_downed_2_total")]
    PlayerDowned2_Total,

    #[serde(rename = "player_downed_3_total")]
    PlayerDowned3_Total,

    #[serde(rename = "player_downed_4_total")]
    PlayerDowned4_Total,

    #[serde(rename = "player_enter_new_area")]
    PlayerEnterNewArea,

    #[serde(rename = "player_enter_new_zone")]
    PlayerEnterNewZone,

    #[serde(rename = "player_enter_terminal")]
    PlayerEnterTerminal,

    #[serde(rename = "player_exit_terminal")]
    PlayerExitTerminal,

    #[serde(rename = "player_fire_bullet")]
    PlayerFireBullet,

    #[serde(rename = "player_fire_glue")]
    PlayerFireGlue,

    #[serde(rename = "player_hacking_start")]
    PlayerHackingStart,

    #[serde(rename = "player_hacking_success")]
    PlayerHackingSuccess,

    #[serde(rename = "player_low_health")]
    PlayerLowHealth,

    #[serde(rename = "player_out_of_ammo_all_gear")]
    PlayerOutOfAmmoAllGear,

    #[serde(rename = "player_out_of_ammo_current_gear")]
    PlayerOutOfAmmoCurrentGear,

    #[serde(rename = "player_pickup_ammokit")]
    PlayerPickupAmmokit,

    #[serde(rename = "player_pickup_artifact")]
    PlayerPickupArtifact,

    #[serde(rename = "player_pickup_commodityLarge")]
    PlayerPickupCommodityLarge,

    #[serde(rename = "player_pickup_commodityMedium")]
    PlayerPickupCommodityMedium,

    #[serde(rename = "player_pickup_commoditySmall")]
    PlayerPickupCommoditySmall,

    #[serde(rename = "player_pickup_consumable")]
    PlayerPickupConsumable,

    #[serde(rename = "player_pickup_keycard")]
    PlayerPickupKeycard,

    #[serde(rename = "player_pickup_medikit")]
    PlayerPickupMedikit,

    #[serde(rename = "player_pickup_toolRefill")]
    PlayerPickupToolRefill,

    #[serde(rename = "player_place_sentrygun")]
    PlayerPlaceSentrygun,

    #[serde(rename = "player_place_tripmine")]
    PlayerPlaceTripmine,

    #[serde(rename = "player_reload")]
    PlayerReload,

    #[serde(rename = "player_revive_start")]
    PlayerReviveStart,

    #[serde(rename = "player_revived")]
    PlayerRevived,

    #[serde(rename = "player_revived_2_total")]
    PlayerRevived2_Total,

    #[serde(rename = "player_revived_3_total")]
    PlayerRevived3_Total,

    #[serde(rename = "player_start_firing")]
    PlayerStartFiring,

    #[serde(rename = "player_stop_firing")]
    PlayerStopFiring,

    #[serde(rename = "player_take_damage")]
    PlayerTakeDamage,

    #[serde(rename = "player_take_friendly_fire")]
    PlayerTakeFriendlyFire,

    #[serde(rename = "scout_enemy_dead")]
    ScoutEnemyDead,

    #[serde(rename = "scout_enemy_found_player")]
    ScoutEnemyFoundPlayer,

    #[serde(rename = "scout_enemy_spotted")]
    ScoutEnemySpotted,

    #[serde(rename = "scout_enemy_use_detect_ability")]
    ScoutEnemyUseDetectAbility,

    #[serde(rename = "security_door_opening")]
    SecurityDoorOpening,

    #[serde(rename = "sentry_gun_ammo_depleated")]
    SentryGunAmmoDepleated,

    #[serde(rename = "sentry_gun_fire")]
    SentryGunFire,

    #[serde(rename = "sticky_mine_explode")]
    StickyMineExplode,

    #[serde(rename = "term_Activate")]
    TermActivate,

    #[serde(rename = "term_ActivateBeacon")]
    TermActivateBeacon,

    #[serde(rename = "term_Close")]
    TermClose,

    #[serde(rename = "term_Cls")]
    TermCls,

    #[serde(rename = "term_Commands")]
    TermCommands,

    #[serde(rename = "term_Deactivate")]
    TermDeactivate,

    #[serde(rename = "term_DisableAlarm")]
    TermDisableAlarm,

    #[serde(rename = "term_DownloadData")]
    TermDownloadData,

    #[serde(rename = "term_EmptyLine")]
    TermEmptyLine,

    #[serde(rename = "term_Exit")]
    TermExit,

    #[serde(rename = "term_Find")]
    TermFind,

    #[serde(rename = "term_Help")]
    TermHelp,

    #[serde(rename = "term_InvalidCommand")]
    TermInvalidCommand,

    #[serde(rename = "term_Locate")]
    TermLocate,

    #[serde(rename = "term_Open")]
    TermOpen,

    #[serde(rename = "term_Override")]
    TermOverride,

    #[serde(rename = "term_Ping")]
    TermPing,

    #[serde(rename = "term_Query")]
    TermQuery,

    #[serde(rename = "term_ShowList")]
    TermShowList,

    #[serde(rename = "term_ViewSecurityLog")]
    TermViewSecurityLog,

    #[serde(rename = "time_InLevel_0")]
    TimeInLevel0,

    #[serde(rename = "time_InLevel_120")]
    TimeInLevel120,

    #[serde(rename = "time_InLevel_15")]
    TimeInLevel15,

    #[serde(rename = "time_InLevel_150")]
    TimeInLevel150,

    #[serde(rename = "time_InLevel_180")]
    TimeInLevel180,

    #[serde(rename = "time_InLevel_210")]
    TimeInLevel210,

    #[serde(rename = "time_InLevel_240")]
    TimeInLevel240,

    #[serde(rename = "time_InLevel_30")]
    TimeInLevel30,

    #[serde(rename = "time_InLevel_45")]
    TimeInLevel45,

    #[serde(rename = "time_InLevel_5")]
    TimeInLevel5,

    #[serde(rename = "time_InLevel_60")]
    TimeInLevel60,

    #[serde(rename = "time_InLevel_90")]
    TimeInLevel90,

    #[serde(rename = "weak_door_closing")]
    WeakDoorClosing,

    #[serde(rename = "weak_door_explode")]
    WeakDoorExplode,

    #[serde(rename = "weak_door_opening")]
    WeakDoorOpening,

    #[serde(rename = "weak_door_punched")]
    WeakDoorPunched,

    #[serde(rename = "wobj_FindLocationInfo")]
    WobjFindLocationInfo,

    #[serde(rename = "wobj_FindLocationInfoHelp")]
    WobjFindLocationInfoHelp,

    #[serde(rename = "wobj_GoToWinCondition")]
    WobjGoToWinCondition,

    #[serde(rename = "wobj_GoToWinConditionHelp")]
    WobjGoToWinConditionHelp,

    #[serde(rename = "wobj_GoToZone")]
    WobjGoToZone,

    #[serde(rename = "wobj_GoToZoneHelp")]
    WobjGoToZoneHelp,

    #[serde(rename = "wobj_InZoneFindItem")]
    WobjInZoneFindItem,

    #[serde(rename = "wobj_InZoneFindItemHelp")]
    WobjInZoneFindItemHelp,

    #[serde(rename = "wobj_SolveItem")]
    WobjSolveItem,

    #[serde(rename = "wobj_SolveItemHelp")]
    WobjSolveItemHelp,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicSwitchDataOnCustomCue {
    pub custom_cue: Option<String>,

    pub switches: Option<Vec<MusicSwitchData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicSwitchData {
    pub random_count: Option<i64>,

    pub switch_group: Option<String>,

    pub switch_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MusicSwitchDataTimed {
    pub repeat: Option<bool>,

    pub switches: Option<Vec<MusicSwitchData>>,

    pub time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimingData {
    pub max_time_in_state: Option<f64>,

    pub min_time_in_state: Option<f64>,

    pub state_after_max_time: Option<OnlyWhenExitingTo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfPlayerDataBlock {
    pub blocks: Option<Vec<PlayerDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDataBlock {
    #[serde(rename = "AdditionalFPSBodyOffsetWhenRunning")]
    pub additional_fps_body_offset_when_running: Option<StickyVector3>,

    pub air_move_speed: Option<f64>,

    #[serde(rename = "AmmoClassInitial")]
    pub ammo_class_initial: Option<i64>,

    #[serde(rename = "AmmoClassInitialOnDropin")]
    pub ammo_class_initial_on_dropin: Option<i64>,

    #[serde(rename = "AmmoClassMaxCap")]
    pub ammo_class_max_cap: Option<i64>,

    #[serde(rename = "AmmoClassResourcePackMaxCap")]
    pub ammo_class_resource_pack_max_cap: Option<i64>,

    #[serde(rename = "AmmoSpecialInitial")]
    pub ammo_special_initial: Option<i64>,

    #[serde(rename = "AmmoSpecialInitialOnDropin")]
    pub ammo_special_initial_on_dropin: Option<i64>,

    #[serde(rename = "AmmoSpecialMaxCap")]
    pub ammo_special_max_cap: Option<i64>,

    #[serde(rename = "AmmoSpecialResourcePackMaxCap")]
    pub ammo_special_resource_pack_max_cap: Option<i64>,

    #[serde(rename = "AmmoStandardInitial")]
    pub ammo_standard_initial: Option<i64>,

    #[serde(rename = "AmmoStandardInitialOnDropin")]
    pub ammo_standard_initial_on_dropin: Option<i64>,

    #[serde(rename = "AmmoStandardMaxCap")]
    pub ammo_standard_max_cap: Option<i64>,

    #[serde(rename = "AmmoStandardResourcePackMaxCap")]
    pub ammo_standard_resource_pack_max_cap: Option<i64>,

    pub battery: Option<i64>,

    pub breathing_debug_enabled: Option<bool>,

    pub breathing_enabled: Option<bool>,

    pub breathing_health_low_limit: Option<f64>,

    pub breathing_scared_enabled: Option<bool>,

    pub breathing_stamina_enabled: Option<bool>,

    pub breathing_volume: Option<f64>,

    pub cam_fov_run_dif: Option<f64>,

    pub cam_pos_crouch: Option<StickyVector3>,

    pub cam_pos_default: Option<StickyVector3>,

    #[serde(rename = "ChromaticAbberationIntensityMax")]
    pub chromatic_abberation_intensity_max: Option<f64>,

    pub crouch_footstep_length: Option<f64>,

    pub crouch_move_speed: Option<f64>,

    pub default_dialog_line_delay: Option<f64>,

    pub dialog_enabled_in_spectator: Option<bool>,

    #[serde(rename = "DofDefault")]
    pub dof_default: Option<FluffyDofSettingsData>,

    #[serde(rename = "DofInElevator")]
    pub dof_in_elevator: Option<FluffyDofSettingsData>,

    #[serde(rename = "DofInTerminal")]
    pub dof_in_terminal: Option<FluffyDofSettingsData>,

    pub fall_damage_max: Option<f64>,

    pub fall_damage_max_height: Option<f64>,

    pub fall_damage_min: Option<f64>,

    pub fall_damage_min_height: Option<f64>,

    #[serde(rename = "FPSArmsOffset")]
    pub fps_arms_offset: Option<StickyVector3>,

    #[serde(rename = "FPSBodyMoveNeckToFollowCamera")]
    pub fps_body_move_neck_to_follow_camera: Option<bool>,

    #[serde(rename = "FPSBodyOffset")]
    pub fps_body_offset: Option<StickyVector3>,

    #[serde(rename = "FPSBodyOffsetWhenNotFollowingCamera")]
    pub fps_body_offset_when_not_following_camera: Option<StickyVector3>,

    #[serde(rename = "FPSBodyScale")]
    pub fps_body_scale: Option<StickyVector3>,

    #[serde(rename = "FPSBodyWantedDisToNeck")]
    pub fps_body_wanted_dis_to_neck: Option<f64>,

    pub friendly_fire_multi: Option<f64>,

    #[serde(rename = "GearMaxSightHeightDiff")]
    pub gear_max_sight_height_diff: Option<f64>,

    pub health: Option<f64>,

    pub health_regen_delay: Option<f64>,

    pub health_regen_per_second: Option<f64>,

    pub health_regen_rel_max: Option<f64>,

    pub health_regen_start_delay_after_damage: Option<f64>,

    pub implant_big_trigger_delay: Option<f64>,

    pub implant_small_trigger_delay: Option<f64>,

    pub item_anim_axis_weight: Option<StickyVector3>,

    pub item_anim_weight: Option<f64>,

    pub item_anim_weight_aiming: Option<f64>,

    pub item_footstep_damping: Option<f64>,

    pub item_footstep_delay: Option<f64>,

    pub item_footstep_impulse_scale: Option<StickyVector3>,

    pub item_footstep_stiffness: Option<f64>,

    pub item_footstep_weight: Option<f64>,

    pub item_footstep_weight_aiming: Option<f64>,

    pub item_lookat_aim_weight: Option<f64>,

    pub item_look_sway_pos_damping: Option<f64>,

    pub item_look_sway_pos_impulse_scale: Option<StickyVector3>,

    pub item_look_sway_pos_impulse_scale_aiming: Option<StickyVector3>,

    pub item_look_sway_pos_stiffness: Option<f64>,

    pub item_look_sway_rot_damping: Option<f64>,

    pub item_look_sway_rot_impulse_scale: Option<StickyVector3>,

    pub item_look_sway_rot_impulse_scale_aiming: Option<StickyVector3>,

    pub item_look_sway_rot_stiffness: Option<f64>,

    pub item_look_sway_rot_x_multi: Option<StickyVector2>,

    pub item_look_sway_weight: Option<f64>,

    pub item_look_sway_weight_aiming: Option<f64>,

    pub item_recoil_weight: Option<f64>,

    pub jump_gravity_mul_after_peak: Option<f64>,

    pub jump_gravity_mul_button_released: Option<f64>,

    pub jump_gravity_mul_default: Option<f64>,

    pub jump_gravity_mul_falling: Option<f64>,

    pub jump_vel_initial: Option<f64>,

    pub jump_vertical_velocity_max: Option<f64>,

    pub ladder_move_speed: Option<f64>,

    pub large_battery_consumtion_per_sec: Option<f64>,

    pub long_dialog_cooldown: Option<f64>,

    pub low_tension_max_limit: Option<f64>,

    pub medium_battery_consumtion_per_sec: Option<f64>,

    pub medium_dialog_cooldown: Option<f64>,

    pub medium_tension_max_limit: Option<f64>,

    pub mouselook_aim_scale_fov_ref: Option<StickyVector2>,

    pub mouselook_aim_scale_min_max: Option<StickyVector2>,

    pub nanoswarm_negative_resistance_curve: Option<HashMap<String, Option<serde_json::Value>>>,

    pub nanoswarm_shield_damage_multiplier_curve: Option<HashMap<String, Option<serde_json::Value>>>,

    pub nanoswarm_shield_resistance_curve: Option<HashMap<String, Option<serde_json::Value>>>,

    pub no_air_damage_delay: Option<f64>,

    pub no_air_damage_rel: Option<f64>,

    pub no_air_time_to_empty: Option<f64>,

    pub radio_distortion_in_spectator: Option<f64>,

    pub radio_enabled_default_distance: Option<f64>,

    pub radio_quality_in_spectator: Option<f64>,

    pub radio_quality_lowest_at_distance: Option<f64>,

    pub run_footstep_length: Option<f64>,

    pub run_move_speed: Option<f64>,

    pub short_dialog_cooldown: Option<f64>,

    pub small_battery_consumtion_per_sec: Option<f64>,

    #[serde(rename = "StaminaCrouchEnterCost")]
    pub stamina_crouch_enter_cost: Option<FluffyActionCost>,

    #[serde(rename = "StaminaEnableAffectMeleeSpeed")]
    pub stamina_enable_affect_melee_speed: Option<bool>,

    #[serde(rename = "StaminaEnableAffectMoveSpeed")]
    pub stamina_enable_affect_move_speed: Option<bool>,

    #[serde(rename = "StaminaExhaustedAudioThreshold")]
    pub stamina_exhausted_audio_threshold: Option<f64>,

    #[serde(rename = "StaminaHighMeleeSpeedModifier")]
    pub stamina_high_melee_speed_modifier: Option<f64>,

    #[serde(rename = "StaminaHighMovespeedModifier")]
    pub stamina_high_movespeed_modifier: Option<f64>,

    #[serde(rename = "StaminaJumpCost")]
    pub stamina_jump_cost: Option<FluffyActionCost>,

    #[serde(rename = "StaminaLowMeleeSpeedModifier")]
    pub stamina_low_melee_speed_modifier: Option<f64>,

    #[serde(rename = "StaminaLowMovespeedModifier")]
    pub stamina_low_movespeed_modifier: Option<f64>,

    #[serde(rename = "StaminaMaximumCapWhenInCombat")]
    pub stamina_maximum_cap_when_in_combat: Option<f64>,

    #[serde(rename = "StaminaMaximumCapWhenInCombatFallRate")]
    pub stamina_maximum_cap_when_in_combat_fall_rate: Option<f64>,

    #[serde(rename = "StaminaMeleeSpeedCurveExponent")]
    pub stamina_melee_speed_curve_exponent: Option<f64>,

    #[serde(rename = "StaminaMinimumCapWhenNotInCombat")]
    pub stamina_minimum_cap_when_not_in_combat: Option<f64>,

    #[serde(rename = "StaminaMoveSpeedCurveExponent")]
    pub stamina_move_speed_curve_exponent: Option<f64>,

    #[serde(rename = "StaminaRegenNotRestingInCombat")]
    pub stamina_regen_not_resting_in_combat: Option<f64>,

    #[serde(rename = "StaminaRegenNotRestingOutOfCombat")]
    pub stamina_regen_not_resting_out_of_combat: Option<f64>,

    #[serde(rename = "StaminaRegenRestingInCombat")]
    pub stamina_regen_resting_in_combat: Option<f64>,

    #[serde(rename = "StaminaRegenRestingOutOfCombat")]
    pub stamina_regen_resting_out_of_combat: Option<f64>,

    #[serde(rename = "StaminaRestedAudioThreshold")]
    pub stamina_rested_audio_threshold: Option<f64>,

    #[serde(rename = "StaminaRunCost")]
    pub stamina_run_cost: Option<FluffyActionCost>,

    #[serde(rename = "StaminaSneakCost")]
    pub stamina_sneak_cost: Option<FluffyActionCost>,

    #[serde(rename = "StaminaTimeBeforeResting")]
    pub stamina_time_before_resting: Option<f64>,

    #[serde(rename = "StaminaUsageAffectedByDrama")]
    pub stamina_usage_affected_by_drama: Option<bool>,

    #[serde(rename = "StaminaWalkCost")]
    pub stamina_walk_cost: Option<FluffyActionCost>,

    #[serde(rename = "StaminaWasTiredAudioThreshold")]
    pub stamina_was_tired_audio_threshold: Option<f64>,

    pub throttle_smooth_acc: Option<f64>,

    pub throttle_smooth_stop: Option<f64>,

    pub throttle_smooth_vertical: Option<f64>,

    pub walk_footstep_length: Option<f64>,

    pub walk_move_speed: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyDofSettingsData {
    pub aperture: Option<f64>,

    pub enabled: Option<bool>,

    pub focal_length: Option<f64>,

    pub focus_distance: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct StickyVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyActionCost {
    pub base_stamina_cost_in_combat: Option<f64>,

    pub base_stamina_cost_out_of_combat: Option<f64>,

    pub reset_resting_timer_in_combat: Option<bool>,

    pub reset_resting_timer_out_of_combat: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfPlayerDialogDataBlock {
    pub blocks: Option<Vec<PlayerDialogDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerDialogDataBlock {
    pub dialog_alternatives: Option<Vec<DialogStructureHolder>>,

    pub drama_filter: Option<DialogDramaFilter>,

    pub priority: Option<Priority>,

    pub probability_to_trigger: Option<ProbabilityToTrigger>,

    pub random_delay: Option<IndigoVector2>,

    pub tension_limit: Option<TensionLimit>,

    pub trigger_interval: Option<TriggerInterval>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DialogStructureHolder {
    pub actor_a: Option<DialogActorSettings>,

    pub actor_b: Option<DialogActorSettings>,

    pub actor_c: Option<DialogActorSettings>,

    pub actor_d: Option<DialogActorSettings>,

    #[serde(rename = "structure")]
    pub structure: Option<DialogStructure>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DialogActorSettings {
    pub char_filter: Option<CharFilter>,

    pub conditions: Option<Vec<ConditionElement>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum CharFilter {
    Enum(DialogCharFilter),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogCharFilter {
    #[serde(rename = "Char_F")]
    CharF,

    #[serde(rename = "Char_G")]
    CharG,

    #[serde(rename = "Char_O")]
    CharO,

    #[serde(rename = "Char_T")]
    CharT,

    None,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConditionElement {
    Enum(DialogCondition),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogCondition {
    #[serde(rename = "HasFlashlight")]
    HasFlashlight,

    #[serde(rename = "HasGlueGun")]
    HasGlueGun,

    #[serde(rename = "HasLockCutter")]
    HasLockCutter,

    #[serde(rename = "HasMapper")]
    HasMapper,

    #[serde(rename = "HasScanner")]
    HasScanner,

    #[serde(rename = "HeavylyArmed")]
    HeavylyArmed,

    #[serde(rename = "IsClose")]
    IsClose,

    #[serde(rename = "IsFarAway")]
    IsFarAway,

    #[serde(rename = "IsVeryClose")]
    IsVeryClose,

    #[serde(rename = "LowAmmo")]
    LowAmmo,

    #[serde(rename = "PlentyAmmo")]
    PlentyAmmo,
}

#[derive(Serialize, Deserialize)]
pub struct DialogStructure {
    pub enabled: Option<bool>,

    pub lines: Option<Vec<DialogLine>>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DialogLine {
    pub a: Option<bool>,

    pub b: Option<bool>,

    pub c: Option<bool>,

    pub d: Option<bool>,

    #[serde(rename = "exit")]
    pub exit: Option<f64>,

    #[serde(rename = "lineEvent")]
    pub line_event: Option<i64>,

    pub subtitle: Option<Subtitle>,

    #[serde(rename = "UNI")]
    pub uni: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Subtitle {
    Integer(i64),

    String(String),

    SubtitleLocalizedText(SubtitleLocalizedText),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SubtitleLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DialogDramaFilter {
    pub alert: Option<bool>,

    pub combat: Option<bool>,

    pub encounter: Option<bool>,

    pub exploration: Option<bool>,

    pub sneaking: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Priority {
    Enum(DialogPriority),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogPriority {
    #[serde(rename = "CombatEvent")]
    CombatEvent,

    #[serde(rename = "IdleTalk")]
    IdleTalk,

    #[serde(rename = "PersonalEvent")]
    PersonalEvent,

    #[serde(rename = "TeamEvent")]
    TeamEvent,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProbabilityToTrigger {
    Enum(DialogProbability),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogProbability {
    Always,

    High,

    Low,

    Medium,

    None,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TensionLimit {
    Enum(DialogTensionLimit),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogTensionLimit {
    High,

    Low,

    Medium,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TriggerInterval {
    Enum(DialogTriggerInterval),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum DialogTriggerInterval {
    #[serde(rename = "LongCooldown")]
    LongCooldown,

    #[serde(rename = "MediumCooldown")]
    MediumCooldown,

    #[serde(rename = "OncePerExpedition")]
    OncePerExpedition,

    #[serde(rename = "OncePerSegment")]
    OncePerSegment,

    #[serde(rename = "ShortCooldown")]
    ShortCooldown,

    Unlimited,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfPlayerOfflineGearDataBlock {
    pub blocks: Option<Vec<PlayerOfflineGearDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerOfflineGearDataBlock {
    #[serde(rename = "GearJSON")]
    pub gear_json: Option<String>,

    #[serde(rename = "Type")]
    pub player_offline_gear_data_block_type: Option<FluffyType>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FluffyType {
    Enum(EOfflineGearType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EOfflineGearType {
    None,

    #[serde(rename = "RundownSpecificInventory")]
    RundownSpecificInventory,

    #[serde(rename = "SpawnedInLevel")]
    SpawnedInLevel,

    #[serde(rename = "StandardInventory")]
    StandardInventory,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfRecepieDataBlock {
    pub blocks: Option<Vec<RecepieDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecepieDataBlock {
    pub andra_vektorn: Option<IndecentVector2>,

    pub objects: Option<Vec<String>>,

    pub santellerfalskt: Option<bool>,

    pub siffran: Option<i64>,

    pub vektorn: Option<IndigoVector3>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct IndigoVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfRecoilDataBlock {
    pub blocks: Option<Vec<RecoilDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecoilDataBlock {
    pub concussion_duration: Option<f64>,

    pub concussion_frequency: Option<f64>,

    pub concussion_intensity: Option<f64>,

    pub dampening: Option<f64>,

    pub directional_similarity: Option<f64>,

    pub hip_fire_crosshair_recoil_pop: Option<f64>,

    pub hip_fire_crosshair_size_default: Option<f64>,

    pub hip_fire_crosshair_size_max: Option<f64>,

    pub horizontal_scale: Option<FluffyMinMaxValue>,

    pub power: Option<FluffyMinMaxValue>,

    pub recoil_aiming_weight: Option<f64>,

    pub recoil_camera_pos_weight: Option<f64>,

    pub recoil_camera_rot_weight: Option<f64>,

    pub recoil_pos_damping: Option<f64>,

    pub recoil_pos_impulse: Option<IndecentVector3>,

    pub recoil_pos_impulse_weight: Option<f64>,

    pub recoil_pos_shift: Option<IndecentVector3>,

    pub recoil_pos_shift_weight: Option<f64>,

    pub recoil_pos_stiffness: Option<f64>,

    pub recoil_rot_damping: Option<f64>,

    pub recoil_rot_impulse: Option<IndecentVector3>,

    pub recoil_rot_impulse_weight: Option<f64>,

    pub recoil_rot_stiffness: Option<f64>,

    pub spring: Option<f64>,

    pub vertical_scale: Option<FluffyMinMaxValue>,

    pub world_to_view_space_blend_horizontal: Option<f64>,

    pub world_to_view_space_blend_vertical: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyMinMaxValue {
    pub max: Option<f64>,

    pub min: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct IndecentVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfRundownDataBlock {
    pub blocks: Option<Vec<RundownDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownDataBlock {
    pub req_to_reach_tier_b: Option<RundownTierProgressionData>,

    pub req_to_reach_tier_c: Option<RundownTierProgressionData>,

    pub req_to_reach_tier_d: Option<RundownTierProgressionData>,

    pub req_to_reach_tier_e: Option<RundownTierProgressionData>,

    pub storytelling_data: Option<RundownStorytellingData>,

    pub tier_a: Option<Vec<ExpeditionInTierData>>,

    pub tier_b: Option<Vec<ExpeditionInTierData>>,

    pub tier_c: Option<Vec<ExpeditionInTierData>>,

    pub tier_d: Option<Vec<ExpeditionInTierData>>,

    pub tier_e: Option<Vec<ExpeditionInTierData>>,

    pub use_tier_unlock_requirements: Option<bool>,

    pub vanity_item_layer_drop_data_block: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownTierProgressionData {
    pub all_cleared_sectors: Option<i64>,

    pub main_sectors: Option<i64>,

    pub secondary_sectors: Option<i64>,

    pub third_sectors: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownStorytellingData {
    pub external_exp_title: Option<ExternalExpTitle>,

    pub surface_description: Option<ExternalExpTitle>,

    pub surface_icon_position: Option<RundownStorytellingDataVector2>,

    pub text_log: Option<ExternalExpTitle>,

    pub text_log_pos: Option<RundownStorytellingDataVector2>,

    pub title: Option<ExternalExpTitle>,

    pub visuals: Option<RundownStorytellingVisualData>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExternalExpTitle {
    ExternalExpTitleLocalizedText(ExternalExpTitleLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExternalExpTitleLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct RundownStorytellingDataVector2 {
    pub x: Option<f64>,

    pub y: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RundownStorytellingVisualData {
    pub color_background: Option<RundownStorytellingVisualDataColor>,

    pub tier_a_visuals: Option<TierVisualData>,

    pub tier_b_visuals: Option<TierVisualData>,

    pub tier_c_visuals: Option<TierVisualData>,

    pub tier_d_visuals: Option<TierVisualData>,

    pub tier_e_visuals: Option<TierVisualData>,
}

#[derive(Serialize, Deserialize)]
pub struct RundownStorytellingVisualDataColor {
    pub a: Option<f64>,

    pub b: Option<f64>,

    pub g: Option<f64>,

    pub r: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TierVisualData {
    pub color: Option<RundownStorytellingVisualDataColor>,

    pub scale: Option<f64>,

    pub scale_y_modifier: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionInTierData {
    pub accessibility: Option<Accessibility>,

    pub build_secondary_from: Option<BuildLayerFromData>,

    pub build_third_from: Option<BuildLayerFromData>,

    pub custom_progression_lock: Option<RundownTierProgressionData>,

    pub descriptive: Option<DescriptiveData>,

    pub dimension_datas: Option<Vec<DimensionInExpeditionData>>,

    pub disable_player_voicelines: Option<bool>,

    pub enabled: Option<bool>,

    pub exclude_from_progression: Option<bool>,

    pub expedition: Option<ExpeditionData>,

    pub gear_picker: Option<GearPicker>,

    pub has_external_style: Option<bool>,

    pub has_story_style: Option<bool>,

    pub is_single_player: Option<bool>,

    pub level_layout_data: Option<i64>,

    pub main_layer_data: Option<LayerData>,

    pub put_icon_above_tier: Option<bool>,

    pub secondary_layer_data: Option<LayerData>,

    pub secondary_layer_enabled: Option<bool>,

    pub secondary_layout: Option<i64>,

    pub seeds: Option<BuildSeedData>,

    pub skip_lobby: Option<bool>,

    pub sound_event_on_warp_to_reality: Option<i64>,

    pub special_override_data: Option<SpecialExpeditionOverridesData>,

    pub third_layer_data: Option<LayerData>,

    pub third_layer_enabled: Option<bool>,

    pub third_layout: Option<i64>,

    pub unlocked_by_expedition: Option<ExpeditionIndex>,

    pub use_gear_picker: Option<bool>,

    pub vanity_items_drop_data: Option<VanityItemsDropData>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Accessibility {
    Enum(EExpeditionAccessibility),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EExpeditionAccessibility {
    #[serde(rename = "AlwayBlock")]
    AlwayBlock,

    #[serde(rename = "AlwaysAllow")]
    AlwaysAllow,

    #[serde(rename = "BlockedAndScrambled")]
    BlockedAndScrambled,

    Normal,

    #[serde(rename = "UnlockedByExpedition")]
    UnlockedByExpedition,

    #[serde(rename = "UseCustomProgressionLock")]
    UseCustomProgressionLock,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildLayerFromData {
    pub layer_type: Option<Layer>,

    pub zone: Option<BuildFromLocalIndexUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DescriptiveData {
    pub custom_matchmaking_tier: Option<Tier>,

    pub dev_info: Option<String>,

    pub estimated_duration: Option<ExternalExpTitle>,

    pub expedition_depth: Option<i64>,

    pub expedition_description: Option<ExternalExpTitle>,

    pub is_extra_expedition: Option<bool>,

    pub prefix: Option<String>,

    pub progression_visual_style: Option<ProgressionVisualStyle>,

    pub public_name: Option<String>,

    pub roleplayed_warden_intel: Option<ExternalExpTitle>,

    pub skip_exp_number_in_name: Option<bool>,

    pub use_custom_matchmaking_tier: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Tier {
    Enum(ERundownTier),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ERundownTier {
    Surface,

    #[serde(rename = "TierA")]
    TierA,

    #[serde(rename = "TierB")]
    TierB,

    #[serde(rename = "TierC")]
    TierC,

    #[serde(rename = "TierD")]
    TierD,

    #[serde(rename = "TierE")]
    TierE,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressionVisualStyle {
    Enum(EProgressionVisualStyle),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EProgressionVisualStyle {
    External,

    #[serde(rename = "ExternalStory")]
    ExternalStory,

    Normal,

    Story,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DimensionInExpeditionData {
    pub dimension_data: Option<i64>,

    pub dimension_index: Option<DimensionIndex>,

    pub enabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionData {
    pub complex_resource_data: Option<i64>,

    pub dust_color: Option<RundownStorytellingVisualDataColor>,

    pub dust_turbulence: Option<f64>,

    pub enemy_population: Option<i64>,

    pub environment_wetness: Option<f64>,

    pub expedition_balance: Option<i64>,

    pub fog_settings: Option<i64>,

    pub light_settings: Option<i64>,

    #[serde(rename = "MLSLevelKit")]
    pub mls_level_kit: Option<i64>,

    pub scout_wave_population: Option<i64>,

    pub scout_wave_settings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GearPicker {
    pub melee_gear: Option<i64>,

    pub special_gear: Option<i64>,

    pub standard_gear: Option<i64>,

    pub tool_gear: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LayerData {
    pub artifact_data: Option<ArtifactLayerData>,

    pub bulkhead_door_controller_placements: Option<Vec<BulkheadDoorPlacementData>>,

    pub bulkhead_key_placements: Option<Vec<Vec<BulkheadKeyPlacementElement>>>,

    pub chained_objective_data: Option<Vec<WardenObjectiveLayerData>>,

    pub objective_data: Option<WardenObjectiveLayerData>,

    pub zones_with_bulkhead_entrance: Option<Vec<ZonesWithBulkheadEntranceElement>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactLayerData {
    pub artifact_amount_multi: Option<f64>,

    #[serde(rename = "ArtifactLayerDistributionDataID")]
    pub artifact_layer_distribution_data_id: Option<i64>,

    pub artifact_zone_distributions: Option<Vec<ArtifactZoneDistribution>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ArtifactZoneDistribution {
    pub advanced_artifact_weight: Option<f64>,

    pub basic_artifact_weight: Option<f64>,

    pub specialized_artifact_weight: Option<f64>,

    pub zone: Option<BuildFromLocalIndexUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkheadDoorPlacementData {
    pub area_seed_offset: Option<i64>,

    pub marker_seed_offset: Option<i64>,

    pub placement_weights: Option<BulkheadDoorControllerPlacementZonePlacementWeights>,

    pub zone_index: Option<BuildFromLocalIndexUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkheadDoorControllerPlacementZonePlacementWeights {
    pub end: Option<f64>,

    pub middle: Option<f64>,

    pub start: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkheadKeyPlacementElement {
    pub dimension_index: Option<DimensionIndex>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub weights: Option<BulkheadDoorControllerPlacementZonePlacementWeights>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WardenObjectiveLayerData {
    pub data_block_id: Option<i64>,

    pub win_condition: Option<WinCondition>,

    pub zone_placement_datas: Option<Vec<Vec<BulkheadKeyPlacementElement>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WinCondition {
    Enum(EWardenObjectiveWinCondition),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWardenObjectiveWinCondition {
    #[serde(rename = "GoToElevator")]
    GoToElevator,

    #[serde(rename = "GoToExitGeo")]
    GoToExitGeo,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ZonesWithBulkheadEntranceElement {
    Enum(ELocalZoneIndex),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BuildSeedData {
    pub build_seed: Option<i64>,

    pub function_marker_offset: Option<i64>,

    pub light_job_seed_offset: Option<i64>,

    pub standard_marker_offset: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialExpeditionOverridesData {
    pub health_level_at_expedition_start: Option<f64>,

    pub infection_level_at_expedition_start: Option<f64>,

    pub special_ammo_at_expedition_start: Option<f64>,

    pub standard_ammo_at_expedition_start: Option<f64>,

    pub tool_ammo_at_expedition_start: Option<f64>,

    pub weak_resource_container_with_pack_chance_for_locked: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpeditionIndex {
    pub exp: Option<Exp>,

    pub tier: Option<Tier>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Exp {
    Enum(EBuildAutoExpeditionNumber),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EBuildAutoExpeditionNumber {
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
pub struct VanityItemsDropData {
    pub groups: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfServiceMarkerDataBlock {
    pub blocks: Option<Vec<ServiceMarkerDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceMarkerDataBlock {
    pub common_data: Option<FluffyMarkerDataCommon>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FluffyMarkerDataCommon {
    pub asset_bundle_name: Option<BundleName>,

    pub bounding_volume: Option<f64>,

    pub compositions: Option<Vec<FluffyMarkerComposition>>,

    pub editor_mesh: Option<String>,

    pub function_potential: Option<f64>,

    pub group: Option<i64>,

    pub rotation_noise: Option<f64>,

    pub rotation_snap: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyMarkerComposition {
    pub function: Option<Function>,

    pub prefab: Option<String>,

    #[serde(rename = "Shard")]
    pub shard: Option<Shard>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfStaticSpawnDataBlock {
    pub blocks: Option<Vec<StaticSpawnDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticSpawnDataBlock {
    pub cluster_chance: Option<f64>,

    pub max_height_from_nodes: Option<f64>,

    pub max_scale: Option<f64>,

    pub min_height_from_nodes: Option<f64>,

    pub min_scale: Option<f64>,

    pub on: Option<bool>,

    pub on_node_normal_dot_limit: Option<f64>,

    pub on_node_rotate_with_surface: Option<bool>,

    pub on_node_spawn_chance: Option<f64>,

    pub path: Option<String>,

    #[serde(rename = "Path_OnNode")]
    pub path_on_node: Option<String>,

    pub placement_type: Option<PlacementType>,

    pub prefab_distance_pairs: Option<Vec<StaticSpawnPrefabDistancePair>>,

    pub push_out_scale_along_normal: Option<f64>,

    pub radius_to_avoid_masked: Option<f64>,

    pub random_type: Option<RandomType>,

    pub register_for_node_update: Option<bool>,

    pub spawn_constraints: Option<SpawnConstraints>,

    pub spread_scale: Option<f64>,

    pub use_prefabs_based_on_distane: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlacementType {
    Enum(LgStaticPlacementType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgStaticPlacementType {
    #[serde(rename = "OnNodes_Ceiling_UpOnly")]
    OnNodesCeilingUpOnly,

    #[serde(rename = "OnNodes_WallsAndCeiling")]
    OnNodesWallsAndCeiling,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticSpawnPrefabDistancePair {
    pub distance_max: Option<f64>,

    pub distance_min: Option<f64>,

    pub distance_prefab_on_node_path: Option<String>,

    pub distance_prefab_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RandomType {
    Enum(LgRandomType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgRandomType {
    #[serde(rename = "BuildSeed")]
    BuildSeed,

    #[serde(rename = "FixedSeed")]
    FixedSeed,

    #[serde(rename = "SessionSeed")]
    SessionSeed,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpawnConstraints {
    Enum(LgStaticSpawnConstraints),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum LgStaticSpawnConstraints {
    #[serde(rename = "AlwaysAndOnlySpawnInRespawnerZones")]
    AlwaysAndOnlySpawnInRespawnerZones,

    None,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfSurvivalWavePopulationDataBlock {
    pub blocks: Option<Vec<SurvivalWavePopulationDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SurvivalWavePopulationDataBlock {
    pub wave_role_boss: Option<i64>,

    pub wave_role_mini_boss: Option<i64>,

    pub wave_role_special: Option<i64>,

    pub wave_role_standard: Option<i64>,

    pub wave_role_weakling: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfSurvivalWaveSettingsDataBlock {
    pub blocks: Option<Vec<SurvivalWaveSettingsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
pub struct SurvivalWaveSettingsDataBlock {
    #[serde(rename = "m_chanceToRandomizeSpawnDirectionPerGroup")]
    pub m_chance_to_randomize_spawn_direction_per_group: Option<f64>,

    #[serde(rename = "m_chanceToRandomizeSpawnDirectionPerWave")]
    pub m_chance_to_randomize_spawn_direction_per_wave: Option<f64>,

    #[serde(rename = "m_filterType")]
    pub m_filter_type: Option<MFilterType>,

    #[serde(rename = "m_overrideWaveSpawnType")]
    pub m_override_wave_spawn_type: Option<bool>,

    #[serde(rename = "m_pauseBeforeStart")]
    pub m_pause_before_start: Option<f64>,

    #[serde(rename = "m_pauseBetweenGroups")]
    pub m_pause_between_groups: Option<f64>,

    #[serde(rename = "m_populationFilter")]
    pub m_population_filter: Option<Vec<MPopulationFilterElement>>,

    #[serde(rename = "m_populationPointsMinPerGroup")]
    pub m_population_points_min_per_group: Option<f64>,

    #[serde(rename = "m_populationPointsPerGroupEnd")]
    pub m_population_points_per_group_end: Option<f64>,

    #[serde(rename = "m_populationPointsPerGroupStart")]
    pub m_population_points_per_group_start: Option<f64>,

    #[serde(rename = "m_populationPointsPerWaveEnd")]
    pub m_population_points_per_wave_end: Option<f64>,

    #[serde(rename = "m_populationPointsPerWaveStart")]
    pub m_population_points_per_wave_start: Option<f64>,

    #[serde(rename = "m_populationPointsTotal")]
    pub m_population_points_total: Option<f64>,

    #[serde(rename = "m_populationRampOverTime")]
    pub m_population_ramp_over_time: Option<f64>,

    #[serde(rename = "m_survivalWaveSpawnType")]
    pub m_survival_wave_spawn_type: Option<WaveSpawnType>,

    #[serde(rename = "m_wavePauseMax")]
    pub m_wave_pause_max: Option<f64>,

    #[serde(rename = "m_wavePauseMax_atCost")]
    pub m_wave_pause_max_at_cost: Option<f64>,

    #[serde(rename = "m_wavePauseMin")]
    pub m_wave_pause_min: Option<f64>,

    #[serde(rename = "m_wavePauseMin_atCost")]
    pub m_wave_pause_min_at_cost: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MFilterType {
    Enum(EEnemyFilterType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EEnemyFilterType {
    Exclude,

    Include,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum MPopulationFilterElement {
    Enum(EEnemyType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WaveSpawnType {
    Enum(SurvivalWaveSpawnType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum SurvivalWaveSpawnType {
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfTechMarkerDataBlock {
    pub blocks: Option<Vec<TechMarkerDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TechMarkerDataBlock {
    pub common_data: Option<TentacledMarkerDataCommon>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TentacledMarkerDataCommon {
    pub asset_bundle_name: Option<BundleName>,

    pub bounding_volume: Option<f64>,

    pub compositions: Option<Vec<TentacledMarkerComposition>>,

    pub editor_mesh: Option<String>,

    pub function_potential: Option<f64>,

    pub group: Option<i64>,

    pub rotation_noise: Option<f64>,

    pub rotation_snap: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct TentacledMarkerComposition {
    pub function: Option<Function>,

    pub prefab: Option<String>,

    #[serde(rename = "Shard")]
    pub shard: Option<Shard>,

    pub weight: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfTextCharacterMetaDataBlock {
    pub blocks: Option<Vec<TextCharacterMetaDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextCharacterMetaDataBlock {
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfTextDataBlock {
    pub blocks: Option<Vec<TextDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TextDataBlock {
    pub character_meta_data: Option<i64>,

    #[serde(rename = "Chinese_Simplified")]
    pub chinese_simplified: Option<LanguageData>,

    #[serde(rename = "Chinese_Traditional")]
    pub chinese_traditional: Option<LanguageData>,

    pub description: Option<String>,

    pub english: Option<String>,

    pub export_version: Option<i64>,

    pub french: Option<LanguageData>,

    pub german: Option<LanguageData>,

    pub import_version: Option<i64>,

    pub italian: Option<LanguageData>,

    pub japanese: Option<LanguageData>,

    pub korean: Option<LanguageData>,

    pub polish: Option<LanguageData>,

    #[serde(rename = "Portuguese_Brazil")]
    pub portuguese_brazil: Option<LanguageData>,

    pub russian: Option<LanguageData>,

    pub skip_localization: Option<bool>,

    pub spanish: Option<LanguageData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LanguageData {
    pub should_translate: Option<bool>,

    pub translation: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfVanityItemsGroupDataBlock {
    pub blocks: Option<Vec<VanityItemsGroupDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VanityItemsGroupDataBlock {
    pub items: Option<Vec<i64>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfVanityItemsLayerDropsDataBlock {
    pub blocks: Option<Vec<VanityItemsLayerDropsDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VanityItemsLayerDropsDataBlock {
    pub layer_drops: Option<Vec<LayerDropData>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LayerDropData {
    pub count: Option<i64>,

    pub groups: Option<Vec<i64>>,

    pub is_all: Option<bool>,

    pub layer: Option<LayerDropLayer>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum LayerDropLayer {
    Enum(ExpeditionLayers),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ExpeditionLayers {
    Main,

    Secondary,

    Third,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfVanityItemsTemplateDataBlock {
    pub blocks: Option<Vec<VanityItemsTemplateDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VanityItemsTemplateDataBlock {
    #[serde(rename = "DropWeight")]
    pub drop_weight: Option<f64>,

    pub icon: Option<String>,

    pub prefab: Option<String>,

    pub public_name: Option<String>,

    #[serde(rename = "type")]
    pub vanity_items_template_data_block_type: Option<TypeUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeUnion {
    Enum(ClothesType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ClothesType {
    Backpack,

    Face,

    Helmet,

    Legs,

    Palette,

    Torso,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfWardenObjectiveDataBlock {
    pub blocks: Option<Vec<WardenObjectiveDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WardenObjectiveDataBlock {
    #[serde(rename = "ActivateHSU_BringItemInElevator")]
    pub activate_hsu_bring_item_in_elevator: Option<bool>,

    #[serde(rename = "ActivateHSU_Events")]
    pub activate_hsu_events: Option<Vec<ActivateHsuEventElement>>,

    #[serde(rename = "ActivateHSU_ItemAfterActivation")]
    pub activate_hsu_item_after_activation: Option<i64>,

    #[serde(rename = "ActivateHSU_ItemFromStart")]
    pub activate_hsu_item_from_start: Option<i64>,

    #[serde(rename = "ActivateHSU_MarkItemInElevatorAsWardenObjective")]
    pub activate_hsu_mark_item_in_elevator_as_warden_objective: Option<bool>,

    #[serde(rename = "ActivateHSU_ObjectiveCompleteAfterInsertion")]
    pub activate_hsu_objective_complete_after_insertion: Option<bool>,

    #[serde(rename = "ActivateHSU_RequireItemAfterActivationInExitScan")]
    pub activate_hsu_require_item_after_activation_in_exit_scan: Option<bool>,

    #[serde(rename = "ActivateHSU_StopEnemyWavesOnActivation")]
    pub activate_hsu_stop_enemy_waves_on_activation: Option<bool>,

    #[serde(rename = "CentralPowerGenClustser_FogDataSteps")]
    pub central_power_gen_clustser_fog_data_steps: Option<Vec<GeneralFogDataStep>>,

    #[serde(rename = "CentralPowerGenClustser_NumberOfGenerators")]
    pub central_power_gen_clustser_number_of_generators: Option<i64>,

    #[serde(rename = "CentralPowerGenClustser_NumberOfPowerCells")]
    pub central_power_gen_clustser_number_of_power_cells: Option<i64>,

    pub chained_puzzle_at_exit: Option<i64>,

    pub chained_puzzle_at_exit_scan_speed_multiplier: Option<f64>,

    pub chained_puzzle_mid_objective: Option<i64>,

    pub chained_puzzle_to_active: Option<i64>,

    pub do_not_mark_pickup_items_as_warden_objectives: Option<bool>,

    pub do_not_solve_objective_on_reactor_complete: Option<bool>,

    pub events_on_activate: Option<Vec<ActivateHsuEventElement>>,

    pub events_on_elevator_land: Option<Vec<ActivateHsuEventElement>>,

    pub events_on_goto_win: Option<Vec<ActivateHsuEventElement>>,

    pub events_on_goto_win_trigger: Option<OnGotoWinTrigger>,

    pub find_location_info: Option<FindLocationInfoUnion>,

    pub find_location_info_help: Option<FindLocationInfoUnion>,

    pub fog_transition_data_on_elevator_land: Option<i64>,

    pub fog_transition_data_on_goto_win: Option<i64>,

    pub fog_transition_duration_on_elevator_land: Option<f64>,

    pub fog_transition_duration_on_goto_win: Option<f64>,

    #[serde(rename = "Gather_ItemId")]
    pub gather_item_id: Option<i64>,

    #[serde(rename = "Gather_MaxPerZone")]
    pub gather_max_per_zone: Option<i64>,

    #[serde(rename = "Gather_RequiredCount")]
    pub gather_required_count: Option<i64>,

    #[serde(rename = "Gather_SpawnCount")]
    pub gather_spawn_count: Option<i64>,

    #[serde(rename = "GatherTerminal_Command")]
    pub gather_terminal_command: Option<String>,

    #[serde(rename = "GatherTerminal_CommandHelp")]
    pub gather_terminal_command_help: Option<FindLocationInfoUnion>,

    #[serde(rename = "GatherTerminal_DownloadCompleteText")]
    pub gather_terminal_download_complete_text: Option<FindLocationInfoUnion>,

    #[serde(rename = "GatherTerminal_DownloadingText")]
    pub gather_terminal_downloading_text: Option<FindLocationInfoUnion>,

    #[serde(rename = "GatherTerminal_DownloadTime")]
    pub gather_terminal_download_time: Option<f64>,

    #[serde(rename = "GatherTerminal_RequiredCount")]
    pub gather_terminal_required_count: Option<i64>,

    #[serde(rename = "GatherTerminal_SpawnCount")]
    pub gather_terminal_spawn_count: Option<i64>,

    pub generic_item_from_start: Option<i64>,

    #[serde(rename = "GoToWinCondition_CustomGeo")]
    pub go_to_win_condition_custom_geo: Option<FindLocationInfoUnion>,

    #[serde(rename = "GoToWinCondition_Elevator")]
    pub go_to_win_condition_elevator: Option<FindLocationInfoUnion>,

    #[serde(rename = "GoToWinCondition_ToMainLayer")]
    pub go_to_win_condition_to_main_layer: Option<FindLocationInfoUnion>,

    #[serde(rename = "GoToWinConditionHelp_CustomGeo")]
    pub go_to_win_condition_help_custom_geo: Option<FindLocationInfoUnion>,

    #[serde(rename = "GoToWinConditionHelp_Elevator")]
    pub go_to_win_condition_help_elevator: Option<FindLocationInfoUnion>,

    #[serde(rename = "GoToWinConditionHelp_ToMainLayer")]
    pub go_to_win_condition_help_to_main_layer: Option<FindLocationInfoUnion>,

    pub go_to_zone: Option<FindLocationInfoUnion>,

    pub go_to_zone_help: Option<FindLocationInfoUnion>,

    pub header: Option<FindLocationInfoUnion>,

    pub in_zone_find_item: Option<FindLocationInfoUnion>,

    pub in_zone_find_item_help: Option<FindLocationInfoUnion>,

    pub lights_on_during_intro: Option<bool>,

    pub lights_on_from_beginning: Option<bool>,

    pub lights_on_when_startup_complete: Option<bool>,

    pub main_objective: Option<FindLocationInfoUnion>,

    pub on_activate_on_solve_item: Option<bool>,

    pub post_command_output: Option<Vec<String>>,

    pub power_cells_to_distribute: Option<i64>,

    pub reactor_waves: Option<Vec<ReactorWaveData>>,

    #[serde(rename = "Retrieve_Items")]
    pub retrieve_items: Option<Vec<i64>>,

    pub show_help_delay: Option<f64>,

    pub solve_item: Option<FindLocationInfoUnion>,

    pub solve_item_help: Option<FindLocationInfoUnion>,

    pub special_command_rule: Option<AlCommandRule>,

    pub special_terminal_command: Option<String>,

    pub special_terminal_command_desc: Option<FindLocationInfoUnion>,

    pub stop_all_waves_before_goto_win: Option<bool>,

    #[serde(rename = "Survival_TimerTitle")]
    pub survival_timer_title: Option<FindLocationInfoUnion>,

    #[serde(rename = "Survival_TimerToActivateTitle")]
    pub survival_timer_to_activate_title: Option<FindLocationInfoUnion>,

    #[serde(rename = "Survival_TimeToActivate")]
    pub survival_time_to_activate: Option<f64>,

    #[serde(rename = "Survival_TimeToSurvive")]
    pub survival_time_to_survive: Option<f64>,

    #[serde(rename = "TimedTerminalSequence_EventsOnSequenceDone")]
    pub timed_terminal_sequence_events_on_sequence_done: Option<Vec<Vec<ActivateHsuEventElement>>>,

    #[serde(rename = "TimedTerminalSequence_EventsOnSequenceFail")]
    pub timed_terminal_sequence_events_on_sequence_fail: Option<Vec<Vec<ActivateHsuEventElement>>>,

    #[serde(rename = "TimedTerminalSequence_EventsOnSequenceStart")]
    pub timed_terminal_sequence_events_on_sequence_start: Option<Vec<Vec<ActivateHsuEventElement>>>,

    #[serde(rename = "TimedTerminalSequence_NumberOfRounds")]
    pub timed_terminal_sequence_number_of_rounds: Option<i64>,

    #[serde(rename = "TimedTerminalSequence_NumberOfTerminals")]
    pub timed_terminal_sequence_number_of_terminals: Option<i64>,

    #[serde(rename = "TimedTerminalSequence_SourceTerminalWorldEventObjectFilter")]
    pub timed_terminal_sequence_source_terminal_world_event_object_filter: Option<String>,

    #[serde(rename = "TimedTerminalSequence_TimeForConfirmation")]
    pub timed_terminal_sequence_time_for_confirmation: Option<f64>,

    #[serde(rename = "TimedTerminalSequence_TimePerRound")]
    pub timed_terminal_sequence_time_per_round: Option<f64>,

    #[serde(rename = "TimedTerminalSequence_UseFilterForSourceTerminalPicking")]
    pub timed_terminal_sequence_use_filter_for_source_terminal_picking: Option<bool>,

    #[serde(rename = "Type")]
    pub warden_objective_data_block_type: Option<TentacledType>,

    #[serde(rename = "Uplink_NumberOfTerminals")]
    pub uplink_number_of_terminals: Option<i64>,

    #[serde(rename = "Uplink_NumberOfVerificationRounds")]
    pub uplink_number_of_verification_rounds: Option<i64>,

    #[serde(rename = "Uplink_WaveSpawnType")]
    pub uplink_wave_spawn_type: Option<WaveSpawnType>,

    pub warden_objective_special_update_type: Option<WardenObjectiveSpecialUpdateType>,

    pub wave_on_elevator_warden_intel: Option<FindLocationInfoUnion>,

    pub wave_on_goto_win_trigger: Option<OnGotoWinTrigger>,

    pub waves_on_activate: Option<Vec<GenericEnemyWaveDataElement>>,

    pub waves_on_elevator_land: Option<Vec<GenericEnemyWaveDataElement>>,

    pub waves_on_goto_win: Option<Vec<GenericEnemyWaveDataElement>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivateHsuEventElement {
    pub chain_puzzle: Option<i64>,

    pub clear_dimension: Option<bool>,

    pub condition: Option<ActivateHsuEventWorldEventConditionPair>,

    pub count: Option<i64>,

    pub custom_sub_objective: Option<FindLocationInfoUnion>,

    pub custom_sub_objective_header: Option<FindLocationInfoUnion>,

    pub delay: Option<f64>,

    #[serde(rename = "DialogueID")]
    pub dialogue_id: Option<i64>,

    pub dimension_index: Option<DimensionIndex>,

    pub duration: Option<f64>,

    pub enabled: Option<bool>,

    #[serde(rename = "EnemyID")]
    pub enemy_id: Option<i64>,

    pub enemy_wave_data: Option<GenericEnemyWaveDataElement>,

    pub fog_setting: Option<i64>,

    pub fog_transition_duration: Option<f64>,

    pub layer: Option<Layer>,

    pub local_index: Option<BuildFromLocalIndexUnion>,

    pub position: Option<ActivateHsuEventVector3>,

    #[serde(rename = "SoundID")]
    pub sound_id: Option<i64>,

    pub sound_subtitle: Option<FindLocationInfoUnion>,

    pub terminal_command: Option<TerminalCommand>,

    pub terminal_command_rule: Option<AlCommandRule>,

    pub trigger: Option<EventsOnBossDeathTrigger>,

    #[serde(rename = "Type")]
    pub warden_objective_event_data_type: Option<EventsOnBossDeathType>,

    pub use_static_bioscan_points: Option<bool>,

    pub warden_intel: Option<FindLocationInfoUnion>,

    pub world_event_object_filter: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ActivateHsuEventWorldEventConditionPair {
    pub condition_index: Option<i64>,

    pub is_true: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum FindLocationInfoUnion {
    FindLocationInfoLocalizedText(FindLocationInfoLocalizedText),

    Integer(i64),

    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FindLocationInfoLocalizedText {
    pub has_translation: Option<bool>,

    pub has_value: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GenericEnemyWaveDataElement {
    pub area_distance: Option<i64>,

    pub intel_message: Option<FindLocationInfoUnion>,

    pub spawn_delay: Option<f64>,

    pub trigger_alarm: Option<bool>,

    pub wave_population: Option<i64>,

    pub wave_settings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct ActivateHsuEventVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct GeneralFogDataStep {
    #[serde(rename = "m_fogDataId")]
    pub m_fog_data_id: Option<i64>,

    #[serde(rename = "m_transitionToTime")]
    pub m_transition_to_time: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum OnGotoWinTrigger {
    Enum(ERetrieveExitWaveTrigger),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ERetrieveExitWaveTrigger {
    #[serde(rename = "OnObjectiveCompleted")]
    OnObjectiveCompleted,

    #[serde(rename = "WhenExitScanMakesProgress")]
    WhenExitScanMakesProgress,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReactorWaveData {
    pub enemy_waves: Option<Vec<ReactorWaveEnemyData>>,

    pub events: Option<Vec<ActivateHsuEventElement>>,

    pub verify: Option<f64>,

    pub verify_fail: Option<f64>,

    pub verify_in_other_zone: Option<bool>,

    pub warmup: Option<f64>,

    pub warmup_fail: Option<f64>,

    pub wave: Option<f64>,

    pub zone_for_verification: Option<BuildFromLocalIndexUnion>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReactorWaveEnemyData {
    pub area_distance: Option<i64>,

    pub spawn_time_rel: Option<f64>,

    pub spawn_type: Option<SpawnType>,

    pub wave_population: Option<i64>,

    pub wave_settings: Option<i64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum SpawnType {
    Enum(EReactorWaveSpawnType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EReactorWaveSpawnType {
    #[serde(rename = "ClosestToReactorNoPlayerBetween")]
    ClosestToReactorNoPlayerBetween,

    #[serde(rename = "InElevatorZone")]
    InElevatorZone,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum TentacledType {
    Enum(EWardenObjectiveType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWardenObjectiveType {
    #[serde(rename = "ActivateSmallHSU")]
    ActivateSmallHsu,

    #[serde(rename = "CentralGeneratorCluster")]
    CentralGeneratorCluster,

    #[serde(rename = "ClearAPath")]
    ClearAPath,

    #[serde(rename = "CorruptedTerminalUplink")]
    CorruptedTerminalUplink,

    Empty,

    #[serde(rename = "GatherSmallItems")]
    GatherSmallItems,

    #[serde(rename = "GatherTerminal")]
    GatherTerminal,

    #[serde(rename = "HSU_FindTakeSample")]
    HsuFindTakeSample,

    #[serde(rename = "PowerCellDistribution")]
    PowerCellDistribution,

    #[serde(rename = "Reactor_Shutdown")]
    ReactorShutdown,

    #[serde(rename = "Reactor_Startup")]
    ReactorStartup,

    #[serde(rename = "RetrieveBigItems")]
    RetrieveBigItems,

    #[serde(rename = "SpecialTerminalCommand")]
    SpecialTerminalCommand,

    Survival,

    #[serde(rename = "TerminalUplink")]
    TerminalUplink,

    #[serde(rename = "TimedTerminalSequence")]
    TimedTerminalSequence,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum WardenObjectiveSpecialUpdateType {
    Enum(EWardenObjectiveSpecialUpdateType),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum EWardenObjectiveSpecialUpdateType {
    #[serde(rename = "IgnoreFoundObjectiveItem")]
    IgnoreFoundObjectiveItem,

    None,

    #[serde(rename = "TriggerFoundObjectiveItemOnlyOnce")]
    TriggerFoundObjectiveItemOnlyOnce,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfWeaponAudioDataBlock {
    pub blocks: Option<Vec<WeaponAudioDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponAudioDataBlock {
    pub event_click: Option<String>,

    pub event_equip: Option<String>,

    pub event_on_auto_fire_end2_d: Option<Vec<String>>,

    pub event_on_auto_fire_end3_d: Option<Vec<String>>,

    pub event_on_auto_fire_start2_d: Option<Vec<String>>,

    pub event_on_auto_fire_start3_d: Option<Vec<String>>,

    pub event_on_burst_fire2_d: Option<Vec<String>>,

    pub event_on_burst_fire3_d: Option<Vec<String>>,

    pub event_on_burst_fire_one_shot2_d: Option<Vec<String>>,

    pub event_on_chargeup2_d: Option<Vec<String>>,

    pub event_on_chargeup3_d: Option<Vec<String>>,

    pub event_on_chargeup_end2_d: Option<String>,

    pub event_on_chargeup_end3_d: Option<String>,

    pub event_on_cooldown2_d: Option<Vec<String>>,

    pub event_on_cooldown3_d: Option<Vec<String>>,

    pub event_on_cooldown_end2_d: Option<String>,

    pub event_on_cooldown_end3_d: Option<String>,

    pub event_on_semi_fire2_d: Option<Vec<String>>,

    pub event_on_semi_fire3_d: Option<Vec<String>>,

    pub event_on_synced_auto_fire_per_shot3_d: Option<Vec<String>>,

    pub event_on_synced_burst_fire_per_shot3_d: Option<Vec<String>>,

    pub event_reload: Option<String>,

    pub event_zoom_in: Option<String>,

    pub event_zoom_out: Option<String>,

    #[serde(rename = "TriggerAutoAudioForEachShot")]
    pub trigger_auto_audio_for_each_shot: Option<bool>,

    #[serde(rename = "TriggerBurstAudioForEachShot")]
    pub trigger_burst_audio_for_each_shot: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfWeaponDataBlock {
    pub blocks: Option<Vec<WeaponDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponDataBlock {
    pub clip_size: Option<i64>,

    #[serde(rename = "EffectiveDamagePerSecond")]
    pub effective_damage_per_second: Option<String>,

    pub event_click: Option<String>,

    pub event_on_auto_fire_end2_d: Option<Vec<String>>,

    pub event_on_auto_fire_end3_d: Option<Vec<String>>,

    pub event_on_auto_fire_start2_d: Option<Vec<String>>,

    pub event_on_auto_fire_start3_d: Option<Vec<String>>,

    pub event_on_burst_fire2_d: Option<Vec<String>>,

    pub event_on_burst_fire3_d: Option<Vec<String>>,

    pub event_on_semi_fire2_d: Option<Vec<String>>,

    pub event_on_semi_fire3_d: Option<Vec<String>>,

    pub event_on_special_auto_fire_end2_d: Option<Vec<String>>,

    pub event_on_special_auto_fire_end3_d: Option<Vec<String>>,

    pub event_on_special_auto_fire_start2_d: Option<Vec<String>>,

    pub event_on_special_auto_fire_start3_d: Option<Vec<String>>,

    pub event_on_special_burst_fire2_d: Option<Vec<String>>,

    pub event_on_special_burst_fire3_d: Option<Vec<String>>,

    pub event_on_special_chargeup2_d: Option<Vec<String>>,

    pub event_on_special_chargeup3_d: Option<Vec<String>>,

    pub event_on_special_chargeup_end2_d: Option<String>,

    pub event_on_special_chargeup_end3_d: Option<String>,

    pub event_on_special_cooldown2_d: Option<Vec<String>>,

    pub event_on_special_cooldown3_d: Option<Vec<String>>,

    pub event_on_special_cooldown_end2_d: Option<String>,

    pub event_on_special_cooldown_end3_d: Option<String>,

    pub event_on_special_semi_fire2_d: Option<Vec<String>>,

    pub event_on_special_semi_fire3_d: Option<Vec<String>>,

    pub event_reload: Option<String>,

    pub has_rotating_cylinder: Option<bool>,

    pub muzzle_feedback: Option<String>,

    #[serde(rename = "muzzleFlash3RD")]
    pub muzzle_flash3_rd: Option<String>,

    #[serde(rename = "muzzleFlash3RDSpecial")]
    pub muzzle_flash3_rd_special: Option<String>,

    #[serde(rename = "muzzleFlashFPS")]
    pub muzzle_flash_fps: Option<String>,

    #[serde(rename = "muzzleFlashFPSSpecial")]
    pub muzzle_flash_fps_special: Option<String>,

    pub recoil_aiming_weight: Option<f64>,

    pub recoil_camera_pos_weight: Option<f64>,

    pub recoil_camera_rot_weight: Option<f64>,

    pub recoil_pos_damping: Option<f64>,

    pub recoil_pos_impulse: Option<HilariousVector3>,

    pub recoil_pos_impulse_weight: Option<f64>,

    pub recoil_pos_shift: Option<HilariousVector3>,

    pub recoil_pos_shift_weight: Option<f64>,

    pub recoil_pos_stiffness: Option<f64>,

    pub recoil_rot_damping: Option<f64>,

    pub recoil_rot_impulse: Option<HilariousVector3>,

    pub recoil_rot_impulse_weight: Option<f64>,

    pub recoil_rot_stiffness: Option<f64>,

    pub reload_duration: Option<f64>,

    pub shell_eject_feedback: Option<String>,

    pub shell_type: Option<ShellType>,

    pub use_semi_audio_for_burst_shots: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct HilariousVector3 {
    pub x: Option<f64>,

    pub y: Option<f64>,

    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ShellType {
    Enum(ShellTypes),

    Integer(i64),
}

#[derive(Serialize, Deserialize)]
pub enum ShellTypes {
    #[serde(rename = "Shell_12_Gauge")]
    Shell12_Gauge,

    #[serde(rename = "Shell_338")]
    Shell338,

    #[serde(rename = "Shell_45_ACP")]
    Shell45_Acp,

    #[serde(rename = "Shell_9mm")]
    Shell9Mm,

    #[serde(rename = "Shell_None")]
    ShellNone,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfWeaponMuzzleFlashDataBlock {
    pub blocks: Option<Vec<WeaponMuzzleFlashDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponMuzzleFlashDataBlock {
    #[serde(rename = "AssetBundleName")]
    pub asset_bundle_name: Option<BundleName>,

    #[serde(rename = "AssetBundleShard")]
    pub asset_bundle_shard: Option<Shard>,

    #[serde(rename = "muzzleFlash3RD")]
    pub muzzle_flash3_rd: Option<String>,

    #[serde(rename = "muzzleFlashFPS")]
    pub muzzle_flash_fps: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameDataBlockWrapperOfWeaponShellCasingDataBlock {
    pub blocks: Option<Vec<WeaponShellCasingDataBlock>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeaponShellCasingDataBlock {
    pub shell_casing_type: Option<ShellType>,
}
