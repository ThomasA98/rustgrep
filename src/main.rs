use std::{env, fs, io, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            println!("Error: {err}");
            process::exit(1);
        },
    };

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

}

struct Config {
    word_search: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        let Some(word_search) = args.get(1) else {
            return  Err("Should have a word to search")
        };

        let Some(file_path) = args.get(2) else {
            return  Err("We need a file to search")
        };

        Ok(Config {
            word_search: word_search.clone(),
            file_path: file_path.clone(),
        })
    }
}

fn run(config: Config) -> io::Result<()> {
    let content = fs::read_to_string(config.file_path)?;

    println!("{content}");

    Ok(())
}