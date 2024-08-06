use std::env;

const QUERY_INDEX: usize = 1;
const FILE_INDEX: usize = 2;
const IGNORE_CASE: &str = "IGNORE_CASE";
const CASE_INSENSITIVE: &str = "insensitive";

pub enum CaseMode {
    Sensitive,
    InSensitive,
}

pub struct Config {
    pub word_search: String,
    pub file_path: String,
    pub ignore_case: CaseMode,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        let Some(word_search) = args.get(QUERY_INDEX) else {
            return Err("Should have a word to search");
        };

        let Some(file_path) = args.get(FILE_INDEX) else {
            return Err("We need a file to search");
        };

        let ignore_case = match env::var(IGNORE_CASE) {
            Ok(case) if case.to_lowercase() == CASE_INSENSITIVE => CaseMode::InSensitive,
            Ok(_) => CaseMode::Sensitive,
            Err(_) => CaseMode::Sensitive,
        };

        let config = Self {
            word_search: word_search.clone(),
            file_path: file_path.clone(),
            ignore_case,
        };

        Ok(config)
    }
}