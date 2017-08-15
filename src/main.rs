#[macro_use]
extern crate nom;
extern crate rustyline;

mod repl;
mod parser;
mod eval;
mod env;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Value {
    List(Vec<Value>),
    Number(i64),
    Atom(String),
    Bool(bool),
}

use Value::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Number(x) => write!(f, "{}", x),
            Bool(x) => write!(f, "{}", x),
            List(ref x) => write!(f, "{:#?}", x),
            Atom(ref x) => write!(f, "'{}", x),
        }
    }
}

fn main() {
  repl::run();
}
