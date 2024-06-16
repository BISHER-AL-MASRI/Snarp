use crate::util::errorhandler::errorhandler;
use std::collections::HashMap;

pub fn assign_func(vars: &mut HashMap<String, String>, args: &[&str]) {
    if args.len() != 3 {
        errorhandler("Invalid number of arguments for assignment");
        return;
    }

    let var_name = args[0].to_string();
    let assignment_op = args[1];
    let value = args[2];

    if !vars.contains_key(var_name.as_str()) {
        errorhandler(&format!("Unknown variable: {}", var_name));
        return;
    }

    let current_value = vars.get(var_name.as_str()).unwrap().clone();

    let new_value = match assignment_op {
        "=" => value.to_string(),
        "+=" => {
            let result = add_values(&current_value, value);
            if result.is_err() {
                errorhandler(&format!("Error adding values: {}", result.unwrap_err()));
                return;
            }
            result.unwrap()
        }
        "-=" => {
            let result = subtract_values(&current_value, value);
            if result.is_err() {
                errorhandler(&format!(
                    "Error subtracting values: {}",
                    result.unwrap_err()
                ));
                return;
            }
            result.unwrap()
        }
        "*=" => {
            let result = multiply_values(&current_value, value);
            if result.is_err() {
                errorhandler(&format!(
                    "Error multiplying values: {}",
                    result.unwrap_err()
                ));
                return;
            }
            result.unwrap()
        }
        "/=" => {
            let result = divide_values(&current_value, value);
            if result.is_err() {
                errorhandler(&format!("Error dividing values: {}", result.unwrap_err()));
                return;
            }
            result.unwrap()
        }
        _ => {
            errorhandler(&format!(
                "Unsupported assignment operator: {}",
                assignment_op
            ));
            return;
        }
    };

    vars.insert(var_name, new_value);
}

fn add_values(current_value: &str, value: &str) -> Result<String, String> {
    let current: Result<i32, _> = current_value.parse();
    let value: Result<i32, _> = value.parse();

    match (current, value) {
        (Ok(curr), Ok(val)) => Ok((curr + val).to_string()),
        _ => Err("Invalid integer values".to_string()),
    }
}

fn subtract_values(current_value: &str, value: &str) -> Result<String, String> {
    let current: Result<i32, _> = current_value.parse();
    let value: Result<i32, _> = value.parse();

    match (current, value) {
        (Ok(curr), Ok(val)) => Ok((curr - val).to_string()),
        _ => Err("Invalid integer values".to_string()),
    }
}

fn multiply_values(current_value: &str, value: &str) -> Result<String, String> {
    let current: Result<i32, _> = current_value.parse();
    let value: Result<i32, _> = value.parse();

    match (current, value) {
        (Ok(curr), Ok(val)) => Ok((curr * val).to_string()),
        _ => Err("Invalid integer values".to_string()),
    }
}

fn divide_values(current_value: &str, value: &str) -> Result<String, String> {
    let current: Result<i32, _> = current_value.parse();
    let value: Result<i32, _> = value.parse();

    match (current, value) {
        (Ok(curr), Ok(val)) => {
            if val == 0 {
                Err("Division by zero".to_string())
            } else {
                Ok((curr / val).to_string())
            }
        }
        _ => Err("Invalid integer values".to_string()),
    }
}
