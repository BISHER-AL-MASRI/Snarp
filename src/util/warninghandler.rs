use colored::*;

pub fn warninghandler(message: &str) {
    eprintln!(
        "{}",
        format!("Error: {} \n ---------------------", message).yellow()
    );
}