use ::rustyline::error::ReadlineError;
use ::rustyline::Editor;

use eval::Evaluator;
use ::Datum;

pub fn run() {
    let mut rl = Editor::<()>::new();
    let mut eval = Evaluator::new();

    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
          Ok(line) => {
            rl.add_history_entry(&line);
            match eval.eval_str(&line, 0) {
                Ok(res) => {
                    if res != Datum::Undefined {
                        println!("=> {}", res);
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
