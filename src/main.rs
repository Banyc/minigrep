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
                ConfigError::UnknownOption(x) => println!("Unknown option: {}", x),
                ConfigError::MissingArg(x) => println!("Missing arg: {}", x),
                ConfigError::TooManyArgs => println!("Too many args"),
                ConfigError::Filename(x) => println!("File name error: {}", x),
                ConfigError::IllegalOptionValue(x) => println!("Illegal operation value: {}", x),
            }
            println!("usage: minigrep [options] [query_string] filename");
            return;
        }
    };

    let grep_result = match grep::grep(&config) {
        Ok(x) => x,
        Err(err) => {
            match err {
                GrepError::FileNotExists => println!("File not exists"),
                GrepError::NotValidUTF8 => println!("File contents not being valid utf-8"),
                GrepError::Query(x) => println!("Query error: {}", x),
            }
            return;
        }
    };

    let s = grep_result.to_string(&config);
    println!("{}", s);
}
