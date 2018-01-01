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
mod preprocess;

use env::EnvRef;

use std::fmt;
use std::cmp::Ordering;
use std::boxed::Box;
use std::collections::BTreeMap;

pub type LispResult = Result<Datum, LispErr>;

#[derive(Debug, PartialEq, Eq)]
pub enum LispErr {
    InvalidNumberOfArguments,
    InvalidTypeOfArguments,
    DefinitionAlreadyDefined,
    DefinitionNotFound,
    IOError,
}

impl fmt::Display for LispErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispErr::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            LispErr::InvalidTypeOfArguments => write!(f, "Invalid types of arguments"),
            LispErr::DefinitionNotFound => write!(f, "Definition not found"),
            LispErr::DefinitionAlreadyDefined => write!(f, "Definition is already defined"),
            LispErr::IOError => write!(f, "IO Error"),
        }
    }
}

type LispFn = fn(Vec<Datum>) -> LispResult;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Promise {
    Delayed(EnvRef, Box<Datum>),
    Result(Box<Datum>),
}

impl PartialOrd for Promise {
    fn partial_cmp(&self, _: &Promise) -> Option<Ordering> {
        None
    }
}

impl Ord for Promise {
    fn cmp(&self, _: &Promise) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LambdaType {
    Var,
    List,
    DottedList,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    Bool(bool),
    Number(i64),
    Character(char),
    Str(String),
    Symbol(String),
    List(Vec<Datum>),
    Vector(Vec<Datum>),
    DottedList(Vec<Datum>, Box<Datum>),
    Lambda(EnvRef, Vec<Symbol>, Box<Expression>, LambdaType),
    Builtin(LispFn),
    Promise(Promise),
    Undefined,
    Nil,
}

// TODO: Fix this
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
            // TODO: remove ambiguity
            Datum::Symbol(ref x) => write!(f, "{}", x),
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Condition(Box<Expression>, Box<Expression>);

pub type Symbol = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    LambdaDef(Vec<Symbol>, Box<Expression>, LambdaType),
    Do(Vec<Expression>, Box<Expression>),
    And(Vec<Expression>, Box<Expression>),
    Or(Vec<Expression>, Box<Expression>),
    Quote(Box<Datum>),
    Conditional(Vec<Condition>, Box<Expression>),
    Case(Box<Expression>, BTreeMap<Datum, Expression>, Box<Expression>),
    Definition(Symbol, Box<Expression>),
    MacroDefinition(Symbol, Box<Expression>),
    Assignment(Symbol, Box<Expression>),
    VectorPush(Symbol, Box<Expression>),
    VectorSet(Symbol, Box<Expression>, Box<Expression>),
    DirectFunctionCall(Symbol, Vec<Expression>),
    SpecialFunctionCall(String, Vec<Expression>),
    SymbolFunctionCall(Symbol, Vec<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    Symbol(Symbol),
    Bool(bool),
    Number(i64),
    Character(char),
    Str(String),
    List(Vec<Datum>),
    DottedList(Vec<Datum>, Box<Datum>),
    Vector(Vec<Datum>),
    Lambda(EnvRef, Vec<Symbol>, Box<Expression>, LambdaType),
    Builtin(LispFn),
    Promise(Promise),
    Undefined,
    Nil,
}

impl PartialOrd for Expression {
    fn partial_cmp(&self, _: &Expression) -> Option<Ordering> {
        None
    }
}

impl Ord for Expression {
    fn cmp(&self, _: &Expression) -> Ordering {
        Ordering::Equal
    }
}
