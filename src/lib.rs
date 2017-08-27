#[macro_use]
extern crate nom;
extern crate rustyline;

pub mod repl;
pub mod eval;
mod parser;
mod env;
mod desugar;

use std::fmt;

use env::Environment;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Atom(String),
    Bool(bool),
    List(Vec<Value>),
    DottedList(Vec<Value>),
    Number(i64),
    Str(String),
    // TODO: Find a way to use just Value here
    Lambda(Environment, Vec<String>, Box<Value>),
    Nil,
}

impl Value {
    pub fn is_pair(&self) -> bool {
        match *self {
          DottedList(_) => true,
          List(_) => true,
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
          List(_) => true,
          _ => false,
        }
    }
}

use Value::*;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Atom(ref x) => write!(f, "{}", x),
            Bool(x) => write!(f, "{}", x),
            List(ref elems) => {
                let mut result = String::new();
                result.push_str("(");
                for (i, e) in elems.iter().enumerate() {
                    if i != 0 {
                        result.push_str(" ");
                    }
                    result.push_str(&e.to_string());
                }
                result.push_str(")");
                write!(f, "{}", result)
            },
            DottedList(ref elems) => {
                let mut result = String::new();
                result.push_str("(");
                for (i, e) in elems.iter().enumerate() {
                    if i != 0 {
                        if i < (elems.len() - 1) {
                            result.push_str(" ");
                        } else {
                            result.push_str(" . ");
                        }
                    }
                    result.push_str(&e.to_string());
                }
                result.push_str(")");
                write!(f, "{}", result)
            },
            Number(x) => write!(f, "{}", x),
            Str(ref s) => write!(f, "\"{}\"", s),
            Nil => write!(f, "'()"),
            Lambda(_, _, _) => write!(f, "<lambda>"),
        }
    }
}
