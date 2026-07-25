use clap::ValueEnum;
use std::path::Path;

use gtde_db::{
    create_objects::{
        create_chained_puzzle::CreateChainedPuzzle,
        create_survival_wave_population::CreateSurvivalWavePopulation, create_text::CreateText,
        generic_constructor::GenericConstructor, load_constructor::load_constructor,
        targetted_constructor::TargettedConstructor,
    },
    datablocks::block_wrapper::BlockWrapper,
    generated::survival_wave_settings::SurvivalWaveSettings,
};
use gtde_error::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CreateFiles {
    ChainedPuzzle,
    SurvivalWaveSettings,
    SurvivalWavePopulation,
    Text,
}

pub fn create(path: impl AsRef<Path>, file_used: CreateFiles) -> Result<(), Error> {
    match file_used {
        CreateFiles::ChainedPuzzle => {
            load_constructor::<CreateChainedPuzzle>(&path, "chained_puzzle.json")?
                .construct_all(&path)?;
        }
        CreateFiles::SurvivalWaveSettings => {
            load_constructor::<BlockWrapper<SurvivalWaveSettings>>(
                &path,
                "survival_wave_settings.json",
            )?
            .construct(&path, "SurvivalWaveSettings")?;
        }
        CreateFiles::Text => {
            load_constructor::<CreateText>(&path, "text.json")?.construct(&path, "Text")?;
        }
        CreateFiles::SurvivalWavePopulation => {
            load_constructor::<CreateSurvivalWavePopulation>(
                &path,
                "survival_wave_population.json",
            )?
            .construct(&path, "SurvivalWavePopulation")?;
        }
    };

    Ok(())
}
