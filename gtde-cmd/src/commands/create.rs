use clap::ValueEnum;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io, path::Path};
use textwrap::wrap;

use gtde_db::{
    create_objects::{
        create_chained_puzzle::CreateChainedPuzzle,
        create_survival_wave_population::SurvivalWavePopulationIntermediary,
        create_text::CreateText,
        generic_constructor::GenericConstructor,
        load_constructor::{create_constructor, create_schema, load_constructor},
        named_contructor::NamedContructorWrapper,
        targetted_constructor::TargettedConstructor,
    },
    generated::survival_wave_settings::SurvivalWaveSettings,
};
use gtde_error::error::Error;

#[derive(Debug, Serialize, Deserialize, Hash, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CreateFiles {
    ChainedPuzzle,
    SurvivalWaveSettings,
    SurvivalWavePopulation,
    Text,
}

impl Into<&'static str> for CreateFiles {
    fn into(self) -> &'static str {
        match self {
            CreateFiles::ChainedPuzzle => "chained-puzzle",
            CreateFiles::SurvivalWaveSettings => "survival-wave-settings",
            CreateFiles::SurvivalWavePopulation => "survival-wave-population",
            CreateFiles::Text => "text",
        }
    }
}

fn get_templates(
    templates: &HashMap<CreateFiles, Vec<Template>>,
    create_file: CreateFiles,
) -> Result<Option<serde_json::Value>, Error> {
    let Some(templates) = templates.get(&create_file) else {
        return Ok(None);
    };

    println!("Your options are: ");
    Template::print_options(templates);
    print!("Please select one: ");
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let text = buffer.trim();
    println!("Read: <{}>", &text);

    for template in templates {
        if template.name == text {
            println!("Selected {}", template.name.green());

            return Ok(Some(template.data.clone()));
        }
    }

    println!(
        "{}",
        "Failed to match your given name to an existing template, ignoring request".red()
    );
    Ok(None)
}

#[derive(Debug, Serialize, Deserialize)]
struct Template {
    pub name: String,
    pub description: String,
    pub data: serde_json::Value,
}

impl Template {
    pub fn print_options(options: &Vec<Template>) {
        let name_width = options
            .iter()
            .map(|option| option.name.len())
            .max()
            .unwrap_or(0);

        const DESCRIPTION_WIDTH: usize = 80;

        for option in options {
            let lines = wrap(&option.description, DESCRIPTION_WIDTH);

            for (i, line) in lines.iter().enumerate() {
                if i == 0 {
                    println!(
                        "\"{:<width$}\"            {}",
                        option.name,
                        line,
                        width = name_width
                    );
                } else {
                    println!("\"{:<width$}\"            {}", "", line, width = name_width);
                }
            }
        }
    }
}

pub fn create(path: impl AsRef<Path>, file_used: CreateFiles, reset: bool) -> Result<(), Error> {
    let create_folder_path = path.as_ref().join("gtde-create");
    let file_name: &'static str = file_used.into();
    let create_file_path = create_folder_path.join(file_name).with_extension("json");

    if reset || !std::fs::exists(create_file_path)? {
        let embed = include_bytes!("../../../resources/templates.json");
        let map = serde_json::from_slice(embed)?;
        let data = get_templates(&map, file_used)?.ok_or(Error::ConstructorNotChosen)?;
        let _ = create_constructor(&path, file_name, data)?;
        match file_used {
            CreateFiles::ChainedPuzzle => create_schema::<CreateChainedPuzzle>(&path, file_name)?,
            CreateFiles::SurvivalWaveSettings => {
                create_schema::<NamedContructorWrapper<SurvivalWaveSettings>>(&path, file_name)?
            }
            CreateFiles::SurvivalWavePopulation => create_schema::<
                NamedContructorWrapper<SurvivalWavePopulationIntermediary>,
            >(&path, file_name)?,
            CreateFiles::Text => create_schema::<CreateText>(&path, file_name)?,
        };

        return Ok(());
    }

    match file_used {
        CreateFiles::ChainedPuzzle => {
            load_constructor::<CreateChainedPuzzle>(&path, "chained-puzzle")?
                .construct_all(&path)?
        }
        CreateFiles::SurvivalWaveSettings => {
            load_constructor::<NamedContructorWrapper<SurvivalWaveSettings>>(
                &path,
                "survival-wave-settings",
            )?
            .construct(&path, "SurvivalWaveSettings")?;
        }
        CreateFiles::SurvivalWavePopulation => {
            load_constructor::<NamedContructorWrapper<SurvivalWavePopulationIntermediary>>(
                &path,
                "survival-wave-population",
            )?
            .construct(&path, "SurvivalWavePopulation")?;
        }
        CreateFiles::Text => {
            load_constructor::<CreateText>(&path, "text")?.construct(&path, "Text")?;
        }
    };

    Ok(())
}
