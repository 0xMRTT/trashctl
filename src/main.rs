mod cli;

fn main() {
    let matches = cli::build_cli().get_matches();

    if matches.is_present("completion") {
        cli::generate_completion(matches.value_of("completion").unwrap());
    }
        
}
