use colored::*;
use std::collections::HashMap;

pub fn exit_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() != 1 {
        eprintln!("{}", format!("Invalid number of arguments for exit").red());
        return;
    }

    let code = match vars.get(args[0]) {
        Some(val) => val.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Invalid number: {}", args[0]);
            std::process::exit(1);
        }),
        None => args[0].parse::<i32>().unwrap_or_else(|_| {
            eprintln!("Invalid number: {}", args[0]);
            std::process::exit(1);
        }),
    };

    println!("Exiting with code: {}", code);
    std::process::exit(code);
}
