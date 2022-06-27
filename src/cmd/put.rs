use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("restore")
        .about("Put a file or a directory in the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
        .arg(
            Arg::new("path")
                .long("path")
                .takes_value(true)
                .help("The path to move to trash")
                .required(false),
        )
}