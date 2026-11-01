use crate::{comment_detector::CommentStyle, text_wrapper::TextWrapper};

pub struct BoxRenderer {
   comment_style: CommentStyle,
   text_wrapper: TextWrapper,
}

impl BoxRenderer {
   #[must_use]
   pub const fn new(comment_style: CommentStyle, max_width: usize) -> Self {
      #[allow(clippy::needless_return)]
      return Self {
         comment_style,
         text_wrapper: TextWrapper::new(max_width),
      };
   }

   #[must_use]
   pub fn render_flower_box(&self, content: &str) -> String {
      // Check if content is already in a flower box
      let clean_content = self.extract_content_from_box(content);

      // Wrap and balance the text
      let wrapped_lines = self.text_wrapper.wrap_text(&clean_content);
      let balanced_lines = self.text_wrapper.balance_lines(wrapped_lines);

      // Find the widest line, in characters, for the box width
      let max_line_len = balanced_lines.iter().map(|line| return line.chars().count()).max().unwrap_or(0);

      // Inner width: one space either side of the text, minimum of 10
      let inner_width = max_line_len.saturating_add(2).max(10);
      let prefix = &self.comment_style.prefix;
      let border = "─".repeat(inner_width);

      // Build the flower box
      let mut result = Vec::new();

      // Top border
      result.push(format!("{prefix} ╭{border}╮"));

      // Content lines
      for line in balanced_lines {
         let padding = inner_width.saturating_sub(line.chars().count()).saturating_sub(2);
         result.push(format!("{prefix} │ {line}{} │", " ".repeat(padding)));
      }

      // Bottom border
      result.push(format!("{prefix} ╰{border}╯"));

      return result.join("\n");
   }

   /// Extracts content from an existing flower box comment, or returns the input unchanged.
   fn extract_content_from_box(&self, content: &str) -> String {
      let lines: Vec<&str> = content.lines().collect();
      let (Some(first), Some(last)) = (lines.first(), lines.last()) else {
         return content.to_string();
      };
      if lines.len() < 3 {
         return content.to_string();
      }

      // Check if this looks like a flower box
      let (first_line, last_line) = (first.trim(), last.trim());
      if !(first_line.contains('╭') && first_line.contains('╮') && last_line.contains('╰') && last_line.contains('╯')) {
         return content.to_string();
      }

      let prefix = self.comment_style.prefix.as_str();
      let mut content_lines = Vec::new();

      // Extract content from the middle lines
      for line in lines.iter().skip(1).take(lines.len().saturating_sub(2)) {
         // Remove the comment prefix first
         let clean_line = if line.trim_start().starts_with(prefix) {
            line.split_once(prefix).map_or(*line, |(_, rest)| return rest)
         } else {
            line
         };

         // The content sits between the first two pipe characters
         let mut parts = clean_line.split('│').skip(1);
         if let (Some(inner), Some(_)) = (parts.next(), parts.next()) {
            if !inner.is_empty() {
               content_lines.push(inner.trim().to_string());
            }
         }
      }
      return content_lines.join(" ");
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::comment_detector::CommentStyle;

   /// # Panics
   ///
   /// Panics if an assertion fails.
   #[test]
   fn test_render_flower_box() {
      let comment_style = CommentStyle::new("//".to_string());
      let renderer = BoxRenderer::new(comment_style, 80);
      let content = "Hello, world!";
      let result = renderer.render_flower_box(content);

      assert!(result.contains("╭"));
      assert!(result.contains("╮"));
      assert!(result.contains("╰"));
      assert!(result.contains("╯"));
      assert!(result.contains("Hello, world!"));
   }

   /// # Panics
   ///
   /// Panics if an assertion fails.
   #[test]
   fn test_extract_content_from_box() {
      let comment_style = CommentStyle::new("//".to_string());
      let renderer = BoxRenderer::new(comment_style, 80);

      let boxed_content = "// ╭─────────────╮\n// │ Hello, world! │\n// ╰─────────────╯";
      let extracted = renderer.extract_content_from_box(boxed_content);

      assert_eq!(extracted, "Hello, world!");
   }
}
