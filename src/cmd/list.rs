use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("list")
        .about("List trash files")
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {
    Ok(())
}