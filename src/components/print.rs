use crate::util::debug::is_debug;
use std::collections::HashMap;
use crate::util::errorhandler::errorhandler;

pub fn print_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    let output: Vec<String> = args
        .iter()
        .map(|arg| {
            if arg.starts_with('*') {
                let var_name = &arg[1..];
                vars.get(var_name)
                    .cloned()
                    .unwrap_or_else(|| {
                        errorhandler(&format!("Unknown variable: {}", var_name));
                        std::process::exit(1);
                    })
            } else {
                arg.to_string()
            }
        })
        .collect();

    if is_debug() {
        println!("Executing print with args: {:?}", args);
    }

    println!("{}", output.join(" "));
}
