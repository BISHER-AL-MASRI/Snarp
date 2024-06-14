use colored::*;
use std::collections::HashMap;
use std::io;
use std::sync::{Arc, Mutex};

mod components;
mod util;

use components::add::add_func;
use components::exit::exit_func;
use components::print::print_func;
use components::r#return::return_func;
use components::var::var_func;
use util::args;

fn get_file() -> String {
    let args = args::main();
    return args[1].clone();
}

fn main() -> io::Result<()> {
    let args = args::main();
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
    let contents = std::fs::read_to_string(file)?;

    let mut lines = contents.lines();

    let table: Arc<
        Mutex<HashMap<&str, Box<dyn Fn(&mut HashMap<String, String>, Vec<&str>) + Send>>>,
    > = Arc::new(Mutex::new(HashMap::new()));
    let funcs: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let vars: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    {
        let mut table = table.lock().unwrap();
        table.insert("print", Box::new(print_func));
        table.insert("return", Box::new(return_func));
        table.insert("var", Box::new(var_func));
        table.insert("add", Box::new(add_func));
        table.insert("exit", Box::new(exit_func));
    }

    Ok(while let Some(line) = lines.next() {
        let line = line.trim();

        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        if words[0] == "Func" {
            let func_name = words[1].trim_end_matches("()").to_string();
            let mut func_body = Vec::new();
            
            while let Some(func_line) = lines.next() {
                let func_line = func_line.trim();
                if func_line == ")" {
                    break;
                }
                if func_line.is_empty() || func_line.starts_with("//") {
                    continue;
                }
                func_body.push(func_line.to_string());
            }

            funcs.lock().unwrap().insert(func_name, func_body);
        } else if let Some(f) = table.lock().unwrap().get(words[0]) {
            let args = words[1..].to_vec();
            f(&mut vars.lock().unwrap(), args);
        } else if funcs.lock().unwrap().contains_key(words[0]) {
            let func_body = funcs.lock().unwrap().get(words[0]).unwrap().clone();
            for func_line in func_body {
                let func_words: Vec<&str> = func_line.split_whitespace().collect();
                if let Some(f) = table.lock().unwrap().get(func_words[0]) {
                    let args = func_words[1..].to_vec();
                    f(&mut vars.lock().unwrap(), args);
                } else {
                    eprintln!("{}", format!("Error: Unknown command in function: {}", func_line).red());
                }
            }
        } else {
            eprintln!("{}", format!("Error: Unknown command: {}", line).red());
        }
    })
}

fn shell_exec() -> io::Result<()> {
    let table: Arc<
        Mutex<HashMap<&str, Box<dyn Fn(&mut HashMap<String, String>, Vec<&str>) + Send>>>,
    > = Arc::new(Mutex::new(HashMap::new()));
    let funcs: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let vars: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut table = table.lock().unwrap();
        table.insert("print", Box::new(print_func));
        table.insert("return", Box::new(return_func));
        table.insert("var", Box::new(var_func));
        table.insert("add", Box::new(add_func));
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

        if words[0] == "Func" {
            let func_name = words[1].trim_end_matches("()").to_string();
            let mut func_body = Vec::new();
            println!("Enter function body (end with ')'):");

            loop {
                let mut func_line = String::new();
                io::stdin().read_line(&mut func_line)?;
                let func_line = func_line.trim().to_string();

                if func_line == ")" {
                    break;
                }
                if func_line.is_empty() || func_line.starts_with("//") {
                    continue;
                }

                func_body.push(func_line);
            }

            funcs.lock().unwrap().insert(func_name, func_body);
        } else if let Some(f) = table.lock().unwrap().get(words[0]) {
            let args = words[1..].to_vec();
            f(&mut vars.lock().unwrap(), args);
        } else if funcs.lock().unwrap().contains_key(words[0]) {
            let func_body = funcs.lock().unwrap().get(words[0]).unwrap().clone();
            for func_line in func_body {
                let func_words: Vec<&str> = func_line.split_whitespace().collect();
                if let Some(f) = table.lock().unwrap().get(func_words[0]) {
                    let args = func_words[1..].to_vec();
                    f(&mut vars.lock().unwrap(), args);
                } else {
                    eprintln!("{}", format!("Error: Unknown command in function: {}", func_line).red());
                }
            }
        } else {
            eprintln!("{}", format!("Error: Unknown command: {}", line).red());
        }
    }
}