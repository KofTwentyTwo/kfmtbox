
use std::collections::HashMap;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct CommentStyle
{
   pub prefix: String,
   pub is_block: bool,
}

impl CommentStyle
{
   pub fn new(prefix: String) -> Self
   {
      return Self {
         prefix,
         is_block: false,
      };
   }
}

pub struct CommentDetector
{
   pub default_style: CommentStyle,
   style_map: HashMap<String, CommentStyle>,
}

impl CommentDetector
{
   pub fn new() -> Self
   {
      let mut style_map = HashMap::new();

      // Default comment styles for common file types
      style_map.insert("rs".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("py".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("js".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("ts".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("go".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("java".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("cpp".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("c".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("h".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("hpp".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("cs".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("php".to_string(), CommentStyle::new("//".to_string()));
      style_map.insert("rb".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("sh".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("bash".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("zsh".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("fish".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("vim".to_string(), CommentStyle::new("\"".to_string()));
      style_map.insert("lua".to_string(), CommentStyle::new("--".to_string()));
      style_map.insert("sql".to_string(), CommentStyle::new("--".to_string()));
      style_map.insert("yaml".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("yml".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("toml".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("ini".to_string(), CommentStyle::new(";".to_string()));
      style_map.insert("cfg".to_string(), CommentStyle::new(";".to_string()));
      style_map.insert("conf".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("dockerfile".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("makefile".to_string(), CommentStyle::new("#".to_string()));
      style_map.insert("cmake".to_string(), CommentStyle::new("#".to_string()));

      Self {
         default_style: CommentStyle::new("#".to_string()),
         style_map,
      }
   }

   pub fn detect_from_filename(&self, filename: &str) -> &CommentStyle
   {
      if let Some(ext) = filename.split('.').last()
      {
         let ext = ext.to_lowercase();
         self.style_map.get(&ext).unwrap_or(&self.default_style)
      }
      else
      {
         &self.default_style
      }
   }

   pub fn detect_from_filetype(&self, filetype: &str) -> &CommentStyle
   {
      let filetype = filetype.to_lowercase();
      self.style_map.get(&filetype).unwrap_or(&self.default_style)
   }

   pub fn load_config(&mut self, config: &Config)
   {
      for (filetype, prefix) in &config.comment_styles
      {
         self
            .style_map
            .insert(filetype.clone(), CommentStyle::new(prefix.clone()));
      }
   }
}
