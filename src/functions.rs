use std::collections::HashMap;
use gio::prelude::{ApplicationExt, ApplicationExtManual};
use gtk4::{prelude::GtkWindowExt, Application, ApplicationWindow};

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
        functions.insert(
            "makeWindow".to_string(),
            SnarpFunction {
                name: "makeWindow".to_string(),
                body: String::from("makeWindow"),
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


pub fn parse_make_window(line: &str, variables: &mut HashMap<String, Window>) -> Option<()> {
    if let Some((var_name, call)) = line.split_once('=') {
        let var_name = var_name.trim();
        let call = call.trim();

        if call.starts_with("createWindow(") && call.ends_with(");") {
            let params = &call["createWindow(".len()..call.len() - 2]; // Extract parameters
            let args: Vec<&str> = params.split(',').map(|s| s.trim()).collect();

            if args.len() == 3 {
                let title = args[0].trim_matches('"'); // Remove quotes
                let width = args[1].parse::<i32>().unwrap();
                let height = args[2].parse::<i32>().unwrap();
                let window = create_window(title, width, height);
                variables.insert(var_name.to_string(), window);
                return Some(());
            } else {
                eprintln!("Error: Expected 3 arguments, found {}", args.len());
            }
        }
    }

    None
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Window {
    title: String,
    width: i32,
    height: i32,
}


pub fn create_window(title: &str, width: i32, height: i32) -> Window {
    gtk4::init().unwrap();

    let app = Application::builder().application_id("test").build();

    let title = title.to_string(); 
    let title_for_closure = title.clone(); 

    app.connect_startup(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title(&title_for_closure)
            .default_width(width)
            .default_height(height)
            .build();
        window.present();
    });

    app.run();

    Window {
        title, 
        width,
        height,
    }
}
