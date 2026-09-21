use std::{fs, path::Path};

use crate::commands::{grab_db::grab_db, init::{self, DatablockEnum}};
use gtde_db::{datablocks::datablock_wrapper::DatablockWrapper, generated::game_setup::{GameSetup, StartupScreenToLoad::StartupScreenData3}};
use gtde_error::error::Error;

pub fn new<'a>(name: String, path: &'a Path, minimum_viable: bool) -> Result<(), Error> {
    let path = path.join(&name);
    fs::create_dir_all(&path).map_err(Error::io_at(&path))?;

    init::init(&path)?;

    if minimum_viable {
        init_minimum(path)?;
    }

    Ok(())
}

fn init_minimum(path: impl AsRef<Path>) -> Result<(), Error> {
    init_game_setup(&path)?;
    init_rundown(&path)?;
    init_level_layout(&path)?;

    Ok(())
}

fn init_game_setup(path: impl AsRef<Path>) -> Result<(), Error> {
    let plugins_path = path.as_ref().join("plugins");
    
    let mut game_setup = DatablockWrapper::<GameSetup>::default();
    game_setup.insert(GameSetup {
        rundown_ids_to_load: vec![1u32.into()],
        startup_screen_to_load: StartupScreenData3,
    }, "Default".into(), 1);
    
    game_setup.save_datablock(&plugins_path, "GameSetupDataBlock.json", "GameData_GameSetupDataBlock_bin")
}

fn init_rundown(path: impl AsRef<Path>) -> Result<(), Error> {
    let plugins_path = path.as_ref().join("plugins");
    
    grab_db(&path, DatablockEnum::Rundown)?;
    let rundowns: DatablockWrapper<serde_json::Value> = DatablockWrapper::<serde_json::Value>::load_datablock(&plugins_path, "GameData_RundownDataBlock_bin")?
        .into_iter()
        .filter(|bw| bw.persistent_id == 39 || bw.persistent_id == 32)
        .map(|mut bw| {
            if bw.persistent_id == 32 {
                bw.data.as_object_mut().unwrap()
                    .get_mut("TierA").unwrap()
                    .as_array_mut().unwrap()[0]
                    .as_object_mut().unwrap()
                    .insert("LevelLayoutData".into(), serde_json::Value::Number(1.into()));
                bw.data.as_object_mut().unwrap()
                    .get_mut("TierB").unwrap()
                    .as_array_mut().unwrap().clear();
                bw.data.as_object_mut().unwrap()
                    .get_mut("TierC").unwrap()
                    .as_array_mut().unwrap().clear();
                bw.data.as_object_mut().unwrap()
                    .get_mut("TierD").unwrap()
                    .as_array_mut().unwrap().clear();

                bw.persistent_id = 1;
            }

            bw
        })
        .collect();

    rundowns.save_datablock(&plugins_path, "RundownDataBlock.json", "GameData_RundownDataBlock_bin")
}

fn init_level_layout(path: impl AsRef<Path>) -> Result<(), Error> {
    let plugins_path = path.as_ref().join("plugins");
    
    grab_db(&path, DatablockEnum::LevelLayout)?;
    let level_layout: DatablockWrapper<serde_json::Value> = DatablockWrapper::<serde_json::Value>::load_datablock(&plugins_path, "GameData_LevelLayoutDataBlock_bin")?
        .into_iter()
        .filter(|bw| bw.persistent_id == 495148404 || bw.persistent_id == 3788602088)
        .map(|mut bw| {
            if bw.persistent_id == 495148404 {
                bw.persistent_id = 1;
            }

            bw
        })
        .collect();

    level_layout.save_datablock(&plugins_path, "LevelLayoutDataBlock.json", "GameData_LevelLayoutDataBlock_bin")
}
