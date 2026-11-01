use std::{collections::HashMap, path::PathBuf};

use dirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
   pub comment_styles: HashMap<String, String>,
}

impl Default for Config {
   fn default() -> Self {
      return Self { comment_styles: HashMap::new() };
   }
}

impl Config {
   /// Loads the configuration from the user's config directory, or returns the default when no file exists.
   ///
   /// # Errors
   ///
   /// Returns an error if the config file cannot be read or is not valid YAML.
   pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
      let config_path = Self::get_config_path();

      if config_path.exists() {
         let content = std::fs::read_to_string(&config_path)?;
         let config: Self = serde_yaml::from_str(&content)?;
         return Ok(config);
      } else {
         return Ok(Self::default());
      }
   }

   #[must_use]
   pub fn get_config_path() -> PathBuf {
      let mut config_dir = dirs::config_dir().unwrap_or_else(|| return PathBuf::from("."));
      config_dir.push("kfmtbox");
      config_dir.push("comments.yaml");
      return config_dir;
   }

   /// Writes the configuration to the user's config directory, creating it if needed.
   ///
   /// # Errors
   ///
   /// Returns an error if the directory or file cannot be written, or the config cannot be serialized.
   pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
      let config_path = Self::get_config_path();

      // Create config directory if it doesn't exist
      if let Some(parent) = config_path.parent() {
         std::fs::create_dir_all(parent)?;
      }

      let content = serde_yaml::to_string(self)?;
      std::fs::write(&config_path, content)?;
      return Ok(());
   }
}
