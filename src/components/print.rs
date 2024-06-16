use crate::util::errorhandler::errorhandler;
use std::collections::HashMap;

pub fn print_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    let prefix = args[0];
    let var_name = args[..1].join(" ");

    if let Some(value) = vars.get(&var_name) {
        // Check if value should be printed as a string literal
        let print_value = if value.starts_with('"') && value.ends_with('"') {
            &value[1..value.len() - 1] // Remove surrounding quotes
        } else {
            value // Print value as is
        };

        println!("{} {}", prefix, print_value);
    } else {
        errorhandler(&format!("Unknown variable: {}", var_name));
    }
}
