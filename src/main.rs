extern crate lisp;

use std::env;
use lisp::eval::Evaluator;

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
                let filename = args.get(2).expect("No filename provided");
                let mut eval = Evaluator::new(use_stdlib);
                match eval.eval_file(filename) {
                    Err(e) => println!("Error: {}", e),
                    _ => (),
                }
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
}
