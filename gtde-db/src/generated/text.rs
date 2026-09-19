use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::datablocks::reference::Reference;

#[derive(Clone, PartialEq, Serialize, Default, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Text {
    pub character_meta_data: i64,
    #[serde(rename = "Chinese_Simplified")]
    pub chinese_simplified: LanguageData,
    #[serde(rename = "Chinese_Traditional")]
    pub chinese_traditional: LanguageData,
    pub description: String,
    pub english: String,
    pub export_version: i64,
    pub french: LanguageData,
    pub german: LanguageData,
    pub import_version: i64,
    pub italian: LanguageData,
    pub japanese: LanguageData,
    pub korean: LanguageData,
    pub polish: LanguageData,
    #[serde(rename = "Portuguese_Brazil")]
    pub portuguese_brazil: LanguageData,
    pub russian: LanguageData,
    pub skip_localization: bool,
    pub spanish: LanguageData,
}

#[derive(Clone, PartialEq, Serialize, Default, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LanguageData {
    pub should_translate: bool,
    pub translation: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum LocalizedText {
    Integer(Reference<Text>),
    String(String),
}

impl Default for LocalizedText {
    fn default() -> Self {
        Self::String(String::new())
    }
}
