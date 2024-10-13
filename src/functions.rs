use std::collections::HashMap;

#[derive(Debug)]
pub struct SnarpFunction {
    pub name: String,
    pub body: String,
}

pub fn built_in_functions() -> HashMap<String, SnarpFunction> {
    let mut functions = HashMap::new();
    functions.insert(
        "print".to_string(),
        SnarpFunction {
            name: "print".to_string(),
            body: String::from("print"), 
        },
    );
    functions
}
pub fn parse_print(line: &str) -> Option<String> {
    let trimmed_line = line.trim(); 

    
    if (trimmed_line.starts_with("print(") && trimmed_line.ends_with(")")) ||
       (trimmed_line.starts_with("print(") && trimmed_line.ends_with(");")) {
        
        let start_index = "print(".len();
        let end_index = if trimmed_line.ends_with(");") {
            trimmed_line.len() - 2 
        } else {
            trimmed_line.len() - 1 
        };
        let inner_content = &trimmed_line[start_index..end_index];

        
        if inner_content.starts_with('"') && inner_content.ends_with('"') {
            return Some(inner_content[1..inner_content.len() - 1].to_string());
        }
    }

    None
}