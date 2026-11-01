use std::collections::HashMap;

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct CommentStyle {
   pub prefix: String,
   pub is_block: bool,
}

impl CommentStyle {
   #[must_use]
   pub const fn new(prefix: String) -> Self {
      return Self { prefix, is_block: false };
   }
}

/// Default comment prefixes for common file types.
const DEFAULT_STYLES: &[(&str, &str)] = &[
   ("rs", "//"),
   ("py", "#"),
   ("js", "//"),
   ("ts", "//"),
   ("go", "//"),
   ("java", "//"),
   ("cpp", "//"),
   ("c", "//"),
   ("h", "//"),
   ("hpp", "//"),
   ("cs", "//"),
   ("php", "//"),
   ("rb", "#"),
   ("sh", "#"),
   ("bash", "#"),
   ("zsh", "#"),
   ("fish", "#"),
   ("vim", "\""),
   ("lua", "--"),
   ("sql", "--"),
   ("yaml", "#"),
   ("yml", "#"),
   ("toml", "#"),
   ("ini", ";"),
   ("cfg", ";"),
   ("conf", "#"),
   ("dockerfile", "#"),
   ("makefile", "#"),
   ("cmake", "#"),
];

pub struct CommentDetector {
   pub default_style: CommentStyle,
   style_map: HashMap<String, CommentStyle>,
}

impl CommentDetector {
   #[must_use]
   pub fn new() -> Self {
      let style_map = DEFAULT_STYLES
         .iter()
         .map(|&(filetype, prefix)| return (filetype.to_string(), CommentStyle::new(prefix.to_string())))
         .collect();

      return Self {
         default_style: CommentStyle::new("#".to_string()),
         style_map,
      };
   }

   #[must_use]
   pub fn detect_from_filename(&self, filename: &str) -> &CommentStyle {
      let ext = filename.rsplit('.').next().unwrap_or_default().to_lowercase();
      return self.style_map.get(&ext).unwrap_or(&self.default_style);
   }

   #[must_use]
   pub fn detect_from_filetype(&self, filetype: &str) -> &CommentStyle {
      let filetype = filetype.to_lowercase();
      return self.style_map.get(&filetype).unwrap_or(&self.default_style);
   }

   pub fn load_config(&mut self, config: &Config) {
      for (filetype, prefix) in &config.comment_styles {
         self.style_map.insert(filetype.clone(), CommentStyle::new(prefix.clone()));
      }
   }
}

impl Default for CommentDetector {
   fn default() -> Self {
      return Self::new();
   }
}
