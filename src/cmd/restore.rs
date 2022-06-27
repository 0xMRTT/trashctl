use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("restore")
        .about("Restore a file or a directory from trash")
        .arg(arg!(--force "Skips confirmation prompts"))
        .arg(
            Arg::new("path")
                .long("path")
                .takes_value(true)
                .help("The path to restore")
                .required(false),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    Ok(())
}