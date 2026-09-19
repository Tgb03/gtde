use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{datablocks::reference::Reference, generated::{EnumWrapper, UndefinedReferenceItem, chained_puzzle::ChainedPuzzle, enums::EEnemyName, rundown::{DimensionIndex, LgLayerType, LocalZoneIndex}, survival_wave_population::SurvivalWavePopulation, survival_wave_settings::SurvivalWaveSettings, text::LocalizedText, vector::Vector3}};


#[repr(i32)]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WardenObjectiveEventType {
    #[default] None = 0,
    OpenSecurityDoor = 1,
    UnlockSecurityDoor = 2,
    AllLightsOff = 3,
    AllLightsOn = 4,
    PlaySound = 5,
    SetFogSetting = 6,
    DimensionFlashTeam = 7,
    DimensionWarpTeam = 8,
    SpawnEnemyWave = 9,
    StopEnemyWaves = 10,
    UpdateCustomSubObjective = 11,
    ForceCompleteObjective = 12,
    LightsInZone = 13,
    LightsInZoneToggle = 14,
    AnimationTrigger = 15,
    SpawnEnemyOnPoint = 16,
    SetNavMarker = 17,
    StepProgressionObjective = 18,
    SetWorldEventCondition = 19,
    LockSecurityDoor = 20,
    SetTerminalCommand = 21,
    ActivateChainedPuzzle = 22,
    LightOnWorldEventObject = 23,
    AddToTimer = 24,
    ResetTimer = 25,
    WinOnDeath = 26,
    ForceInstantWin = 27,
    DialogueOnClosest = 28,
    GetAchievement = 29,
    ClearDimension = 30,
    StartRepeatingFog = 31,
    StopSustainedEvent = 32,
    EventBreak = 999
}

#[derive(Serialize, Default, Deserialize)]
#[serde(default)]
#[serde(rename_all = "PascalCase")]
pub struct WardenObjectiveEvent {
    #[serde(rename = "Type")]
    pub warden_objective_event_data_type: EnumWrapper<WardenObjectiveEventType>,
    pub chain_puzzle: Reference<ChainedPuzzle>,
    pub clear_dimension: bool,
    pub condition: WorldEventConditionPair,
    pub count: i64,
    pub custom_sub_objective: LocalizedText,
    pub custom_sub_objective_header: LocalizedText,
    pub delay: f64,
    #[serde(rename = "DialogueID")]
    pub dialogue_id: Reference<UndefinedReferenceItem>,
    pub dimension_index: EnumWrapper<DimensionIndex>,
    pub duration: f64,
    pub enabled: bool,
    #[serde(rename = "EnemyID")]
    pub enemy_id: EnumWrapper<EEnemyName>,
    pub enemy_wave_data: GenericEnemyWaveData,
    pub fog_setting: Reference<UndefinedReferenceItem>,
    pub fog_transition_duration: f64,
    pub layer: EnumWrapper<LgLayerType>,
    pub local_index: EnumWrapper<LocalZoneIndex>,
    pub position: Vector3<f64>,
    #[serde(rename = "SoundID")]
    pub sound_id: Reference<UndefinedReferenceItem>,
    pub sound_subtitle: LocalizedText,
    pub terminal_command: EnumWrapper<TerminalCommand>,
    pub terminal_command_rule: EnumWrapper<TermCommandRule>,
    pub trigger: EnumWrapper<WardenObjectiveEventTrigger>,
    pub use_static_bioscan_points: bool,
    pub warden_intel: LocalizedText,
    pub world_event_object_filter: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct WorldEventConditionPair {
    pub condition_index: i64,
    pub is_true: bool,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GenericEnemyWaveData {
    pub intel_message: LocalizedText,
    pub spawn_delay: f64,
    pub trigger_alarm: bool,
    pub area_distance: i64,
    pub wave_population: Reference<SurvivalWavePopulation>,
    pub wave_settings: Reference<SurvivalWaveSettings>,
}

#[repr(i32)]
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TerminalCommand {
    Activate,
    ActivateBeacon,
    Close,
    Cls,
    Commands,
    Deactivate,
    DisableAlarm,
    DownloadData,
    EmptyLine,
    Exit,
    Find,
    Help,
    Info,
    InvalidCommand,
    ListLogs,
    Locate,
    #[serde(rename = "MAX_COUNT")]
    MaxCount,
    #[default] None,
    Open,
    Override,
    Ping,
    Query,
    ReactorShutdown,
    ReactorStartup,
    ReactorVerify,
    ReadLog,
    ShowList,
    Start,
    TerminalCorruptedUplinkConnect,
    TerminalCorruptedUplinkVerify,
    TerminalUplinkConfirm,
    TerminalUplinkConnect,
    TerminalUplinkVerify,
    TimedConnectionSend,
    TimedConnectionVerify,
    TryUnlockingTerminal,
    UniqueCommand1,
    UniqueCommand2,
    UniqueCommand3,
    UniqueCommand4,
    UniqueCommand5,
    UsedCommand,
    ViewSecurityLog,
    WardenObjectiveGatherCommand,
    WardenObjectiveSpecialCommand,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TermCommandRule {
    #[default] Normal,
    OnlyOnce,
    OnlyOnceDelete,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WardenObjectiveEventTrigger {
    #[default] None,
    OnStart,
    OnMid,
    OnEnd,
}
