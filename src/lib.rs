pub mod config;
pub mod helpers;

use std::{fs, io};
use self::config::{Config, CaseMode};
use self::helpers::{ search, search_case_insensitive };

pub fn run(config: Config) -> io::Result<()> {
    let content = fs::read_to_string(config.file_path)?;

    let lines = match config.ignore_case {
        CaseMode::Sensitive => search(&config.word_search, &content),
        CaseMode::InSensitive => search_case_insensitive(&config.word_search, &content),
    };

    for (index, line) in lines.iter().enumerate() {
        println!("{index}: {line}");
    }

    Ok(())
}