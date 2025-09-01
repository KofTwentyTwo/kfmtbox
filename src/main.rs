use std::io::{self, Read};

use clap::Parser;
use kfmtbox::{BoxRenderer, Cli, CommentDetector, Config};

/// Main entry point for the kfmtbox application.
///
/// Reads text from stdin, formats it as a flower box comment, and outputs to stdout.
/// The comment style is determined by the file type specified via command line arguments.
///
/// # Errors
///
/// This function will return an error if:
/// - Input cannot be read from stdin
/// - Configuration file cannot be loaded or parsed
/// - Output cannot be written to stdout
fn main() -> Result<(), Box<dyn std::error::Error>>
{
   let cli = Cli::parse();

   // Load configuration
   let config = Config::load().unwrap_or_else(|_| Config::default());

   // Initialize comment detector
   let mut detector = CommentDetector::new();
   detector.load_config(&config);

   // Determine comment style
   let comment_style = if let Some(filename) = &cli.filename
   {
      detector.detect_from_filename(filename).clone()
   }
   else if let Some(filetype) = &cli.filetype
   {
      detector.detect_from_filetype(filetype).clone()
   }
   else
   {
      detector.default_style.clone()
   };

   // Read input from stdin
   let mut input = String::new();
   io::stdin().read_to_string(&mut input)?;

   // Remove trailing newline if present and normalize line endings
   let input = input.trim_end();
   let input = input.replace("\r\n", "\n").replace("\n\r", "\n").replace("\r", "\n");

   if input.is_empty()
   {
      return Ok(());
   }

   // Create box renderer and render the flower box
   let renderer = BoxRenderer::new(comment_style, cli.max_width);
   let result = renderer.render_flower_box(&input);

   // Output to stdout
   println!("{}", result);

   Ok(())
}
