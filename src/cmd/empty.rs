use clap::{arg, App, Arg, ArgMatches};
use crate::trash::home_trash_dir_path_from_env;

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("empty")
        .about("Empty the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    let home_trash_dir_path = home_trash_dir_path_from_env();
    println!("{}", home_trash_dir_path.display());
    Ok(())
}