use std::collections::HashMap;

#[derive(Debug)]
pub struct SnarpFunction {
    pub name: String,
    pub body: String,
}

// Dummy implementation of built-in functions
pub fn built_in_functions() -> HashMap<String, SnarpFunction> {
    let mut functions = HashMap::new();
    functions.insert(
        "print".to_string(),
        SnarpFunction {
            name: "print".to_string(),
            body: String::from("print"), // Placeholder for print logic
        },
    );
    functions
}
pub fn parse_print(line: &str) -> Option<String> {
    let trimmed_line = line.trim(); // Trim the line to remove leading and trailing whitespace

    // Check if the line starts with "print(" and ends with ")" or ");"
    if (trimmed_line.starts_with("print(") && trimmed_line.ends_with(")")) ||
       (trimmed_line.starts_with("print(") && trimmed_line.ends_with(");")) {
        // Extract the string inside the parentheses
        let start_index = "print(".len();
        let end_index = if trimmed_line.ends_with(");") {
            trimmed_line.len() - 2 // Exclude the closing parenthesis and semicolon
        } else {
            trimmed_line.len() - 1 // Exclude the closing parenthesis
        };
        let inner_content = &trimmed_line[start_index..end_index];

        // Remove surrounding quotes if present
        if inner_content.starts_with('"') && inner_content.ends_with('"') {
            return Some(inner_content[1..inner_content.len() - 1].to_string());
        }
    }

    None
}