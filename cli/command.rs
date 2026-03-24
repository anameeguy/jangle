use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new sheet.
    New {
        /// The path to create the new sheet.
        path: PathBuf,
        /// Enable this to not begin working with the new sheet.
        #[arg(short, long)]
        ignore: bool,
    },

    /// Finds the current working sheets location.
    FindWs,

    /// Sets the working sheet.
    Work {
        /// The sheets paths.
        path: PathBuf,
    },
}
