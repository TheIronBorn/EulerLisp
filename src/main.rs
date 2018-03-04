extern crate colored;
extern crate indicatif;
extern crate lisp;
extern crate csv;

use std::env;
use std::io;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use std::collections::HashMap;

use indicatif::ProgressBar;
use colored::*;

use lisp::eval::Evaluator;
use lisp::{repl, doc};

fn find_file_for_problem(problem: isize, include_all: bool) -> Option<PathBuf> {
    let mut paths = Vec::new();

    let subfolder = (problem - 1) / 50;
    let solved_path = format!(
        "./project_euler/{:03}-{:03}/{:02}.scm",
        subfolder * 50 + 1,
        subfolder * 50 + 50,
        problem
    );
    paths.push(Path::new(&solved_path).to_path_buf());

    if include_all {
        let wip_path = format!("./project_euler/wip/{:02}.scm", problem);
        let slow_path = format!("./project_euler/slow/{:02}.scm", problem);
        paths.push(Path::new(&wip_path).to_path_buf());
        paths.push(Path::new(&slow_path).to_path_buf());
    }

    for path in paths.into_iter() {
        if path.exists() {
            return Some(path)
        }
    }

    None
}

fn format_duration(duration : Duration) -> String {
    let millis = duration.subsec_nanos() / 1000000;
    format!("{}.{}s", duration.as_secs(), millis).to_string()
}

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
            v @ "run" | v @ "doc" => {
                let mut filename = args.get(0).expect("No filename provided").clone();

                if !filename.ends_with(".scm") {
                    let problem = filename.parse::<isize>().unwrap();
                    if let Some(problem_path) = find_file_for_problem(problem, true) {
                        filename = problem_path.to_str().unwrap().to_string();
                    } else {
                        panic!(format!("Could not find file for problem {}", problem));
                    }
                }
                
                if v == "run" {
                    let mut eval = Evaluator::new(
                        Rc::new(RefCell::new(io::stdout())),
                        use_stdlib
                    );
                    match eval.eval_file(&filename) {
                        Err(e) => println!("Error: {}", e),
                        _ => (),
                    }
                } else {
                    doc::process_file(&filename);
                }
            },
            "test" => {
                let mut solutions : HashMap<isize, String> = HashMap::new();

                let solutions_file = File::open("./project_euler/solutions.csv").unwrap();
                let mut rdr = csv::Reader::from_reader(solutions_file);

                for result in rdr.records() {
                    let record = result.unwrap();
                    if record.len() != 2 {
                        panic!(format!("Invalid solution entry: {:?}", record));
                    }

                    let problem = record.get(0).unwrap().parse::<isize>().unwrap();
                    let solution = record.get(1).unwrap().to_string();

                    if solution != "" {
                        solutions.insert(problem, solution);
                    }
                }

                let mut full = Duration::new(0, 0);

                let from = args.get(0).unwrap().parse::<isize>().unwrap();
                let to = args.get(1).unwrap().parse::<isize>().unwrap();

                let mut missing = Vec::new();
                let mut correct = Vec::new();
                let mut wrong = Vec::new();

                let pb = ProgressBar::new((to - from + 1) as u64);

                for problem in from..(to+1) {
                    if let Some(path) = find_file_for_problem(problem, false) {
                        let now = Instant::now();
                        let mut output = Rc::new(RefCell::new(Vec::new()));
                        let mut eval = Evaluator::new(
                            output.clone(),
                            true
                        );

                        let s = path.to_str().unwrap();

                        // TODO: Collect errors
                        eval.eval_file(s).expect("Failed to evaluate file");

                        // TODO: Mark slow problems
                        let duration = now.elapsed();
                        full += duration;

                        let solution = String::from_utf8(output.borrow().clone()).unwrap();
                        let got = solution.trim_left_matches("Solution: ").trim().to_string();

                        match solutions.get(&problem) {
                            Some(expected) => {
                                if expected == &got {
                                    correct.push((problem, duration));
                                } else {
                                    wrong.push((problem, got, expected.clone()));
                                }
                            },
                            None => {
                                panic!(format!("No reference solution for {}", problem));
                            }
                        }
                    } else {
                        missing.push(problem);
                    }
                    pb.inc(1);
                }
                pb.finish_with_message("done");
                println!("");

                if correct.len() > 0 {
                    println!("{}", "Correct".green().bold());
                    for (problem, duration) in correct {
                        let time = format_duration(duration);
                        println!(" {} {}", problem.to_string().green(), time);
                    }
                    println!("");
                }

                if missing.len() > 0 {
                    println!("{}", "Missing".yellow().bold());
                    for problem in missing {
                        println!("{}", problem.to_string().yellow());
                    }
                    println!("");
                }

                if wrong.len() > 0 {
                    println!("{}", "Wrong".red().bold());
                    for (problem, got, expected) in wrong {
                        println!(" {}", problem.to_string().red());
                        println!("   Expected: {}", expected.green());
                        println!("   Got:      {}", got.red());
                    }
                    println!("");
                }

                let millis = full.subsec_nanos() / 1000000;
                println!("Time: {}.{}s", full.as_secs(), millis);
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
