use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage:\t{} keywords filename", args[0]);
        return
    }
    let query = &args[1..args.len()-1].join(" ");
    let file = &args[args.len()-1];
    eprintln!("Searching for \"{}\" in {}", query, file);
}
