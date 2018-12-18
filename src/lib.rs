// Copyright (C) 2018 Robert A. Wallis, all rights reserved.
use std::{env, error, fmt, fs, io};

/// Checks the program args, as search paramaters.  And then runs a search on the file.
pub fn run() -> Result<(), GrepError> {
    let args: Vec<String> = env::args().collect();
    let params = SearchParams::from_args(&args)?;
    let contents = read_file(&params)?;
    let results = search(&params.term, &contents)?;
    println!("{}", results.join("\n"));
    Ok(())
}

/// Open the appropriate file.
pub fn read_file(params: &SearchParams) -> Result<String, GrepError> {
    match fs::read_to_string(&params.filename) {
        Err(err) => Err(GrepError::IOError(params.filename.to_string(), err)),
        Ok(contents) => Ok(contents),
    }
}

/// Find the text in the contents.
pub fn search<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, GrepError> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    Ok(results)
}

/// Organized set of options for searching a file.
pub struct SearchParams {
    pub term: String,
    pub filename: String,
}

impl SearchParams {
    /// Given &sys::env::args().collect(), from_args() returns a filled out SearchParams.
    fn from_args(args: &[String]) -> Result<SearchParams, GrepError> {
        if args.len() < 3 {
            return Err(GrepError::NotEnoughParams);
        }
        let term = String::from(&args[1..args.len() - 1].join(" ")[..]);
        let filename = String::from(&args[args.len() - 1][..]);
        Ok(SearchParams { term, filename })
    }
}

#[derive(Debug)]
pub enum GrepError {
    /// program needs more agurments to run correctly
    NotEnoughParams,
    /// the OS returned a filesystem error
    IOError(String, io::Error),
}

/// Used by print to turn GrepError enum into a string.
impl fmt::Display for GrepError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GrepError::NotEnoughParams => write!(f, "usage: minigrep keywords filename"),
            GrepError::IOError(filename, err) => match err.kind() {
                io::ErrorKind::NotFound => write!(f, "File {} not found.", filename),
                _ => write!(f, "{}", err),
            },
        }
    }
}

/// Makes GrepError compatable with rust errors, so a function can use dyn Error type.
impl error::Error for GrepError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_params_zero_args() {
        let params = SearchParams::from_args(&Vec::new());
        assert!(params.is_err());
    }
    #[test]
    fn search_params_just_prog() {
        let params = SearchParams::from_args(&vec!["prog".to_string()]);
        assert!(params.is_err());
    }
    #[test]
    fn search_params_one_arg() {
        let params = SearchParams::from_args(&vec!["prog".to_string(), "two".to_string()]);
        assert!(params.is_err());
    }
    #[test]
    fn search_params_two_args() {
        let params = SearchParams::from_args(&vec![
            "prog".to_string(),
            "two".to_string(),
            "three".to_string(),
        ]);
        let params = params.unwrap();
        assert_eq!(params.term, "two");
        assert_eq!(params.filename, "three");
    }
    #[test]
    fn search_params_three_args() {
        let params = SearchParams::from_args(&vec![
            "prog".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
        ]);
        let params = params.unwrap();
        assert_eq!(params.term, "two three");
        assert_eq!(params.filename, "four");
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents).unwrap()
        );
    }
}
