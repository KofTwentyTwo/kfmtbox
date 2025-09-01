use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "kfmtbox")]
#[command(about = "A command-line utility that wraps text in flower box style comments")]
#[command(version)]
pub struct Cli
{
   /// Input filename for file type detection
   #[arg(short = 'f', long = "filename")]
   pub filename: Option<String>,

   /// File type for comment style detection
   #[arg(short = 't', long = "filetype")]
   pub filetype: Option<String>,

   /// Maximum line width (default: 120)
   #[arg(short = 'w', long = "max-width", default_value = "120")]
   pub max_width: usize,
}
