#[macro_use]
extern crate nom;
extern crate rustyline;

mod repl;
mod parser;
mod eval;
mod env;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    List(Vec<Value>),
    Number(i64),
    Atom(String),
    Bool(bool),
}

fn main() {
  repl::run();
}
