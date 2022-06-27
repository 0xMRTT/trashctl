
use clap::{Command, AppSettings, Arg, ValueHint, value_parser};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::process::exit;

pub fn build_cli() -> Command<'static> {
    Command::new("example")
         .arg(Arg::new("file")
             .help("some input file")
                .value_hint(ValueHint::AnyPath),
        )
    .arg(
        Arg::new("completion")
        .long("completion")
        .help("Generate completion script")
        .value_name("SHELL")
    )
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

pub fn generate_completion(shell:&str) {
    match shell {
        "bash" => {
            print_completions(Shell::Bash, &mut build_cli());
            exit(0);
        }
        "zsh" => {
            print_completions(Shell::Zsh, &mut build_cli());
            exit(0);
        },
        "fish" => {
            print_completions(Shell::Fish, &mut build_cli());
            exit(0);
        },
        "powershell" => {
            print_completions(Shell::PowerShell, &mut build_cli());
            exit(0);
        },
        "elvish" => {
            print_completions(Shell::Elvish, &mut build_cli());
            exit(0);
        },    
        _ => {
            println!("Unknown completion type");
            exit(1);
        }
    }
}