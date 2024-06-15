use colored::*;
use std::collections::HashMap;

use crate::util::errorhandler::errorhandler;

pub fn exit_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() != 1 {
        errorhandler(&format!(
            "{} {}",
            format!("Invalid number of arguments for exit:").red(),
            args.len()
        ));
        std::process::exit(0);
    }

    let code = match vars.get(args[0]) {
        Some(val) => val.parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[0]));
            std::process::exit(0);
        }),
        None => args[0].parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[0]));
            std::process::exit(0);
        }),
    };

    println!("Exiting with code: {}", code);
    std::process::exit(code);
}
