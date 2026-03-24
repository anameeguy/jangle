use clap::Parser;
use jangle::DotPath;

use crate::{
    command::{Cli, Commands},
    sheet::Sheet,
    working_sheet::{get_working_sheet_location, set_working_sheet},
};

mod command;
mod dir;
mod sheet;
mod working_sheet;

fn main() {
    let beep = DotPath::new("[[root-place]].beep1.beep2.beep3.").unwrap();
    println!("{beep:?}");
    return;

    //

    //

    //

    //

    //

    #[allow(unreachable_code)]
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::New { path, ignore } => {
            if !path
                .try_exists()
                .expect("Can't check if file at path exists for whatever reason.")
            {
                Sheet::default().save(path);

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
            if Sheet::is_sheet_at_path(path) {
                set_working_sheet(path);
                println!("Working directory set!");
            } else {
                println!("That is not a Jangle sheet.");
            }
        }
    }
}
