use crate::assets::get_assets;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    models: Vec<String>,
    textures: Vec<String>,
}

impl Version {
    pub fn new(version: &str) -> Result<Version, reqwest::Error> {
        let models = get_assets(version, "minecraft", "models", "json")?;
        let textures = get_assets(version, "minecraft", "textures", "png")?;
        Ok(Version { models, textures })
    }
}
