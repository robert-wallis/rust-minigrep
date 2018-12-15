use minigrep;

fn main() {
    if let Err(err) = minigrep::run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
