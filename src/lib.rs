use std::{env, error, fmt, fs, io};

pub fn run() -> Result<(), GrepError> {
    let params = SearchParams::from_args(env::args().collect())?;
    search(&params)?;
    Ok(())
}

pub fn search(params: &SearchParams) -> Result<(), GrepError> {
    match fs::read_to_string(&params.filename) {
        Err(err) => Err(GrepError::IOError(params.filename.to_string(), err)),
        Ok(contents) => {
            eprintln!("Searching for \"{}\" in {}", &params.term, &params.filename);
            println!("len {}", contents.len());
            Ok(())
        }
    }
}

pub struct SearchParams {
    pub term: String,
    pub filename: String,
}

impl SearchParams {
    fn from_args(args: Vec<String>) -> Result<SearchParams, GrepError>
    {
        // let args: Vec<String> = args.collect();
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
    NotEnoughParams,
    IOError(String, io::Error),
}

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

impl error::Error for GrepError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_params_zero_args() {
        let params = SearchParams::from_args(Vec::new());
        assert!(params.is_err());
    }
    #[test]
    fn search_params_just_prog() {
        let params = SearchParams::from_args(vec!["prog".to_string()]);
        assert!(params.is_err());
    }
    #[test]
    fn search_params_one_arg() {
        let params = SearchParams::from_args(vec!["prog".to_string(), "two".to_string()]);
        assert!(params.is_err());
    }
    #[test]
    fn search_params_two_args() {
        let params = SearchParams::from_args(vec!["prog".to_string(), "two".to_string(), "three".to_string()]);
        let params = params.unwrap();
        assert_eq!(params.term, "two");
        assert_eq!(params.filename, "three");
    }
    #[test]
    fn search_params_three_args() {
        let params = SearchParams::from_args(vec!["prog".to_string(), "two".to_string(), "three".to_string(), "four".to_string()]);
        let params = params.unwrap();
        assert_eq!(params.term, "two three");
        assert_eq!(params.filename, "four");
    }
}