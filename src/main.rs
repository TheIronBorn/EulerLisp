extern crate lisp;

use std::env;
use lisp::eval::Evaluator;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage:");
        println!("  $prog_name repl");
        println!("  $prog_name run <filename>");
    } else {
        match &args[1][..] {
            "repl" => {
                lisp::repl::run();
            },
            "run" => {
                let filename = args.get(2).expect("No filename provided");
                let mut eval = Evaluator::new();
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
