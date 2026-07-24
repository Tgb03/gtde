use crate::datablocks::{block_wrapper::BlockWrapper, reference::Reference};
use gtde_error::error::{Error, ErrorRanOutOfPersistentIDs};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::{
    ops::{Deref, DerefMut}, path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddResult<T> {
    pub was_added: bool,
    pub reference: Reference<T>,
}

impl<T> AddResult<T> {
    pub fn new(reference: Reference<T>, was_added: bool) -> Self {
        Self {
            was_added,
            reference,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatablockWrapper<T> {
    #[serde(rename = "Blocks")]
    blocks: Vec<BlockWrapper<T>>,
    #[serde(rename = "LastPersistentID")]
    last_persistent_id: u32,
}

impl<T> Deref for DatablockWrapper<T> {
    type Target = [BlockWrapper<T>];

    fn deref(&self) -> &Self::Target {
        &self.blocks
    }
}

impl<T> DerefMut for DatablockWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.blocks
    }
}

impl<T> Default for DatablockWrapper<T> {
    fn default() -> Self {
        Self {
            blocks: Default::default(),
            last_persistent_id: Default::default(),
        }
    }
}

impl<T> DatablockWrapper<T> {
    pub fn ensure_correct(&mut self) {
        self.blocks.sort_by_key(|e| e.persistent_id);
        self.last_persistent_id = self
            .blocks
            .last()
            .map(|e| e.persistent_id)
            .unwrap_or_default();
    }
}

impl<T: PartialEq> DatablockWrapper<T> {
    /// add some data to the datablock. If data is already present it does not get added.
    /// in both cases the id of where the data is stored is responded.
    ///
    /// the name variable gets cloned when the data is constructed.
    pub fn check_add(
        &mut self,
        data: T,
        name: &str,
    ) -> Result<AddResult<T>, ErrorRanOutOfPersistentIDs> {
        if let Some(id) = self.check_exists(&data) {
            return Ok(AddResult::new(id, false));
        }

        if self.last_persistent_id == u32::MAX {
            return self.check_add_super_slow(data, name);
        }

        self.last_persistent_id += 1;
        self.blocks.push(BlockWrapper::new(
            data,
            name.to_owned(),
            self.last_persistent_id,
        ));
        
        Ok(AddResult::new(self.last_persistent_id.into(), true))
    }

    fn check_exists(&self, data: &T) -> Option<Reference<T>> {
        self.blocks
            .iter()
            .find(|e| &e.data == data)
            .map(|e| e.persistent_id.into())
    }

    fn check_add_super_slow(
        &mut self,
        data: T,
        name: &str,
    ) -> Result<AddResult<T>, ErrorRanOutOfPersistentIDs> {
        let first_empty_id = self
            .blocks
            .iter()
            .enumerate()
            .filter(|(id, v)| *id != v.persistent_id as usize)
            .next()
            .map(|(id, _)| id)
            .ok_or(ErrorRanOutOfPersistentIDs)?;

        self.blocks.insert(
            first_empty_id,
            BlockWrapper::new(data, name.to_owned(), first_empty_id as u32),
        );

        Ok(AddResult::new((first_empty_id as u32).into(), true))
    }
}

impl<T: Serialize + DeserializeOwned> DatablockWrapper<T> {
    pub fn load_datablock(
        env_path: impl AsRef<Path>,
        name: &str,
    ) -> Result<DatablockWrapper<T>, Error> {
        let file_path = env_path.as_ref().join(name).with_extension("json");
        let file_contents = std::fs::read(&file_path).map_err(Error::io_at(file_path))?;
        let mut result_object: DatablockWrapper<T> = serde_json::from_slice(&file_contents)?;
        result_object.ensure_correct();

        Ok(result_object)
    }

    pub fn save_datablock(
        self,
        env_path: impl AsRef<Path>,
        schema_name: &str,
        name: &str,
    ) -> Result<(), Error> {
        let mut json_value = serde_json::to_value(&self)?;
        if let Value::Object(map) = &mut json_value {
            map.insert(
                "$schema".to_owned(),
                Value::String(format!("../.schemas/{}", schema_name)),
            );
        }
        let file_contents = serde_json::to_string_pretty(&json_value)?;
        let file_path = env_path.as_ref().join(name).with_extension("json");
        std::fs::write(&file_path, &file_contents).map_err(Error::io_at(file_path))?;

        Ok(())
    }
}

impl<T: Serialize + DeserializeOwned + PartialEq> DatablockWrapper<T> {
    pub fn add_block_to_files(
        env_path: impl AsRef<Path>,
        datablock_name: &str,
        schema_name: &str,
        data_name: &str,
        data: T,
    ) -> Result<Reference<T>, Error> {
        let mut datablock = Self::load_datablock(&env_path, datablock_name)?;
        let result = datablock.check_add(data, data_name)?;

        if result.was_added == true {
            datablock.save_datablock(&env_path, schema_name, datablock_name)?;
        }

        Ok(result.reference)
    }
}
