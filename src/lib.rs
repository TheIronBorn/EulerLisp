extern crate time;
extern crate nom;
extern crate rustyline;

pub mod repl;
pub mod eval;

mod builtin;
mod parser;
mod env;
mod desugar;
mod symbol_table;

use std::fmt;
use std::cmp::Ordering;

use std::boxed::Box;
use std::borrow::Borrow;

use std::rc::Rc;

pub type LispResult = Result<Value, LispErr>;

#[derive(Debug, PartialEq, Eq)]
pub enum LispErr {
    InvalidNumberOfArguments,
    InvalidTypeOfArguments,
    DefinitionAlreadyDefined,
    DefinitionNotFound,
}

impl fmt::Display for LispErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispErr::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            LispErr::InvalidTypeOfArguments => write!(f, "Invalid types of arguments"),
            LispErr::DefinitionAlreadyDefined => write!(f, "Definition is already defined"),
            LispErr::DefinitionNotFound => write!(f, "Definition not found"),
        }
    }
}

#[derive(Clone)]
pub struct LispFn(Rc<Fn(Vec<Value>)->LispResult>);

impl fmt::Debug for LispFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "builtin fn")
    }
}

// TODO: This is a lie, eq is not reflexive
impl Eq for LispFn {}

impl PartialEq for LispFn {
    fn eq(&self, other: &LispFn) -> bool {
        false
    }
}

impl PartialOrd for LispFn {
    fn partial_cmp(&self, other: &LispFn) -> Option<Ordering> {
        None
    }
}

// impl core::cmp::PartialOrd for LispFn {
//     fn partial_cmp(&self, other: &LispFn) -> Option<Ordering> {
//         None
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Promise {
    Delayed(env::EnvRef, Box<Value>),
    Result(Box<Value>),
}

// Undefined is used as a response for methods
// that don't return a value.
// If a return value is undefined,
// it will not be printed in the REPL
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Value {
    Atom(String),
    Bool(bool),
    List(Vec<Value>),
    DottedList(Vec<Value>),
    Number(i64),
    Str(String),
    // TODO: Find a way to use just Value here
    Lambda(env::EnvRef, Vec<String>, Box<Value>),
    Builtin(LispFn),
    Promise(Promise),
    Nil,
    Undefined,
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
            Undefined => write!(f, "undefined"),
            Lambda(_, _, _) => write!(f, "<lambda>"),
            Builtin(_) => write!(f, "<builtin>"),
            Promise(Promise::Delayed(_, _)) => write!(f, "promise(?)"),
            Promise(Promise::Result(ref r)) => write!(f, "promise({})", r),
        }
    }
}
