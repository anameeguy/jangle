use clap::Parser;
#[allow(unused_imports)]
use jangle::{
    DotPath,
    dot_path::{BranchTarget, DataTarget, PositionedRoot, TrueRoot},
};
use ron::ser::PrettyConfig;

use crate::{
    command::{Cli, Commands},
    sheet::Sheet,
    working_sheet::{get_working_sheet_location, set_working_sheet},
};

mod command;
mod dir;
mod sheet;
mod working_sheet;

fn main() -> anyhow::Result<()> {
    let pretty_config = PrettyConfig::new().compact_arrays(true);

    let dotpath = DotPath::<TrueRoot>::new("#.beep")?;

    println!(
        "{}\n{dotpath}",
        ron::ser::to_string_pretty(&dotpath, pretty_config)?
    );

    #[allow(unused_variables)]
    let sheet = Sheet::load("elaugdrin.sheet");

    // println!("{}", sheet.true_root);
    return Ok(());

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

    Ok(())
}
