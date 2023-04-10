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
        #[arg(required = true)]
        inputs: Vec<PathBuf>,
        #[arg(required = true)]
        output: PathBuf,
        #[arg(short)]
        force: bool,
    },

    #[command(arg_required_else_help = true)]
    #[command(about = "Split a binary into binaries")]
    Split {
        #[arg(required = true)]
        input: PathBuf,
        #[arg(required = true)]
        outputs: Vec<PathBuf>,
        #[arg(short)]
        force: bool,
    },
}
