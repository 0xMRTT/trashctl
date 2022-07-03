use std::path::PathBuf;

use clap::{arg, App, Arg, ArgMatches};
use crate::lib::Trash;
use crate::lib::home_trash_dir_path;

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("put")
        .about("Put a file or a directory in the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
        .arg(
            Arg::new("path").required(true).min_values(1),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    let mut trash_path = home_trash_dir_path();
    let mut files = args.values_of("path").unwrap();
   

    let mut trash = Trash::new( trash_path);
    
    
    for file in files {
        trash.add(PathBuf::from(file));
    }
    Ok(())
}
