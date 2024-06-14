use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn func_def_func(
    funcs: &mut HashMap<String, Vec<String>>,
    args: Vec<&str>,
    lines: &mut dyn Iterator<Item = &str>,
) {
    if args.len() != 1 {
        eprintln!("Invalid function definition. Correct format: Func name() {{...}}");
        return;
    }

    let func_name = args[0].to_string();
    let mut func_body = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line == ")" {
            break;
        }
        func_body.push(line.to_string());
    }

    funcs.insert(func_name, func_body);
}

pub fn func_call_func(
    funcs: &HashMap<String, Vec<String>>,
    func_name: &str,
    vars: &mut HashMap<String, String>,
    table: &HashMap<&str, Box<dyn Fn(&mut HashMap<String, String>, Vec<&str>) + Send>>,
) {
    if let Some(func_body) = funcs.get(func_name) {
        for line in func_body {
            let line = line.trim();

            if line.is_empty() {
                continue;
            }

            let words: Vec<&str> = line.split_whitespace().collect();

            if let Some(function) = words.get(0) {
                if let Some(f) = table.get(function) {
                    let args = words[1..].to_vec();
                    f(vars, args);
                } else {
                    eprintln!("Error: Unknown command: {}", line);
                }
            } else {
                eprintln!("Warning: Unknown statement format: {}", line);
            }
        }
    } else {
        eprintln!("Error: Undefined function: {}", func_name);
    }
}