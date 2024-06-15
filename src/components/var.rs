use crate::util::debug::is_debug;
use std::collections::HashMap;
use crate::util::errorhandler::errorhandler;

pub fn var_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() != 2 {
        errorhandler(&format!(
            "Invalid number of arguments for var: {}",
            args.len()
        ));
        return;
    }

    let var = args[0].to_string();
    let value = args[1].to_string();

    vars.insert(var.clone(), value.clone());

    if is_debug() {
        println!("Creating variable: {} with value: {}", var, value);
    }
}
