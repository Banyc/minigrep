use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    path,
};

use regex::Regex;

use crate::grep_config::{self, GrepConfig, GrepOption};

#[derive(Debug)]
pub enum GrepError {
    FileNotExists,
    NotValidUTF8,
    Query(String),
}

struct LineResult {
    line_number: i32,
    text: String,
}

struct FileResult {
    file_path: path::PathBuf,
    lines: Vec<LineResult>,
}

pub struct GrepResult {
    file_results: Vec<FileResult>,
}

impl GrepResult {
    fn check_rep(&self) {}
    pub fn new(config: &grep_config::GrepConfig) -> Result<GrepResult, GrepError> {
        let mut file_results = Vec::new();
        let mut queries = Vec::new();
        for query_str in &config.queries {
            let mut query_str = query_str.clone();
            // case insensitive
            if config.options.contains(&GrepOption::CaseInsensitive) {
                query_str = query_str.to_lowercase();
            }
            // word
            if config.options.contains(&GrepOption::Word) {
                let mut tmp = "\\W".to_string();
                tmp.push_str(&query_str);
                tmp.push_str("\\W");
                query_str = tmp;
            }
            let re = match Regex::new(&query_str) {
                Ok(x) => x,
                Err(_) => return Err(GrepError::Query(query_str)),
            };
            queries.push(re);
        }
        for filename in &config.filenames {
            let single_file_config = GrepSingleFileConfig {
                options: &config.options,
                queries: &queries,
                filename: &filename,
                num_pre_lines: config.num_pre_lines,
                num_post_lines: config.num_post_lines,
            };
            let file_result = FileResult::new(&single_file_config)?;
            file_results.push(file_result);
        }
        let grep_result = GrepResult { file_results };
        grep_result.check_rep();
        return Ok(grep_result);
    }
    pub fn to_string(&self, config: &GrepConfig) -> String {
        let mut s = String::new();
        let lines = self.to_lines(config.options.contains(&GrepOption::LineNumber));
        if config.options.contains(&GrepOption::Count) {
            s.push_str(&lines.len().to_string());
            return s;
        }
        for line in lines {
            s.push_str(&line);
            s.push('\n');
        }
        if s.ends_with('\n') {
            s.pop();
        }
        return s;
    }
    pub fn to_lines(&self, is_with_line_num: bool) -> Vec<String> {
        let mut matches = Vec::new();
        for file_result in &self.file_results {
            for line in &file_result.lines {
                let mut hinted_line = String::new();
                if self.file_results.len() > 1 {
                    hinted_line.push_str(
                        &file_result
                            .file_path
                            .file_name()
                            .expect("impossible")
                            .to_str()
                            .unwrap()
                            .to_string(),
                    );
                    hinted_line.push_str(":");
                }
                if is_with_line_num {
                    hinted_line.push_str(&line.line_number.to_string());
                    hinted_line.push_str(":");
                }
                hinted_line.push_str(&line.text);
                matches.push(hinted_line);
            }
        }
        return matches;
    }
}

struct GrepSingleFileConfig<'a> {
    options: &'a HashSet<grep_config::GrepOption>,
    queries: &'a Vec<regex::Regex>,
    filename: &'a path::PathBuf,
    num_pre_lines: usize,
    num_post_lines: usize,
}

impl FileResult {
    fn check_rep(&self) {}
    fn new(config: &GrepSingleFileConfig) -> Result<FileResult, GrepError> {
        let f = match File::open(config.filename) {
            Ok(f) => f,
            Err(_err) => return Err(GrepError::FileNotExists),
        };
        let mut reader = BufReader::new(f);
        let mut buffer = String::new();
        let mut matches = Vec::new();
        let mut line_number = 1;
        let mut prev_unmatched_lines = VecDeque::new();
        let mut post_lines_left = 0;

        while match reader.read_line(&mut buffer) {
            Ok(num_bytes) => num_bytes,
            Err(_err) => return Err(GrepError::NotValidUTF8),
        } > 0
        {
            // trim trailing newline
            if buffer.ends_with('\n') {
                // remove the last `\n`
                buffer.pop();

                if buffer.ends_with('\r') {
                    // for Windows
                    buffer.pop();
                }
            }
            let original_buffer = buffer.clone();
            // case insensitive
            if config.options.contains(&GrepOption::CaseInsensitive) {
                buffer = buffer.to_lowercase();
            }
            // pattern matching
            let mut is_matched = false;
            for query in config.queries {
                if query.is_match(&buffer) {
                    is_matched = true;
                    break;
                }
            }
            // invert, context
            let line_result = LineResult {
                line_number,
                text: original_buffer,
            };
            if is_matched != config.options.contains(&GrepOption::Invert) {
                while prev_unmatched_lines.len() > 0 {
                    matches.push(prev_unmatched_lines.pop_front().expect("impossible"));
                }
                matches.push(line_result);
                post_lines_left = config.num_post_lines;
            } else {
                if post_lines_left > 0 {
                    matches.push(line_result);
                    post_lines_left -= 1;
                } else {
                    prev_unmatched_lines.push_back(line_result);
                    while prev_unmatched_lines.len() > config.num_pre_lines {
                        prev_unmatched_lines.pop_front();
                    }
                }
            }
            // clear buffer for next line
            buffer.clear();
            line_number += 1;
        }

        let file_result = FileResult {
            file_path: config.filename.clone(),
            lines: matches,
        };
        file_result.check_rep();
        return Ok(file_result);
    }
}
