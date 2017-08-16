#[macro_use]
extern crate nom;
extern crate rustyline;

mod repl;
mod parser;
mod eval;
mod env;

use std::fmt;

use env::Environment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(String),
    Bool(bool),
    List(Vec<Value>),
    Number(i64),
    Pair(Box<Value>, Box<Value>),
    Str(String),
    // TODO: Find a way to use just Value here
    Lambda(Environment, Vec<String>, Box<Value>),
    Nil,
}

impl Value {
    pub fn is_pair(&self) -> bool {
        match *self {
          Pair(_, _) => true,
          _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match *self {
          Nil => true,
          _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match *self {
          Pair(_, ref rst) => rst.is_list_end(),
          _ => false,
        }
    }

    fn is_list_end(&self) -> bool {
        self.is_nil() || self.is_list()
    }
}

use Value::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom(ref x) => write!(f, "'{}", x),
            Bool(x) => write!(f, "{}", x),
            List(ref x) => write!(f, "{:#?}", x),
            Number(x) => write!(f, "{}", x),
            Pair(ref a, ref b) => write!(f, "({} . {})", a, b),
            Str(ref s) => write!(f, "\"{}\"", s),
            Nil => write!(f, "'()"),
            Lambda(_, _, _) => write!(f, "<lambda>"),
        }
    }
}

fn main() {
  repl::run();
}
