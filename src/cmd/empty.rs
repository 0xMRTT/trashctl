use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("empty")
        .about("Empty the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
}