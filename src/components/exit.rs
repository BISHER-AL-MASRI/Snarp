use crate::util::warninghandler::warninghandler;
use colored::*;
use std::collections::HashMap;

pub fn exit_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() != 1 {
        warninghandler(&format!(
            "{} {}",
            format!("Invalid number of arguments for exit:").yellow(),
            args.len()
        ));
        std::process::exit(0);
    }

    let code = match vars.get(args[0]) {
        Some(val) => val.parse::<i32>().unwrap_or_else(|_| {
            warninghandler(&format!(
                "Invalid number: {}, exitting with code 0 instead",
                args[0]
            ));
            std::process::exit(0);
        }),
        None => args[0].parse::<i32>().unwrap_or_else(|_| {
            warninghandler(&format!(
                "Invalid number: {}, exitting with code 0 instead",
                args[0]
            ));
            std::process::exit(0);
        }),
    };

    println!("Exiting with code: {}", code);
    std::process::exit(code);
}
