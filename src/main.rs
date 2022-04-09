use std::env;

use minigrep::{
    grep::{self, GrepError},
    grep_config::{self, ConfigError},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match grep_config::GrepConfig::new(&args) {
        Ok(config) => config,
        Err(err) => {
            match err {
                ConfigError::UnknownOption(x) => eprintln!("Unknown option: {}", x),
                ConfigError::MissingArg(x) => eprintln!("Missing arg: {}", x),
                ConfigError::TooManyArgs => eprintln!("Too many args"),
                ConfigError::Filename(x) => eprintln!("File name error: {}", x),
                ConfigError::IllegalOptionValue(x) => eprintln!("Illegal option value: {}", x),
            }
            eprintln!("\
usage: minigrep [options] [query_string] filename
  options:
    -i           ignore case distinctions in both the query string and the file contents
    -w           match only whole words
    -v           select non-matching lines
    -n           print line numbers with output lines
    -c           print only a count of matching lines
    -e pattern   set the query string
    -A N         print N lines of leading context before matching lines
    -B N         print N lines of trailing context after matching lines
    -C N         print N lines of context surrounding matching lines");
            return;
        }
    };

    let grep_result = match grep::grep(&config) {
        Ok(x) => x,
        Err(err) => {
            match err {
                GrepError::FileNotExists => eprintln!("File not exists"),
                GrepError::NotValidUTF8 => eprintln!("File contents not being valid utf-8"),
                GrepError::Query(x) => eprintln!("Query error: {}", x),
            }
            return;
        }
    };

    let s = grep_result.to_string(&config);
    println!("{}", s);
}
