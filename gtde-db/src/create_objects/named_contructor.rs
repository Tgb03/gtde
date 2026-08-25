use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::create_objects::targetted_constructor::{
    TargettedConstructor, TargettedConstructorWithoutName,
};

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct NamedContructorWrapper<T> {
    pub name: String,
    #[serde(flatten)]
    pub data: T,
}

impl<T> TargettedConstructor for NamedContructorWrapper<T>
where
    T: TargettedConstructorWithoutName,
{
    type Data = T::Data;

    fn construct(
        self,
        env_path: impl AsRef<std::path::Path>,
        datablock_name: &'static str,
    ) -> Result<crate::datablocks::reference::Reference<Self::Data>, gtde_error::error::Error> {
        self.data.construct(env_path, self.name, datablock_name)
    }
}
