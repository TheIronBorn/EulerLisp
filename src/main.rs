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
                let main_env = eval.make_root_env();
                eval.eval_file(filename, main_env);
            },
            _ => {
                println!("Unknown command");
            }
        }
    }
}
