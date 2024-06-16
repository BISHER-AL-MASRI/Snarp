use crate::util::errorhandler::errorhandler;
use std::collections::HashMap;

pub fn let_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() < 3 {
        errorhandler(&format!(
            "Invalid number of arguments for let: {} (minimum 3 required)",
            args.len()
        ));
        return;
    }

    let var_name = args[0].to_string();
    let assignment_op = args[1];
    let value = args[2..].join(" ");

    match assignment_op {
        "=" => {
            vars.insert(var_name.clone(), value);
        }
        _ => {
            errorhandler(&format!(
                "Unsupported assignment operator: {}",
                assignment_op
            ));
        }
    }
}
