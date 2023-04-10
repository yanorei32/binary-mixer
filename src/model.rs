use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(about = "Mix binaries into a binary")]
    Mix {
        #[arg(short, required = true)]
        inputs: Vec<PathBuf>,
        #[arg(short)]
        output: PathBuf,
        #[arg(short)]
        force: bool,
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Split a binary into binaries")]
    Split {
        #[arg(short)]
        input: PathBuf,
        #[arg(short, required = true)]
        outputs: Vec<PathBuf>,
        #[arg(short)]
        force: bool,
    },
}
