use crate::util::debug::is_debug;
use crate::util::errorhandler::errorhandler;
use std::collections::HashMap;

pub fn let_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.len() < 3 || args[1] != "=" {
        errorhandler("Invalid syntax. Correct usage: let varname = expression");
        return;
    }

    let var = args[0].to_string();
    let expression = args[2..].join(" ");

    match eval_expression(&expression, vars) {
        Ok(value) => {
            let value_str = value.to_string();
            vars.insert(var.clone(), value_str.clone());

            if is_debug() {
                println!("Creating variable: {} with value: {}", var, value_str);
            }
        }
        Err(e) => {
            errorhandler(&format!("Error evaluating expression: {}", e));
        }
    }
}

fn eval_expression(expr: &str, vars: &HashMap<String, String>) -> Result<i32, String> {
    let mut tokens = vec![];
    let mut num = String::new();
    let mut last_op = '+';
    let mut last_was_op = true;

    for c in expr.chars() {
        if c.is_digit(10) {
            num.push(c);
            last_was_op = false;
        } else if "+-*/".contains(c) {
            if num.is_empty() && !last_was_op {
                return Err(format!("Invalid syntax: {}", expr));
            }
            if !num.is_empty() {
                tokens.push((last_op, parse_number_or_variable(&num, vars)?));
                num.clear();
            }
            last_op = c;
            last_was_op = true;
        } else if c.is_alphabetic() {
            num.push(c);
            last_was_op = false;
        } else if !c.is_whitespace() {
            return Err(format!("Invalid character: {}", c));
        }
    }

    if !num.is_empty() {
        tokens.push((last_op, parse_number_or_variable(&num, vars)?));
    }

    let mut result = 0;
    let mut current_op = '+';
    for (op, n) in tokens {
        match current_op {
            '+' => result += n,
            '-' => result -= n,
            '*' => result *= n,
            '/' => {
                if n == 0 {
                    return Err("Division by zero".to_string());
                }
                result /= n;
            }
            _ => unreachable!(),
        }
        current_op = op;
    }

    Ok(result)
}

fn parse_number_or_variable(token: &str, vars: &HashMap<String, String>) -> Result<i32, String> {
    if let Ok(n) = token.parse::<i32>() {
        Ok(n)
    } else if let Some(val) = vars.get(token) {
        val.parse::<i32>().map_err(|_| format!("Invalid variable value: {}", val))
    } else {
        Err(format!("Unknown variable: {}", token))
    }
}