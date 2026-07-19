use std::path::Path;

use gtde_error::error::Error;
use serde::{Serialize, de::DeserializeOwned};

use crate::datablocks::{
    block_wrapper::BlockWrapper, datablock_wrapper::DatablockWrapper, reference::Reference,
};

pub trait TargettedConstructorWithoutName {
    type Data: Serialize + DeserializeOwned + PartialEq;

    fn construct(
        self,
        env_path: impl AsRef<Path>,
        name: String,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, Error>;
}

impl<T: Serialize + DeserializeOwned + PartialEq> TargettedConstructorWithoutName for T {
    type Data = T;

    fn construct(
        self,
        env_path: impl AsRef<Path>,
        name: String,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, Error> {
        let schema_name = format!("{}DataBlock.json", datablock_name);
        let datablock_name = format!("GameData_{}DataBlock_bin.json", datablock_name);

        let result = DatablockWrapper::<T>::add_block_to_files(
            env_path.as_ref().join("plugins"),
            &datablock_name,
            &schema_name,
            &name,
            self,
        )?;

        println!(
            "Added {} with id {} to {}",
            name,
            Into::<u32>::into(result),
            datablock_name
        );
        Ok(result)
    }
}

pub trait TargettedConstructor {
    type Data: Serialize + DeserializeOwned + PartialEq;

    fn construct(
        self,
        env_path: impl AsRef<Path>,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, Error>;
}

impl<T: Serialize + DeserializeOwned + PartialEq> TargettedConstructor for BlockWrapper<T> {
    type Data = T;

    fn construct(
        self,
        env_path: impl AsRef<Path>,
        datablock_name: &'static str,
    ) -> Result<Reference<Self::Data>, Error> {
        let schema_name = format!("{}DataBlock.json", datablock_name);
        let datablock_name = format!("GameData_{}DataBlock_bin.json", datablock_name);

        let object_name = self.name.clone();
        let result = DatablockWrapper::<T>::add_block_to_files(
            env_path.as_ref().join("plugins"),
            &datablock_name,
            &schema_name,
            &object_name,
            self.data,
        )?;

        println!(
            "Added {} with id {} to {}",
            object_name,
            Into::<u32>::into(result),
            datablock_name
        );
        Ok(result)
    }
}
