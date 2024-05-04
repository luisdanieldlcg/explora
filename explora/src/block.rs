use std::{collections::HashMap, path::Path};

use common::block::BlockId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockSettings {
    pub name: String,
    pub textures: BlockTextures,
}

// TODO: improve this
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockTextures {
    pub top: String,
    pub bottom: String,
    pub north: String,
    pub west: String,
    pub east: String,
    pub south: String,
}

impl BlockSettings {}

pub struct BlockMap {
    pub blocks: HashMap<BlockId, BlockSettings>,
}

impl BlockMap {
    pub fn load<P: AsRef<Path>>(resource_path: P) -> Self {
        tracing::info!("Loading block map...");
        let Ok(dir) = std::fs::read_dir(&resource_path) else {
            panic!(
                "The directory `{}` does not exists",
                resource_path.as_ref().display()
            );
        };

        let mut registry = HashMap::new();
        for entry in dir.flatten() {
            let file = std::fs::read_to_string(entry.path()).unwrap(); // TODO: ignore if could not read.
            let settings = toml::from_str::<BlockSettings>(&file).unwrap();
            tracing::info!(?settings, "Reading block settings");
            registry.insert(BlockId::from(&settings.name), settings);
        }
        tracing::info!("Loaded {} blocks", registry.len());
        Self { blocks: registry }
    }
}
