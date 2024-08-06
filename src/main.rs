use std::{ env, process };
use rustgrep::config::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = match Config::build(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        },
    };

    if let Err(e) = rustgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}