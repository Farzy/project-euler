use std::env;
use std::collections::HashMap;
use project_euler::helper;

mod euler_1;

// Simplify long hashmap type
type FunctionHash = HashMap<String, (String, fn())>;

fn main() {
    let mut functions: FunctionHash = HashMap::new();
    functions.insert(String::from("1"), (String::from("Euler 1"), euler_1::main));

    if env::args().len() == 1 { // No arguments
        eprintln!("Error: Missing argument");
        usage(&functions);
        return;
    } else {
        for k in env::args().skip(1) {
            if k == "-h" || k == "--help" {
                usage(&functions);
                return;
            } else if functions.contains_key(&k) {
                let (description, func) = functions.get(&k).unwrap();
                helper::section(description);
                func();
            } else {
                eprintln!("ERROR: Problem '{}' not found", k);
            }
        }
    }

}

fn usage(functions: &FunctionHash) {
    eprintln!(r#"
Usage: PROGNAME [options] [number]

 -h: Print this help

Specify the number of the Euler problem to execute

List of problems:"#);
    for (name, (description, _)) in functions {
        eprintln!(" - {}: {}", name, description);
    }
}
