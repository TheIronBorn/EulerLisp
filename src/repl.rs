use ::rustyline::error::ReadlineError;
use ::rustyline::Editor;

use ::LispResult;
use parser;
use eval::Evaluator;
use desugar;

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
            let mut result = parser::parse(&line);
            let desugared = desugar::desugar(&result);
            match eval.eval(&desugared) {
                Ok(res) => println!("=> {}", res),
                Err(msg) => println!("!! {}", msg),
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
