use std::env;
use std::collections::HashMap;
use project_euler::*;

// Simplify long hashmap type
type FunctionHash = HashMap<String, (String, fn() -> String)>;

fn main() {
    let mut functions: FunctionHash = HashMap::new();

    macro_rules! module {
        ($mod_name:ident, $desc:expr) => (
            functions.insert(String::from(stringify!($mod_name)),
                             (String::from($desc), $mod_name::main));
        );
    }

    module!(euler_1, "Euler 1: Multiples of 3 and 5");
    module!(euler_2, "Euler 2: Even Fibonacci numbers");

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
                println!("{}", func());
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
