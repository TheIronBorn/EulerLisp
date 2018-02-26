extern crate lisp;

use std::env;
use std::io;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};

use lisp::eval::Evaluator;
use lisp::{repl, doc};

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
                repl::run(use_stdlib);
            },
            "run" => {
                let filename = args.get(0).expect("No filename provided");
                let mut eval = Evaluator::new(
                    Rc::new(RefCell::new(io::stdout())),
                    use_stdlib
                );
                match eval.eval_file(filename) {
                    Err(e) => println!("Error: {}", e),
                    _ => (),
                }
            },
            "doc" => {
                let filename = args.get(0).expect("No filename provided");
                doc::process_file(filename);
            }
            "test" => {
                let mut full = Duration::new(0, 0);

                let from = args.get(0).unwrap().parse::<i64>().unwrap();
                let to = args.get(1).unwrap().parse::<i64>().unwrap();

                for problem in from..(to+1) {
                    let subfolder = (problem - 1) / 50;
                    let path = format!(
                        "./project_euler/{:03}-{:03}/{:02}.scm",
                        subfolder * 50 + 1,
                        subfolder * 50 + 50,
                        problem
                    );

                    println!("Testing {}", path);

                    let now = Instant::now();
                    let mut eval = Evaluator::new(
                        Rc::new(RefCell::new(io::stdout())),
                        true
                    );
                    eval.eval_file(&path).expect("Failed to evaluate file");

                    let duration = now.elapsed();
                    full += duration;
                    let millis = duration.subsec_nanos() / 1000000;
                    println!("Time: {}.{}s", duration.as_secs(), millis);
                    println!("");
                }

                let millis = full.subsec_nanos() / 1000000;
                println!("Overall Time: {}.{}s", full.as_secs(), millis);
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
