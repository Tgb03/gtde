use std::{fmt::Display, fs, path::Path};

use clap::ValueEnum;
use gtde_error::error::Error;
use gtde_file::file_utils;
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};

use crate::{config::Config, manifest::Manifest};

static PROJECT_SCHEMAS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../resources/.schemas");

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum DatablockEnum {
    Archetype,
    Artifact,
    ArtifactDistribution,
    ArtifactTag,
    Atmosphere,
    BigPickupDistribution,
    BoosterImplantCondition,
    BoosterImplantEffect,
    BoosterImplantTemplate,
    ChainedPuzzle,
    ChainedPuzzleType,
    Clouds,
    Commodity,
    ComplexResourceSet,
    Consumable,
    CustomAssetShard,
    Dimension,
    EffectNode,
    EnemyBalancing,
    EnemyBehavior,
    Enemy,
    EnemyDetection,
    EnemyGroup,
    EnemyMovement,
    EnemyPopulation,
    EnemySFX,
    EnvironmentFeedback,
    EventSequenceAction,
    EventSequence,
    ExpeditionBalance,
    ExtractionEvent,
    FeedbackSettings,
    FlashlightSettings,
    FogScenario,
    FogSettings,
    GameplayTrailer,
    GameSetup,
    GearCategory,
    GearCategoryFilter,
    Gear,
    GearDecal,
    GearFlashlightPart,
    GearFrontPart,
    GearMagPart,
    GearMeleeHandlePart,
    GearMeleeHeadPart,
    GearMeleeNeckPart,
    GearMeleePommelPart,
    GearPalette,
    GearPartAttachment,
    GearPattern,
    GearPerk,
    GearReceiverPart,
    GearSightPart,
    GearStockPart,
    Item,
    ItemFPSSettings,
    ItemMovementAnimation,
    ItemPart,
    LevelGenSettings,
    LevelLayout,
    LightSettings,
    Loot,
    MarkerGroup,
    MeleeAnimationSet,
    MeleeArchetype,
    MeleeSFX,
    MiningMarker,
    MLSArrayDescriptorReference,
    MusicState,
    Player,
    PlayerDialog,
    PlayerOfflineGear,
    Recepie,
    Rundown,
    ServiceMarker,
    StaticSpawn,
    SurvivalWavePopulation,
    SurvivalWaveSettings,
    TechMarker,
    TextCharacterMeta,
    Text,
    VanityItemsGroup,
    VanityItemsLayerDrops,
    VanityItemsTemplate,
    WardenObjective,
    Weapon,
    WeaponMuzzleFlash,
    WeaponShellCasing,
}

impl Display for DatablockEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameData_{:?}DataBlock_bin.json", self)
    }
}

pub fn init<'a>(path: &'a Path) -> Result<(), Error> {
    fs::create_dir_all(path.join("Assets")).map_err(Error::io_at(path.join("Assets")))?;
    fs::create_dir_all(path.join("config")).map_err(Error::io_at(path.join("config")))?;
    fs::create_dir_all(path.join("plugins")).map_err(Error::io_at(path.join("plugins")))?;
    fs::create_dir_all(path.join("Custom")).map_err(Error::io_at(path.join("Custom")))?;
    fs::create_dir_all(path.join("gtde-create")).map_err(Error::io_at(path.join("gtde-create")))?;
    fs::create_dir_all(path.join(".schemas")).map_err(Error::io_at(path.join(".schemas")))?;
    file_utils::create_file_if_doesnt_exist(path, "CHANGELOG.md", "")?;
    file_utils::create_file_if_doesnt_exist(path, "README.md", "")?;
    file_utils::create_file_if_doesnt_exist(
        path,
        "manifest.json",
        serde_json::to_string_pretty(&Manifest::default()).unwrap_or_default(),
    )?;

    let config_data = serde_json::to_string_pretty(&Config::default()).unwrap_or_default();
    file_utils::create_file_if_doesnt_exist(path, "gtde.config", config_data)?;
    initialize_schemas(path)?;

    Ok(())
}

pub fn initialize_schemas(path: impl AsRef<Path>) -> Result<(), Error> {
    let path = path.as_ref().join(".schemas");
    PROJECT_SCHEMAS
        .extract(&path)
        .map_err(Error::io_at(&path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::init::init;
    use tempfile::tempdir;

    #[test]
    fn test_simple_init() {
        let temp = tempdir().unwrap();
        assert!(init(temp.path()).is_ok());
    }
}
