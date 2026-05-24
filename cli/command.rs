use std::path::PathBuf;

use clap::{Parser, Subcommand};
use jangle::{
    DotPath,
    dot_path::{BranchTarget, TrueRoot},
};

use crate::generic_root_dotpath::GenericRootDotpath;

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

    /// Does something with the sheet.
    Sheet {
        /// Path to the sheet that will be used.
        /// Working sheet will be used if this is unset.
        #[arg(short = 'p', long)]
        sheet_path: Option<PathBuf>,

        #[command(subcommand)]
        command: SheetCommands,
    },
}

#[derive(Subcommand)]
pub enum SheetCommands {
    /// Sets the current working branch within the working sheet.
    Work {
        /// Dotpath to the new working branch.
        branch_path: GenericRootDotpath<BranchTarget>,

        /// Defines the refrence location for a local root.
        /// If left as none it will be the current working branch.
        /// This is required if not on the working sheet.
        #[arg(short = 'r', long, value_parser = DotPath::<TrueRoot, BranchTarget>::new)]
        position: Option<DotPath<TrueRoot, BranchTarget>>,
    },
}
