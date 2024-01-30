use json_parser::Scanner;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to json file.");
        return;
    }
    let filepath = &args[1];
    let content = std::fs::read_to_string(filepath);
    if let Ok(source) = content {
        let mut scanner = Scanner::from(source);
        scanner.scan_tokens();
    }
}
