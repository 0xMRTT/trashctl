use clap::{arg, App, Arg, ArgMatches};

pub fn make_subcommand<'help>() -> App<'help> {
    App::new("rm")
        .about("Delete permanently a file or a directory from the trash")
        .arg(arg!(--force "Skips confirmation prompts"))
        .arg(
            Arg::new("files").required(true).min_values(1),
        )
}

pub fn execute(args: &ArgMatches) -> Result<(), ()> {

    let files: Vec<_> = args.values_of("files").unwrap().collect();
    
    Ok(())
}