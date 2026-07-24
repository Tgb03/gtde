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

#[repr(u32)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, JsonSchema)]
pub enum EEnemyName {
    ShooterWave = 11,
    #[default]
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
