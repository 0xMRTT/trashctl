use clap::{arg, App, Arg, ArgMatches};
use crate::lib::home_trash_dir_path;
use crate::lib::Trash;
pub fn make_subcommand<'help>() -> App<'help> {
    App::new("empty")
        .about("Empty the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    let home_trash_dir_path = home_trash_dir_path();
    println!("{}", home_trash_dir_path.display());
    let trash = Trash::auto_recon(home_trash_dir_path);
    trash.empty();

    Ok(())
}