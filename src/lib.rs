extern crate time;
extern crate nom;
extern crate rustyline;
extern crate rand;

#[macro_use]
mod macros;

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
use std::rc::Rc;

pub type LispResult = Result<Datum, LispErr>;

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
pub struct LispFn(Rc<Fn(Vec<Datum>)->LispResult>);

impl fmt::Debug for LispFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "builtin fn")
    }
}

// TODO: This is a lie, eq is not reflexive
impl Eq for LispFn {}

impl PartialEq for LispFn {
    fn eq(&self, _: &LispFn) -> bool {
        false
    }
}

impl PartialOrd for LispFn {
    fn partial_cmp(&self, _: &LispFn) -> Option<Ordering> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Promise {
    Delayed(env::EnvRef, Box<Datum>),
    Result(Box<Datum>),
}

// Undefined is used as a response for methods
// that don't return a value.
// If a return value is undefined,
// it will not be printed in the REPL
// #[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
// pub enum Datum {
//     Symbol(String),
//     Bool(bool),
//     List(Vec<Datum>),
//     DottedList(Vec<Datum>),
//     Number(i64),
//     Str(String),
//     Lambda(env::EnvRef, Vec<String>, Box<Datum>),
//     Builtin(LispFn),
//     Promise(Promise),
//     Nil,
//     Undefined,
// }

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum LambdaType {
    Var,
    List,
    DottedList,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Datum {
    Bool(bool),
    Number(i64),
    Character(char),
    Str(String),
    Symbol(String),
    List(Vec<Datum>),
    DottedList(Vec<Datum>, Box<Datum>),
    Vector(Vec<Datum>),
    Lambda(env::EnvRef, Vec<String>, Box<Datum>, LambdaType),
    Builtin(LispFn),
    Promise(Promise),
    Undefined,
    Nil, // TODO: Remove this in favor of empty lists
}

impl Datum {
    pub fn is_pair(&self) -> bool {
        match *self {
          DottedList(_, _) => true,
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

use Datum::*;

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbol(ref x) => write!(f, "{}", x),
            Bool(x) => {
                if x {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            },
            Character(c) => write!(f, "#\\{}", c),
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
            DottedList(ref elems, ref tail) => {
                let mut result = String::new();
                result.push_str("(");
                for e in elems.iter() {
                    result.push_str(&e.to_string());
                    result.push_str(" ");
                }
                result.push_str(". ");
                result.push_str(&tail.to_string());
                result.push_str(")");
                write!(f, "{}", result)
            },
            Vector(ref elems) => {
                let mut result = String::new();
                result.push_str("#(");
                for (i, e) in elems.iter().enumerate() {
                    if i != 0 {
                        result.push_str(" ");
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
            Lambda(_, _, _, _) => write!(f, "<lambda>"),
            Builtin(_) => write!(f, "<builtin>"),
            Promise(Promise::Delayed(_, _)) => write!(f, "promise(?)"),
            Promise(Promise::Result(ref r)) => write!(f, "promise({})", r),
        }
    }
}
