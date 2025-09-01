
use std::{collections::HashMap, path::PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config
{
   pub comment_styles: HashMap<String, String>,
}

impl Default for Config
{
   fn default() -> Self
   {
      Self {
         comment_styles: HashMap::new(),
      }
   }
}

impl Config
{
   pub fn load() -> Result<Self, Box<dyn std::error::Error>>
   {
      let config_path = Self::get_config_path();

      if config_path.exists()
      {
         let content = std::fs::read_to_string(&config_path)?;
         let config: Config = serde_yaml::from_str(&content)?;
         Ok(config)
      }
      else
      {
         Ok(Config::default())
      }
   }

   pub fn get_config_path() -> PathBuf
   {
      let mut config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
      config_dir.push("kfmtbox");
      config_dir.push("comments.yaml");
      config_dir
   }

   pub fn save(&self) -> Result<(), Box<dyn std::error::Error>>
   {
      let config_path = Self::get_config_path();

      // Create config directory if it doesn't exist
      if let Some(parent) = config_path.parent()
      {
         std::fs::create_dir_all(parent)?;
      }

      let content = serde_yaml::to_string(self)?;
      std::fs::write(&config_path, content)?;
      Ok(())
   }
}
