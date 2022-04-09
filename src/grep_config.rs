use std::{
    collections::HashSet,
    fs,
    path::{self, PathBuf},
};

#[derive(PartialEq, Eq, Hash)]
pub enum GrepOption {
    CaseInsensitive, // i
    Word,            // w
    Invert,          // v
    LineNumber,      // n
    Count,           // c
}

#[derive(Debug)]
pub enum ConfigError {
    UnknownOption(String),
    MissingArg(String),
    TooManyArgs,
    Filename(String),
    IllegalOptionValue(String),
}

pub struct GrepConfig {
    pub options: HashSet<GrepOption>,
    pub queries: Vec<String>,
    pub filenames: Vec<PathBuf>,
    pub num_pre_lines: usize,
    pub num_post_lines: usize,
}

impl GrepConfig {
    fn check_rep(&self) {
        if self.queries.len() == 0 {
            panic!("GrepConfig.queries.len() == 0");
        }
        if self.filenames.len() == 0 {
            panic!("GrepConfig.filenames.len() == 0");
        }
    }
    pub fn new(args: &Vec<String>) -> Result<GrepConfig, ConfigError> {
        #[derive(PartialEq)]
        enum State {
            Start,
            Option,
            QueryString,
            Filename,
            Done,
        }
        let mut state = State::Start;
        let mut arg_index = 1;
        let mut config = GrepConfig {
            options: HashSet::new(),
            queries: Vec::new(),
            filenames: Vec::new(),
            num_pre_lines: 0,
            num_post_lines: 0,
        };

        while state != State::Done {
            if arg_index >= args.len() {
                match state {
                    State::Start => return Err(ConfigError::MissingArg("query".to_string())),
                    State::Option => return Err(ConfigError::MissingArg("query".to_string())),
                    State::QueryString => return Err(ConfigError::MissingArg("query".to_string())),
                    State::Filename => return Err(ConfigError::MissingArg("filename".to_string())),
                    State::Done => panic!("impossible"),
                }
            }
            state = match state {
                State::Start => State::Option,
                State::Option => {
                    let arg = &args[arg_index];
                    if arg.as_bytes()[0] as char == '-' {
                        for char_index in 1..arg.len() {
                            let option = match arg.as_bytes()[char_index] as char {
                                'i' => Some(GrepOption::CaseInsensitive),
                                'w' => Some(GrepOption::Word),
                                'v' => Some(GrepOption::Invert),
                                'n' => Some(GrepOption::LineNumber),
                                'c' => Some(GrepOption::Count),
                                'e' => {
                                    arg_index += 1;
                                    config.queries.push(args[arg_index].clone());
                                    None
                                }
                                c if c == 'A' || c == 'B' || c == 'C' => {
                                    arg_index += 1;
                                    let size: usize = match args[arg_index].parse() {
                                        Ok(x) => x,
                                        Err(_) => {
                                            return Err(ConfigError::IllegalOptionValue(
                                                args[arg_index].clone(),
                                            ))
                                        }
                                    };
                                    match c {
                                        'A' => config.num_post_lines = size,
                                        'B' => config.num_pre_lines = size,
                                        'C' => {
                                            config.num_pre_lines = size;
                                            config.num_post_lines = size;
                                        }
                                        _ => panic!("impossible"),
                                    }
                                    None
                                }
                                other_char => {
                                    return Err(ConfigError::UnknownOption(other_char.to_string()))
                                }
                            };
                            if option.is_some() {
                                config.options.insert(option.expect("impossible"));
                            }
                        }
                        arg_index += 1;
                        State::Option
                    } else {
                        State::QueryString
                    }
                }
                State::QueryString => {
                    if arg_index < args.len() - 1 {
                        // there is still place for filename
                        config.queries.push(args[arg_index].clone());
                        arg_index += 1;
                    }
                    State::Filename
                }
                State::Filename => {
                    let wildcard = &args[arg_index];
                    lookup_filenames(&mut config.filenames, wildcard)?;
                    arg_index += 1;
                    if args.len() != arg_index {
                        return Err(ConfigError::TooManyArgs);
                    }
                    State::Done
                }
                State::Done => panic!("impossible"),
            };
        }
        if config.queries.len() == 0 {
            return Err(ConfigError::MissingArg("query".to_string()));
        }
        config.check_rep();
        return Ok(config);
    }
}

fn lookup_filenames(
    filenames: &mut Vec<path::PathBuf>,
    wildcard: &String,
) -> Result<(), ConfigError> {
    let path = PathBuf::from(&wildcard);
    if path.is_file() {
        filenames.push(path);
        return Ok(());
    }

    let filename = match path.file_name() {
        Some(filename) => filename.to_str().unwrap(),
        None => return Err(ConfigError::Filename(wildcard.clone())),
    };

    if filename.contains("*") || filename.contains("?") {
        let wild_match = wildmatch::WildMatch::new(filename);
        let parent = match path.parent() {
            Some(parent) => parent,
            None => return Err(ConfigError::Filename(wildcard.clone())),
        };
        let paths = match fs::read_dir(parent) {
            Ok(paths) => paths,
            Err(_err) => return Err(ConfigError::Filename(wildcard.clone())),
        };
        for path in paths {
            let path = match path {
                Ok(path) => path.path(),
                Err(_err) => return Err(ConfigError::Filename(wildcard.clone())),
            };
            let filename = match path.file_name() {
                Some(filename) => filename.to_str().unwrap(),
                None => continue,
            };
            if path.is_file() && wild_match.matches(filename) {
                filenames.push(path);
            }
        }
        return Ok(());
    }

    return Err(ConfigError::Filename(wildcard.clone()));
}
