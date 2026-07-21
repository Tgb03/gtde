use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{create_objects::targetted_constructor::TargettedConstructor, datablocks::{block_wrapper::BlockWrapper, reference::Reference}, generated::text::{LanguageData, Text}};

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
enum CharacterMetaDataTextInner {
    Generic,
    Warden,
    Code,
    Human,
    Terminal
}

#[derive(Clone, Serialize, Deserialize, JsonSchema)]
pub struct CreateText {
    character_meta_data: CharacterMetaDataTextInner,
    #[serde(rename = "Chinese_Simplified")]
    pub chinese_simplified: Option<String>,
    #[serde(rename = "Chinese_Traditional")]
    pub chinese_traditional: Option<String>,
    pub description: String,
    pub english: String,
    pub import_version: i64,
    pub export_version: i64,
    pub french: Option<String>,
    pub german: Option<String>,
    pub italian: Option<String>,
    pub japanese: Option<String>,
    pub korean: Option<String>,
    pub polish: Option<String>,
    #[serde(rename = "Portuguese_Brazil")]
    pub portuguese_brazil: Option<String>,
    pub russian: Option<String>,
    pub skip_localization: bool,
    pub spanish: Option<String>,

    pub block_name: String,
}

fn into_localized_text(text: Option<String>) -> LanguageData {
    LanguageData { 
        should_translate: text.as_ref().is_some(), 
        translation: text.unwrap_or_default(),
    }
}

impl Default for CreateText {
    fn default() -> Self {
        Self {
            character_meta_data: CharacterMetaDataTextInner::Generic,
            skip_localization: false,
            import_version: 1,
            export_version: 1,
            description: "Default description".into(),
            english: "".into(),
            chinese_simplified: None,
            chinese_traditional: None,
            french: None,
            german: None,
            italian: None,
            japanese: None,
            korean: None,
            polish: None,
            portuguese_brazil: None,
            russian: None,
            spanish: None,
            block_name: "default name".into(),
        }
    }
}

impl Into<Text> for CreateText {
    fn into(self) -> Text {
        Text {
            character_meta_data: self.character_meta_data as i64,
            skip_localization: self.skip_localization,
            import_version: self.import_version,
            export_version: self.export_version,
            description: self.description,
            english: self.english,
            chinese_simplified: into_localized_text(self.chinese_simplified),
            chinese_traditional: into_localized_text(self.chinese_traditional),
            french: into_localized_text(self.french),
            german: into_localized_text(self.german),
            italian: into_localized_text(self.italian),
            japanese: into_localized_text(self.japanese),
            korean: into_localized_text(self.korean),
            polish: into_localized_text(self.polish),
            portuguese_brazil: into_localized_text(self.portuguese_brazil),
            russian: into_localized_text(self.russian),
            spanish: into_localized_text(self.spanish),
        }
    }
}

impl TargettedConstructor for CreateText {
    type Data = Text;

    fn construct(
        self,
        env_path: impl AsRef<std::path::Path>,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, gtde_error::error::Error> {
        let actual_object: Text = self.clone().into();
        let block = BlockWrapper::new(actual_object, self.block_name, 0);
        let reference = block.construct(env_path, datablock_name)?;

        Ok(reference)
    }
}