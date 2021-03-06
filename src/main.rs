
use clap::{AppSettings,App, Arg};
use clap_complete::Shell;
use env_logger;
use anyhow::anyhow;
mod cmd;
pub mod lib;
use log::{debug, error, info};
use std::path::PathBuf;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn create_clap_app() -> App<'static> {
    let app = App::new("trashctl")
        .about("A command line interface for trash")
        .author("0xMRTT <0xMRTT@tuta.io>")
        .version(VERSION)
        .setting(AppSettings::PropagateVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .after_help(
            "For more information about a specific command, try `trashctl <command> --help`\n\
             The source code for trashctl is available at: https://github.com/0xMRTT/trashctl",
        )
        .long_about("Trash management tool written in Rust
               
This tool is used to manage trash files in your system.
It is useful to clean up your system trash files.
It is also useful to manage your trash files in a centralized way.

https://github.com/0xMRTT/trashctl")
        .subcommand(cmd::empty::make_subcommand())
        .subcommand(cmd::list::make_subcommand())
        .subcommand(cmd::put::make_subcommand())
        .subcommand(cmd::restore::make_subcommand())
        .subcommand(cmd::rm::make_subcommand())
        .subcommand(
            App::new("completions")
                .about("Generate shell completions for your shell to stdout")
                .arg(
                    Arg::new("shell")
                        .help("the shell to generate completions for")
                        .value_name("SHELL")
                        .required(true),
                ),
        );

    app
}


fn main() {
    let app = create_clap_app();
    env_logger::init();
    info!("Successfuly initialized logger");
    let res = match app.get_matches().subcommand() {
        Some(("empty", sub_matches)) => cmd::empty::execute(sub_matches),
        Some(("list", sub_matches)) => cmd::list::execute(sub_matches),
        Some(("put", sub_matches)) => cmd::put::execute(sub_matches),
        Some(("restore", sub_matches)) => cmd::restore::execute(sub_matches),
        Some(("rm", sub_matches)) => cmd::rm::execute(sub_matches),
        Some(("completions", sub_matches)) => (|| {
            let shell_arg = sub_matches
                .value_of("shell")
                .ok_or_else(|| anyhow!("Shell name missing."))
                .unwrap().to_lowercase();

            let mut shell: Shell = Shell::Bash;

            match shell_arg.as_str() {
                "bash" => shell=Shell::Bash,
                "elvish" => shell=Shell::Elvish,
                "fish" => shell=Shell::Fish,
                "powershell" => shell=Shell::PowerShell,
                "zsh" => shell=Shell::Zsh,
                &_ => {error!("Shell does not exists")}
            }
            

            let mut complete_app = create_clap_app();
            info!("Generating completions for {:?}", shell.clone());
            clap_complete::generate(
                shell,
                &mut complete_app,
                "crs",
                &mut std::io::stdout().lock(),
            );
            Ok(())
        })(),
        _ => unreachable!(),
    };

    if let Err(e) = res {
        eprintln!("{:?}", e.clone());
        error!("{:?}", e);
        std::process::exit(101);
    }
    
}