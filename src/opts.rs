use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[arg(short, long)]
    /// Input file path
    pub input: Option<PathBuf>,
    #[arg(short, long)]
    /// Output file path
    pub output: Option<PathBuf>,

    /// Expression from command-line args, e.g. `1 + 2 * 3`
    #[arg()]
    pub expr: Vec<String>,
}
