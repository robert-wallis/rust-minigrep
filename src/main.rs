use std::{env, error, fmt, fs, io};

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), GrepError> {
    let params = SearchParams::from_args(env::args())?;
    search(&params)?;
    Ok(())
}

fn search(params: &SearchParams) -> Result<(), GrepError> {
    match fs::read_to_string(&params.filename) {
        Err(err) => Err(GrepError::IOError(params.filename.to_string(), err)),
        Ok(contents) => {
            eprintln!("Searching for \"{}\" in {}", &params.term, &params.filename);
            println!("len {}", contents.len());
            Ok(())
        }
    }
}

struct SearchParams {
    term: String,
    filename: String,
}

impl SearchParams {
    fn from_args(args: env::Args) -> Result<SearchParams, GrepError> {
        let args: Vec<String> = args.collect();
        if args.len() < 3 {
            return Err(GrepError::NotEnoughParams);
        }
        let term = String::from(&args[1..args.len() - 1].join(" ")[..]);
        let filename = String::from(&args[args.len() - 1][..]);
        Ok(SearchParams { term, filename })
    }
}

#[derive(Debug)]
enum GrepError {
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
