use std::collections::HashMap;
use crate::functions::SnarpFunction;

// Main function to process Snarp code
pub fn process_snarp_code(code: &str) -> HashMap<String, SnarpFunction> {
    let mut functions: HashMap<String, SnarpFunction> = HashMap::new();
    let mut lines: Vec<&str> = code.lines().collect();

    // First pass: Define functions
    let mut i = 0;
    while i < lines.len() {
        let _line = lines[i].trim(); // Prefix with underscore to avoid warning
        if let Some((func, end_index)) = parse_function_definition(&lines, i) {
            functions.insert(func.name.clone(), func);
            lines.drain(i..=end_index); // Remove the function definition from lines
        } else {
            i += 1; // Move to the next line only if we didn't find a function definition
        }
    }

    functions
}


// Function to process top-level statements
pub fn process_top_level_statements(code: &str, functions: &HashMap<String, SnarpFunction>) {
    let lines: Vec<&str> = code.lines().collect();
    let mut inside_function = false;

    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        // Skip function definitions
        if trimmed_line.starts_with("func ") && trimmed_line.contains("{") {
            inside_function = true;
        }
        if inside_function {
            if trimmed_line.contains("}") {
                inside_function = false;
            }
            continue;
        }

        // Process top-level statements
        if let Some(output) = crate::functions::parse_print(trimmed_line) {
            println!("{}", output);
        } else if let Some(func_name) = parse_function_call(trimmed_line) {
            if let Some(func) = functions.get(&func_name) {
                execute_function(func, functions);
            } else {
                eprintln!("Error: Function '{}' not defined.", func_name);
            }
        }
    }
}

// Function to execute a Snarp function
pub fn execute_function(func: &SnarpFunction, functions: &HashMap<String, SnarpFunction>) {
    let body_lines: Vec<&str> = func.body.lines().collect();
    for line in body_lines {
        let trimmed_line = line.trim();
        if let Some(output) = crate::functions::parse_print(trimmed_line) {
            println!("{}", output);
        } else if let Some(func_name) = parse_function_call(trimmed_line) {
            if let Some(func) = functions.get(&func_name) {
                execute_function(func, functions);
            } else {
                eprintln!("Error: Function '{}' not defined.", func_name);
            }
        }
    }
}

// Parse function definitions like "func greet() { ... }"
pub fn parse_function_definition(lines: &[&str], start_index: usize) -> Option<(SnarpFunction, usize)> {
    let line = lines[start_index].trim();
    if line.starts_with("func ") && line.contains("(") && line.contains(")") && line.contains("{") {
        let name_start = "func ".len();
        let name_end = line.find("(").unwrap(); // Find the position of the opening parenthesis
        let name = &line[name_start..name_end].trim();

        // Find the end of the function body
        let mut body = String::new();
        let mut end_index = start_index;
        let mut open_braces = 0;
        for (i, line) in lines[start_index..].iter().enumerate() {
            let trimmed_line = line.trim();
            if trimmed_line.contains("{") {
                open_braces += 1;
            }
            if trimmed_line.contains("}") {
                open_braces -= 1;
            }
            body.push_str(line);
            body.push('\n');
            if open_braces == 0 {
                end_index = start_index + i;
                break;
            }
        }

        return Some((
            SnarpFunction {
                name: name.to_string(),
                body: body.trim().to_string(),
            },
            end_index,
        ));
    }

    None
}

// Parse function calls (like "greet();")
pub fn parse_function_call(line: &str) -> Option<String> {
    if line.ends_with("();") {
        let func_name = &line[..line.len() - 3].trim(); // Remove the "();"
        return Some(func_name.to_string());
    } else if line.ends_with("()") {
        let func_name = &line[..line.len() - 2].trim(); // Remove the "()"
        return Some(func_name.to_string());
    }
    None
}