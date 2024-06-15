use crate::util::debug::is_debug;
use std::collections::HashMap;
use crate::util::errorhandler::errorhandler;

pub fn add_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() != 3 {
        errorhandler(&format!(
            "Invalid number of arguments for add: {}",
            args.len()
        ));
        return;
    }

    let var_name = args[0].to_string();
    let num1 = match vars.get(args[1]) {
        Some(val) => val.parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[1]));
            std::process::exit(1);
        }),
        None => args[1].parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[1]));
            std::process::exit(1);
        }),
    };
    let num2 = match vars.get(args[2]) {
        Some(val) => val.parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[2]));
            std::process::exit(1);
        }),
        None => args[2].parse::<i32>().unwrap_or_else(|_| {
            errorhandler(&format!("Invalid number: {}", args[2]));
            std::process::exit(1);
        }),
    };

    let result = num1 + num2;

    vars.insert(var_name.clone(), result.to_string());

    if is_debug() {
        println!(
            "Adding {} and {} to get {} and storing in variable: {}",
            num1, num2, result, var_name
        );
    }
}
