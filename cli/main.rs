use anyhow::Context;
use clap::Parser;
use jangle::TrueRootSheet;
#[allow(unused_imports)]
use jangle::{
    DotPath,
    dot_path::{BranchTarget, DataTarget, PositionedRoot, TrueRoot},
};
use ron::ser::PrettyConfig;

use crate::{
    command::{Cli, Commands},
    working_sheet::{WorkingSheetCache, get_working_sheet_location, set_working_sheet},
};

mod command;
mod dir;
mod generic_root_dotpath;
mod working_sheet;

fn main() -> anyhow::Result<()> {
    #[allow(unused_variables)]
    let pretty_config = PrettyConfig::new().compact_arrays(true);

    //

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match cli.command {
        Commands::New { path, ignore } => {
            if !path
                .try_exists()
                .expect("Can't check if file at path exists for whatever reason.")
            {
                TrueRootSheet::default().save(&path).unwrap();

                if !ignore {
                    set_working_sheet(path);
                }

                println!("Created sheet.")
            } else {
                println!("Please do not try to overwrite an already existing file.")
            }
        }
        Commands::FindWs => {
            if let Some(local) = get_working_sheet_location() {
                println!("Current working sheet is at:\n{local:?}")
            } else {
                println!("There doesn't appear to be a working sheet at the moment.")
            }
        }
        Commands::Work { path } => {
            // TODO: Make this actually properly checks the file to make sure that it is good.
            set_working_sheet(path);
            println!("Working sheet updated.");
        }
        Commands::Sheet {
            sheet_path,
            command,
        } => {
            #[allow(unused_variables)]
            let (sheet_path, is_hard_defined) = if let Some(actual_sheet_path) = sheet_path {
                (actual_sheet_path, true)
            } else {
                (
                    get_working_sheet_location()
                        .context("Failed to find a working sheet location. Likely doesn't exist")?,
                    false,
                )
            };

            match command {
                #[allow(unused_variables)]
                command::SheetCommands::Work { branch_path } => {
                    let working_sheet_cache = WorkingSheetCache::get(); // TODO: Remove the need to pull from cache multiple times.

                    match branch_path {
                        generic_root_dotpath::GenericRootDotpath::TrueRootDotpath(dot_path) => {
                            //

                            todo!()
                        }
                        generic_root_dotpath::GenericRootDotpath::PositionedRootDotpath(
                            dot_path,
                        ) => {
                            //

                            todo!()
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
