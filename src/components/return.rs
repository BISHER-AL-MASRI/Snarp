use std::collections::HashMap;

use crate::util::warninghandler::warninghandler;

pub fn return_func(vars: &mut HashMap<String, String>, args: Vec<&str>) {
    if args.is_empty() {
        warninghandler(&format!(
            "Invalid number of arguments for return: {}, returning with code 0 instead",
            args.len()
        ));
        return;
    }

    let arg = args[0];
    let num = if let Ok(num) = arg.parse::<i32>() {
        num
    } else if let Some(value) = vars.get(arg) {
        value.parse::<i32>().unwrap_or_else(|_| {
            warninghandler(&format!(
                "Invalid number: {}, returning with code 1 instead",
                value
            ));
            std::process::exit(1);
        })
    } else {
        warninghandler(&format!(
            "Invalid number: {}, returning with code 1 instead",
            arg
        ));
        std::process::exit(1);
    };

    println!("Returning with code: {}", num);
    std::process::exit(num);
}
