use colored::*;

pub fn warninghandler(message: &str) {
    eprintln!(
        "{}",
        format!("Warning: {} \n ---------------------", message).yellow()
    );
}
