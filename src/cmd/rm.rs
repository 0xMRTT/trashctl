use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("rm")
        .about("Delete permanently a file or a directory from the trash")
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