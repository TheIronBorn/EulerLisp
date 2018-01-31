extern crate lisp;
extern crate glob;

use std::env;
use lisp::eval::Evaluator;
use glob::glob;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage:");
        println!("  $prog_name repl");
        println!("  $prog_name run <filename>");
        println!("");
        println!("Flags:");
        println!("  --no-stdlib, don't load the stdlib on startup");
    } else {
        args.remove(0); // skip program name
        let command = args.remove(0);
        let use_stdlib = !args.iter().any(|x| *x == String::from("--no-stdlib"));
        match &command[..] {
            "repl" => {
                lisp::repl::run(use_stdlib);
            },
            "run" => {
                let filename = args.get(0).expect("No filename provided");
                let mut eval = Evaluator::new(use_stdlib);
                match eval.eval_file(filename) {
                    Err(e) => println!("Error: {}", e),
                    _ => (),
                }
            },
            "test" => {

                for path in glob("./project_euler/*-*/*.scm").expect("Failed to read glob pattern") {
                    let path = path.expect("Failed to read path").display().to_string();
                    println!("Testing {}", path);

                    let mut eval = Evaluator::new(true);
                    eval.eval_file(&path);
                }
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
