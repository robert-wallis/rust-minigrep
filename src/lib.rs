// Copyright (C) 2018 Robert A. Wallis, all rights reserved.

//! Searches through files like grep, but with less features.
//! ```
//! use minigrep::{SearchParams, search};
//! 
//! let params = SearchParams {
//!     term: "robot".to_string(),
//!     filename: "example.txt".to_string(),
//!     ignore_case: true
//! };
//! 
//! assert_eq!(
//!     vec!["Domo arigato Mr. Roboto."],
//!     search(&params, "Hey there\nDomo arigato Mr. Roboto.\nDomo...\nDomo...")
//! );
//! ```

use std::fs;
mod error;
pub use self::error::GrepError;

/// Find the text in the contents.
pub fn search<'a>(params: &SearchParams, contents: &'a str) -> Vec<&'a str> {
    let term = term_for_params(&params);
    contents
        .lines()
        .filter(|line| line_for_params(&params, line).contains(&term))
        .collect()
}

/// Organized set of options for searching a file.
pub struct SearchParams {
    pub term: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl SearchParams {
    /// Given &sys::env::args().collect(), from_args() returns a filled out SearchParams.
    pub fn from_args(args: &[String]) -> Result<SearchParams, GrepError> {
        if args.len() < 3 {
            return Err(GrepError::NotEnoughParams);
        }
        let (ignore_case, start_arg) = if &args[1] == "-i" {
            (true, 2)
        } else {
            (false, 1)
        };
        let term = String::from(&args[start_arg..args.len() - 1].join(" ")[..]);
        let filename = String::from(&args[args.len() - 1][..]);
        Ok(SearchParams {
            term,
            filename,
            ignore_case,
        })
    }
}

/// Modify the term based on the recipe in the SearchParams.
fn term_for_params(params: &SearchParams) -> String {
    if params.ignore_case {
        params.term.to_lowercase()
    } else {
        params.term.to_string()
    }
}

/// Modify each line based on the recipe in the SearchParams.
fn line_for_params(params: &SearchParams, line: &str) -> String {
    if params.ignore_case {
        line.to_lowercase()
    } else {
        line.to_string()
    }
}

/// Open the appropriate file.
pub fn read_file(params: &SearchParams) -> Result<String, GrepError> {
    match fs::read_to_string(&params.filename) {
        Err(err) => Err(GrepError::IOError(params.filename.to_string(), err)),
        Ok(contents) => Ok(contents),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|&s| String::from(s)).collect::<Vec<String>>()
    }

    #[test]
    fn search_params_zero_args() {
        let params = SearchParams::from_args(&Vec::new());
        assert!(params.is_err());
    }
    #[test]
    fn search_params_just_prog() {
        let params = SearchParams::from_args(&args(&vec!["minigrep"]));
        assert!(params.is_err());
    }
    #[test]
    fn search_params_one_arg() {
        let params = SearchParams::from_args(&args(&vec!["minigrep", "two"]));
        assert!(params.is_err());
    }
    #[test]
    fn search_params_two_args() {
        let params = SearchParams::from_args(&args(&vec!["minigrep", "two", "three"]));
        let params = params.unwrap();
        assert_eq!(params.term, "two");
        assert_eq!(params.filename, "three");
    }
    #[test]
    fn search_params_three_args() {
        let params = SearchParams::from_args(&args(&vec!["minigrep", "two", "three", "four"]));
        let params = params.unwrap();
        assert_eq!(params.term, "two three");
        assert_eq!(params.filename, "four");
    }

    #[test]
    fn search_params_args_case_sensitive() {
        let params =
            SearchParams::from_args(&args(&vec!["minigrep", "-i", "term", "filename"])).unwrap();
        assert_eq!(params.term, "term");
        assert_eq!(params.filename, "filename");
        assert_eq!(params.ignore_case, true);

        let params = SearchParams::from_args(&args(&vec!["minigrep", "term", "filename"])).unwrap();
        assert_eq!(params.term, "term");
        assert_eq!(params.filename, "filename");
        assert_eq!(params.ignore_case, false);
    }

    fn search_params(term: &str) -> SearchParams {
        SearchParams {
            term: String::from(term),
            filename: String::from("test_file"),
            ignore_case: false,
        }
    }

    fn search_params_ignore_case(term: &str) -> SearchParams {
        SearchParams {
            term: String::from(term),
            filename: String::from("test_file"),
            ignore_case: true,
        }
    }

    #[test]
    fn search_one_result() {
        let query = search_params("Robert");
        assert_eq!(vec!["Robert"], search(&query, "Rob\nRobert"));
    }

    #[test]
    fn search_two_results() {
        let query = search_params("Rob");
        assert_eq!(vec!["Rob", "Robert"], search(&query, "Rob\nRobert"));
    }

    #[test]
    fn search_zero_results() {
        let query = search_params("Bob");
        let empty: Vec<String> = Vec::new();
        assert_eq!(empty, search(&query, "Rob\nRobert"));
    }

    #[test]
    fn search_case_sensitive() {
        let query = search_params_ignore_case("rob");
        assert_eq!(vec!["Rob", "Robert"], search(&query, "Rob\nRobert"));
    }
}
