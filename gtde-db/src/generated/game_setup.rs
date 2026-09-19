use serde::{Deserialize, Serialize};

use crate::{datablocks::reference::Reference, generated::rundown::Rundown};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameSetup {
    #[serde(rename = "RundownIdsToLoad")]
    pub rundown_ids_to_load: Vec<Reference<Rundown>>,
    pub startup_screen_to_load: StartupScreenToLoad,
}

#[derive(Serialize, Deserialize)]
pub enum StartupScreenToLoad {
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
