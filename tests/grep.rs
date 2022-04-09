// https://www.thegeekstuff.com/2009/03/15-practical-unix-grep-command-examples/

use std::collections::HashSet;
use std::path::PathBuf;

use minigrep::grep;
use minigrep::grep_config::{GrepConfig, GrepOption};

#[test]
fn test_grep_literal_string_single_file1() {
    let queries = vec!["this".to_string()];
    let filenames = vec![PathBuf::from("demo_file.txt".to_string())];
    let config = GrepConfig {
        queries,
        filenames,
        options: HashSet::new(),
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("this line is the 1st lower case line in this file.".to_string(), matches[0]);
    assert_eq!("Two lines above this line is empty.".to_string(), matches[1]);
    assert_eq!("And this is the last line.".to_string(), matches[2]);
}

#[test]
fn test_grep_literal_string_single_file2() {
    let args = vec![
        "./minigrep".to_string(),
        "this".to_string(),
        "demo_file.txt".to_string()];
    let config = GrepConfig::new(&args).unwrap();
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("this line is the 1st lower case line in this file.".to_string(), matches[0]);
    assert_eq!("Two lines above this line is empty.".to_string(), matches[1]);
    assert_eq!("And this is the last line.".to_string(), matches[2]);
}

#[test]
fn test_grep_literal_string_single_file3() {
    let args = vec![
        "./minigrep".to_string(),
        "-e".to_string(),
        "this".to_string(),
        "demo_file.txt".to_string()];
    let config = GrepConfig::new(&args).unwrap();
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("this line is the 1st lower case line in this file.".to_string(), matches[0]);
    assert_eq!("Two lines above this line is empty.".to_string(), matches[1]);
    assert_eq!("And this is the last line.".to_string(), matches[2]);
}

#[test]
fn test_grep_literal_string_multiple_files1() {
    let queries = vec!["this".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string()),
        PathBuf::from("demo_file1.txt".to_string())];
    let config = GrepConfig {
        queries,
        filenames,
        options: HashSet::new(),
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("demo_file.txt:this line is the 1st lower case line in this file.".to_string(), matches[0]);
    assert_eq!("demo_file.txt:Two lines above this line is empty.".to_string(), matches[1]);
    assert_eq!("demo_file.txt:And this is the last line.".to_string(), matches[2]);
    assert_eq!("demo_file1.txt:this line is the 1st lower case line in this file.".to_string(), matches[3]);
    assert_eq!("demo_file1.txt:Two lines above this line is empty.".to_string(), matches[4]);
    assert_eq!("demo_file1.txt:And this is the last line.".to_string(), matches[5]);
}

#[test]
fn test_grep_literal_string_multiple_files2() {
    let args = vec![
        "./minigrep".to_string(),
        "this".to_string(),
        "demo_*.txt".to_string()];
    let config = GrepConfig::new(&args).unwrap();
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("demo_file.txt:this line is the 1st lower case line in this file.".to_string(), matches[0]);
    assert_eq!("demo_file.txt:Two lines above this line is empty.".to_string(), matches[1]);
    assert_eq!("demo_file.txt:And this is the last line.".to_string(), matches[2]);
    assert_eq!("demo_file1.txt:this line is the 1st lower case line in this file.".to_string(), matches[3]);
    assert_eq!("demo_file1.txt:Two lines above this line is empty.".to_string(), matches[4]);
    assert_eq!("demo_file1.txt:And this is the last line.".to_string(), matches[5]);
}

#[test]
fn test_grep_case_insensitive() {
    let queries = vec!["the".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string())];
    let options = vec![
        GrepOption::CaseInsensitive].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("THIS LINE IS THE 1ST UPPER CASE LINE IN THIS FILE.".to_string(), matches[0]);
    assert_eq!("this line is the 1st lower case line in this file.".to_string(), matches[1]);
    assert_eq!("This Line Has All Its First Character Of The Word With Upper Case.".to_string(), matches[2]);
    assert_eq!("And this is the last line.".to_string(), matches[3]);
}

#[test]
fn test_grep_regex() {
    let queries = vec!["lines.*empty".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string())];
    let options = HashSet::new();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("Two lines above this line is empty.".to_string(), matches[0]);
}

#[test]
fn test_grep_word() {
    let queries = vec!["is".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string())];
    let options = vec![
        GrepOption::CaseInsensitive,
        GrepOption::Word].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("THIS LINE IS THE 1ST UPPER CASE LINE IN THIS FILE.".to_string(), matches[0]);
    assert_eq!("this line is the 1st lower case line in this file.".to_string(), matches[1]);
    assert_eq!("Two lines above this line is empty.".to_string(), matches[2]);
    assert_eq!("And this is the last line.".to_string(), matches[3]);
}

#[test]
fn test_grep_invert() {
    let queries = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string()];
    let filenames = vec![
        PathBuf::from("test-file.txt".to_string())];
    let options = vec![
        GrepOption::Invert].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let matches = grep::grep(&config).unwrap().to_lines(false);
    assert_eq!("d".to_string(), matches[0]);
}

#[test]
fn test_grep_line_number() {
    let queries = vec![
        "go".to_string()];
    let filenames = vec![
        PathBuf::from("demo_text.txt".to_string())];
    let options = vec![
        GrepOption::LineNumber].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("5: * e - go to the end of the current word.
6: * E - go to the end of the current WORD.
7: * b - go to the previous (before) word.
8: * B - go to the previous (before) WORD.
9: * w - go to the next word.
10: * W - go to the next WORD.".to_string(), s);
}

#[test]
fn test_grep_count1() {
    let queries = vec![
        "this".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string())];
    let options = vec![
        GrepOption::Count].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("3".to_string(), s);
}

#[test]
fn test_grep_count2() {
    let queries = vec![
        "is".to_string()];
    let filenames = vec![
        PathBuf::from("demo_file.txt".to_string())];
    let options = vec![
        GrepOption::Invert,
        GrepOption::Count].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 0,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("2".to_string(), s);
}

#[test]
fn test_grep_after() {
    let queries = vec![
        "example".to_string()];
    let filenames = vec![
        PathBuf::from("demo_text.txt".to_string())];
    let options = vec![
        GrepOption::CaseInsensitive].into_iter().collect();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 0,
        num_post_lines: 3,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("Example to show the difference between WORD and word

 * 192.168.1.1 - single WORD
 * 192.168.1.1 - seven words.".to_string(), s);
}

#[test]
fn test_grep_before() {
    let queries = vec![
        "single WORD".to_string()];
    let filenames = vec![
        PathBuf::from("demo_text.txt".to_string())];
    let options = HashSet::new();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 2,
        num_post_lines: 0,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("Example to show the difference between WORD and word

 * 192.168.1.1 - single WORD".to_string(), s);
}

#[test]
fn test_grep_around() {
    let queries = vec![
        "Example".to_string()];
    let filenames = vec![
        PathBuf::from("demo_text.txt".to_string())];
    let options = HashSet::new();
    let config = GrepConfig {
        queries,
        filenames,
        options,
        num_pre_lines: 2,
        num_post_lines: 2,
    };
    let s = grep::grep(&config).unwrap().to_string(&config);
    assert_eq!("word - word consists of a sequence of letters, digits and underscores.

Example to show the difference between WORD and word

 * 192.168.1.1 - single WORD".to_string(), s);
}
