
use crate::{comment_detector::CommentStyle, text_wrapper::TextWrapper};

pub struct BoxRenderer
{
   comment_style: CommentStyle,
   text_wrapper: TextWrapper,
}

impl BoxRenderer
{
   #[must_use]
   pub fn new(comment_style: CommentStyle, max_width: usize) -> Self
   {
      #[allow(clippy::needless_return)]
      return Self {
         comment_style,
         text_wrapper: TextWrapper::new(max_width),
      };
   }

   #[must_use]
   pub fn render_flower_box(&self, content: &str) -> String
   {
      // Check if content is already in a flower box
      let clean_content = self.extract_content_from_box(content);

      // Wrap and balance the text
      let wrapped_lines = self.text_wrapper.wrap_text(&clean_content);
      let balanced_lines = self.text_wrapper.balance_lines(wrapped_lines);

      // Find the maximum line length for box width
      let max_line_len = balanced_lines
         .iter()
         .map(|line| line.len())
         .max()
         .unwrap_or(0);

      let box_width = (max_line_len + 4).max(10); // Minimum width of 10
      let prefix = &self.comment_style.prefix;

      // Build the flower box
      let mut result = Vec::new();

      // Top border
      result.push(format!(
         "{prefix} {}",
         "/".repeat(box_width)
      ));

      // Content lines
      for line in balanced_lines
      {
         let padding = box_width - line.len() - 2;
         let padded_line = format!(
            "{prefix} {line}{} {prefix}",
            " ".repeat(padding)
         );
         result.push(padded_line);
      }

      // Bottom border
      result.push(format!(
         "{prefix} {}",
         "/".repeat(box_width)
      ));

      result.join("\n")
   }

   /// Extracts content from an existing flower box comment.
   ///
   /// # Panics
   ///
   /// This function may panic if the input contains malformed flower box syntax
   /// or if string slicing operations fail due to UTF-8 boundary issues.
   fn extract_content_from_box(&self, content: &str) -> String
   {
      let lines: Vec<&str> = content.lines().collect();
      if lines.len() < 3
      {
         return content.to_string();
      }

      let prefix = &self.comment_style.prefix;
      let mut content_lines = Vec::new();

      // Check if this looks like a flower box
      let first_line = lines[0].trim();
      let last_line = lines[lines.len() - 1].trim();

      if first_line.contains("╭")
         && first_line.contains("╮")
         && last_line.contains("╰")
         && last_line.contains("╯")
      {
         // Extract content from middle lines
         for line in lines.iter().skip(1).take(lines.len() - 2)
         {
            // Remove the comment prefix first
            let clean_line = if line.trim_start().starts_with(prefix)
            {
               &line[line.find(prefix).unwrap() + prefix.len()..]
            }
            else
            {
               line
            };

            // Find the content between the pipe characters using char-based approach
            let chars: Vec<char> = clean_line.chars().collect();
            let mut start_idx = None;
            let mut end_idx = None;

            for (i, ch) in chars.iter().enumerate()
            {
               if *ch == '│'
               {
                  if start_idx.is_none()
                  {
                     start_idx = Some(i + 1);
                  }
                  else
                  {
                     end_idx = Some(i);
                     break;
                  }
               }
            }

            if let (Some(start), Some(end)) = (start_idx, end_idx)
            {
               if end > start
               {
                  let content_part: String = chars[start..end].iter().collect();
                  content_lines.push(content_part.trim().to_string());
               }
            }
         }
         return content_lines.join(" ");
      }
      #[allow(clippy::needless_return)]
      return content.to_string();
   }
}

#[cfg(test)]
mod tests
{
   use super::*;
   use crate::comment_detector::CommentStyle;

   #[test]
   fn test_render_flower_box()
   {
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

   #[test]
   fn test_extract_content_from_box()
   {
      let comment_style = CommentStyle::new("//".to_string());
      let renderer = BoxRenderer::new(comment_style, 80);

      let boxed_content = "// ╭─────────────╮\n// │ Hello, world! │\n// ╰─────────────╯";
      let extracted = renderer.extract_content_from_box(boxed_content);

      assert_eq!(extracted, "Hello, world!");
   }
}
