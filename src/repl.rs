use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use ::rustyline::error::ReadlineError;
use ::rustyline::Editor;

use eval::Evaluator;
use ::Datum;

pub fn run(stdlib: bool) {
    let mut rl = Editor::<()>::new();
    let mut eval = Evaluator::new(
        Rc::new(RefCell::new(io::stdout())),
        stdlib
    );

    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
          Ok(line) => {
            rl.add_history_entry(&line);

            match eval.eval_str(&line) {
                Ok(res) => {
                    if res != Datum::Undefined {
                        println!("=> {}", res.to_string(&mut eval.symbol_table));
                    }
                },
                Err(msg) => println!("!! {}", msg)
            };
          },
          Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            break
          },
          Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            break
          }
          Err(err) => {
            println!("Error: {:?}", err);
            break
          }
        }
    }

    rl.save_history("history.txt").unwrap();
}
