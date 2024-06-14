use colored::*;
use std::collections::HashMap;

pub fn return_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.is_empty() {
        eprintln!(
            "{}",
            format!("No argument provided for return function").red()
        );
        return;
    }

    let arg = args[0];
    let num = if let Ok(num) = arg.parse::<i32>() {
        num
    } else if let Some(value) = vars.get(arg) {
        value.parse::<i32>().unwrap_or_else(|_| {
            eprintln!("{} {}", format!("Invalid number:").red(), arg);
            std::process::exit(1);
        })
    } else {
        eprintln!("{} {}", format!("Unknown variable:").red(), arg);
        std::process::exit(1);
    };

    println!("Returning with code: {}", num);
    std::process::exit(num);
}
