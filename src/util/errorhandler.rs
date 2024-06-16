use colored::*;

pub fn errorhandler(message: &str) {
    eprintln!(
        "{}",
        format!("Error: {} \n ---------------------", message).red()
    );
}
