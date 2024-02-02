use json_parser::{Scanner, Validator};
use std::{env, fs, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to json file.");
        return ExitCode::from(65);
    }
    if let Ok(source) = fs::read_to_string(&args[1]) {
        let mut scanner = Scanner::from(source);
        let validator = Validator::from(scanner.scan_tokens());
        if validator.valid() {
            println!("JSON is valid");
            return ExitCode::from(0);
        } else {
            eprintln!("JSON is invalid");
            return ExitCode::from(1);
        }
    }
    ExitCode::from(1)
}
