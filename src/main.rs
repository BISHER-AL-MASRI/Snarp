use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

mod components;

use components::exit::exit_func;
use components::print::print_func;
use components::r#return::return_func;
use components::r#let::let_func;

mod util;
use util::args;
use util::errorhandler::errorhandler;

fn get_file() -> String {
    let args = args::args();
    return args[1].clone();
}

fn main() -> io::Result<()> {
    let args = args::args();
    if args.len() <= 1 {
        println!("Snarp Version 0.1.0 \n Shell Mode");
        shell_exec()?;
    } else {
        let _ = file_exec();
    }
    Ok(())
}

fn file_exec() -> io::Result<()> {
    let file = get_file();
    
    let fileexten = file.split('.').last().unwrap_or("");
    if fileexten != "sp" {
        errorhandler(&format!("Unknown file extension: {}", fileexten));
        return Ok(());
    }

    let contents = std::fs::read_to_string(&file)?;
    let lines = contents.lines();

    let table: Arc<
        Mutex<HashMap<&str, Box<dyn Fn(&mut HashMap<String, String>, Vec<&str>) + Send>>>,
    > = Arc::new(Mutex::new(HashMap::new()));
    let vars: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut table = table.lock().unwrap();
        table.insert("print", Box::new(print_func));
        table.insert("return", Box::new(return_func));
        table.insert("let", Box::new(let_func));
        table.insert("exit", Box::new(exit_func));
    }

    for line in lines {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        if let Some(f) = table.lock().unwrap().get(words[0]) {
            let args = words[1..].to_vec();
            f(&mut vars.lock().unwrap(), args);
        } else {
            errorhandler(&format!("Unknown command: {}", line));
        }
    }

    Ok(())
}

fn shell_exec() -> io::Result<()> {
    let table: Arc<
        Mutex<HashMap<&str, Box<dyn Fn(&mut HashMap<String, String>, Vec<&str>) + Send>>>,
    > = Arc::new(Mutex::new(HashMap::new()));
    let vars: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    
    {
        let mut table = table.lock().unwrap();
        table.insert("print", Box::new(print_func));
        table.insert("return", Box::new(return_func));
        table.insert("var", Box::new(let_func));
        table.insert("exit", Box::new(exit_func));
    }

    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)?;
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        if let Some(f) = table.lock().unwrap().get(words[0]) {
            let args = words[1..].to_vec();
            f(&mut vars.lock().unwrap(), args);
        } else {
            errorhandler(&format!("Unknown command: {}", line));
        }
    }
}