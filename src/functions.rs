use std::{collections::HashMap, thread};

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


/// Parse `makeWindow` calls
pub fn parse_make_window(line: &str) -> Option<()> {
    if line.starts_with("makeWindow(") && line.ends_with(");") {
        let params = &line["makeWindow(".len()..line.len() - 2]; // Extract parameters
        let args: Vec<&str> = params.split(',').map(|s| s.trim()).collect();

        if args.len() == 2 {
            let title = args[0].trim_matches('"'); // Remove quotes
            let platform = args[1].trim_matches('"');
            create_window(title, platform);
            return Some(());
        }
    }
    None
}

/// Create a window
pub fn create_window(title: &str, platform: &str) {
    let title = title.to_string();
    let platform = platform.to_string();

    if platform == "macos" || platform == "macOS" {
        println!("Creating macOS window with title '{}'", title);

        use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicyRegular, NSWindow, NSWindowStyleMask};
        use cocoa::base::nil;
        use cocoa::foundation::{NSAutoreleasePool, NSRect, NSPoint, NSSize, NSString};

        unsafe {
            let _pool = NSAutoreleasePool::new(nil);
            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
                NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(400.0, 300.0)),
                NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask,
                cocoa::appkit::NSBackingStoreBuffered,
                false,
            );
            let ns_title = NSString::alloc(nil).init_str(&title);
            window.setTitle_(ns_title);
            window.makeKeyAndOrderFront_(nil);

            app.run(); // Run the app on the main thread
        }
    } else {
        eprintln!("Unsupported platform: {}", platform);
    }
}
