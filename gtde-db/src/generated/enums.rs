use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum EEnemyType {
    Boss,
    #[serde(rename = "MiniBoss")]
    MiniBoss,
    Special,
    Standard,
    Weakling,
}

#[repr(i64)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, JsonSchema)]
pub enum EEnemyName {
    #[default]
    None = -1,
    ShooterWave = 11,
    StrikerWave = 13,
    StrikerBigWave = 16,
    ShooterBig = 18,
    StrikerBoss = 19,
    Scout = 20,
    Shadow = 21,
    Cocoon = 22,
    StrikerHibernate = 24,
    ShooterHibernate = 26,
    StrikerBigHibernate = 28,
    Tank = 29,
    StrikerBullrush = 30,
    StrikerWaveFast = 31,
    StrikerPatrol = 32,
    ShooterBigRapidFire = 33,
    ShooterBigInfection = 34,
    StrikerBigShadow = 35,
    Birther = 36,
    BirtherBoss = 37,
    StrikerChild = 38,
    StrikerBigBullrush = 39,
    ScoutShadow = 40,
    ScoutBullrush = 41,
    Flyer = 42,
    Squidward = 43,
    SquidBossBig = 44,
    FlyerBig = 45,
    Pouncer = 46,
    TankBoss = 47,
    ShooterSpread = 52,
    StrikerBerserk = 53,
    ScoutZoomer = 54,
    MegaMother = 55,
    ScoutNightmare = 56,
    SquidBossBigComplex = 58,
    SquidBossVS = 61,
    StrikerBigNightmare = 62,
    StrikerChildNightmare = 63,
}

impl Into<u32> for EEnemyName {
    fn into(self) -> u32 {
        self as u32
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
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
