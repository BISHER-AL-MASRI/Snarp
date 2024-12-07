mod functions;
mod parser;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: snarp <file_path>");
        return;
    }

    let file_path = &args[1];

    let content = fs::read_to_string(file_path).expect("Could not read the file");

    let mut functions = functions::built_in_functions();
    functions.extend(parser::process_snarp_code(&content));

    parser::process_top_level_statements(&content, &functions);
}
