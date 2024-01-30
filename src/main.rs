use json_parser::{Parser, Scanner};
use std::{env, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the path to json file.");
        return ExitCode::from(65);
    }
    let filepath = &args[1];
    let content = std::fs::read_to_string(filepath);
    if let Ok(source) = content {
        let mut scanner = Scanner::from(source);
        let parser = Parser::from(scanner.scan_tokens());
        if parser.is_valid() {
            return ExitCode::from(0);
        }
    }
    ExitCode::from(1)
}
