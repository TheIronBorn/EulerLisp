use ::rustyline::error::ReadlineError;
use ::rustyline::Editor;

use parser;
use eval;

pub fn run() {
    let mut rl = Editor::<()>::new();
    if let Err(_) = rl.load_history("history.txt") {
      println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
          Ok(line) => {
            rl.add_history_entry(&line);
            let mut result = parser::parse(&line);
            result = eval::eval(result);
            println!("{:?}", result);
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
