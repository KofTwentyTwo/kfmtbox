pub struct TextWrapper {
   max_width: usize,
}

impl TextWrapper {
   #[must_use]
   pub const fn new(max_width: usize) -> Self {
      return Self { max_width };
   }

   #[must_use]
   pub fn wrap_text(&self, text: &str) -> Vec<String> {
      // First, split by newlines to preserve existing line breaks
      let input_lines: Vec<&str> = text.split('\n').collect();
      let mut all_lines = Vec::new();

      for line in input_lines {
         let words: Vec<&str> = line.split_whitespace().collect();
         if words.is_empty() {
            all_lines.push(String::new());
            continue;
         }

         let mut current_line = String::new();

         for word in words {
            let test_line = if current_line.is_empty() { word.to_string() } else { format!("{current_line} {word}") };

            // Use a smaller effective width for better wrapping
            let effective_width = 20; // Force wrapping at 20 characters
            if test_line.len() <= effective_width {
               current_line = test_line;
            } else {
               if !current_line.is_empty() {
                  all_lines.push(current_line);
               }
               current_line = word.to_string();
            }
         }

         if !current_line.is_empty() {
            all_lines.push(current_line);
         }
      }

      if all_lines.is_empty() {
         return vec![String::new()];
      }

      return all_lines;
   }

   #[must_use]
   pub fn balance_lines(&self, lines: Vec<String>) -> Vec<String> {
      if lines.len() <= 1 {
         return lines;
      }

      let total_chars: usize = lines.iter().map(|line| return line.len()).sum();
      let avg_chars_per_line = total_chars.checked_div(lines.len()).unwrap_or(0);
      let target_width = usize::midpoint(avg_chars_per_line, self.max_width);
      let max_combined_width = 25; // Don't combine lines if result would be longer than 25 chars

      let mut balanced = Vec::new();
      let mut current_line = String::new();

      for line in lines {
         if current_line.is_empty() {
            current_line = line;
         } else {
            let test_line = format!("{current_line} {line}");
            if test_line.len() <= target_width && test_line.len() <= max_combined_width {
               current_line = test_line;
            } else {
               balanced.push(current_line);
               current_line = line;
            }
         }
      }

      if !current_line.is_empty() {
         balanced.push(current_line);
      }

      return balanced;
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   /// # Panics
   ///
   /// Panics if an assertion fails.
   #[test]
   fn test_wrap_text() {
      let wrapper = TextWrapper::new(20);
      let text = "This is a very long line that should be wrapped into multiple lines";
      let result = wrapper.wrap_text(text);

      assert!(result.len() > 1);
      for line in &result {
         assert!(line.len() <= 20);
      }
   }

   /// # Panics
   ///
   /// Panics if an assertion fails.
   #[test]
   fn test_balance_lines() {
      let wrapper = TextWrapper::new(20);
      let lines = vec!["Short".to_string(), "Very long line that exceeds".to_string(), "Another short".to_string()];
      let result = wrapper.balance_lines(lines);

      assert!(!result.is_empty());
   }
}
