
use clap::{Command, AppSettings, Arg, ValueHint, value_parser};
use clap_complete::{generate, Generator, Shell};
use std::io;

pub fn build_cli() -> Command<'static> {
    Command::new("example")
         .arg(Arg::new("file")
             .help("some input file")
                .value_hint(ValueHint::AnyPath),
        )
       .arg(
           Arg::new("generator")
               .long("generate")
               .value_parser(value_parser!(Shell)),
       )
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
