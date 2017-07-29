#[macro_use]
extern crate nom;
extern crate rustyline;

mod repl;
mod parser;
mod eval;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    List(Vec<Value>),
    Number(i64),
    Atom(String),
}

fn main() {
  repl::run();
}
