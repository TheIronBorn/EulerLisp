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
mod bignum;

use env::EnvRef;

use std::fmt;
use std::cmp::Ordering;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::mem;

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

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Arity {
    Exact(usize),
    Range(usize, usize),
    Min(usize)
}

impl Arity {
    fn check(&self, provided: usize) {
        match *self {
            Arity::Exact(a) => {
                if a != provided {
                    panic!("Expected {} arguments, got {}", a, provided);
                }
            },
            Arity::Min(a) => {
                if a > provided {
                    panic!("Expected at least {} arguments, got {}", a, provided);
                }
            },
            Arity::Range(a, b) => {
                if provided < a || provided > b {
                    panic!("Expected between {} and {} arguments, got {}", a, b, provided);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct LispFn(fn(&mut [Datum]) -> LispResult, Arity);

impl fmt::Debug for LispFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LispFn with arity {:?}", self.1)
    }
}

impl PartialOrd for LispFn {
    fn partial_cmp(&self, _: &LispFn) -> Option<Ordering> {
        None
    }
}

impl Ord for LispFn {
    fn cmp(&self, _: &LispFn) -> Ordering {
        Ordering::Equal
    }
}

impl PartialEq for LispFn {
    fn eq(&self, _other: &LispFn) -> bool {
        false
    }
}
impl Eq for LispFn {}

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
    List,
    DottedList,
}

#[derive(Clone)]
pub struct Lambda {
    env: EnvRef,
    params: Vec<Symbol>,
    defaults: Vec<Datum>,
    body: Box<Expression>,
    kind: LambdaType
}

// TODO: Implement comparison on datums by hand,
// these traits are mostly wrong
impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lambda({:?})", self.params)
    }
}
impl PartialEq for Lambda {
    fn eq(&self, _other: &Lambda) -> bool {
        false
    }
}
impl Eq for Lambda {}

impl PartialOrd for Lambda {
    fn partial_cmp(&self, _: &Lambda) -> Option<Ordering> {
        None
    }
}

impl Ord for Lambda {
    fn cmp(&self, _: &Lambda) -> Ordering {
        Ordering::Equal
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Datum {
    Bool(bool),
    Number(isize),
    Character(char),
    Str(String),
    Symbol(String),
    List(Vec<Datum>),
    DottedList(Vec<Datum>, Box<Datum>),
    Lambda(Lambda),
    Builtin(LispFn),
    Promise(Promise),
    Undefined,
    Nil,
    Bignum(bignum::Bignum)
}

// TODO: Fix this
impl Datum {
    pub fn is_pair(&self) -> bool {
        match *self {
          Datum::DottedList(_, _) => true,
          Datum::List(_) => true,
          _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match *self {
          Datum::Nil => true,
          _ => false,
        }
    }

    pub fn is_list(&self) -> bool {
        match *self {
          Datum::List(_) => true,
          _ => false,
        }
    }

    pub fn take(&mut self) -> Datum {
        mem::replace(self, Datum::Undefined)
    }

    pub fn replace(&mut self, other: Datum) {
        mem::replace(self, other);
    }

    pub fn push(&mut self, value: Datum) {
        if let Datum::List(ref mut elems) = *self {
            elems.push(value);
        } else if self.is_nil() {
            self.replace(Datum::List(vec![value]));
        } else {
            panic!("push! only works on lists and '()");
        }
    }
}

impl fmt::Display for Datum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Datum::Symbol(ref x) => write!(f, "{}", x),
            Datum::Bool(x) => {
                if x {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            },
            Datum::Character(c) => write!(f, "#\\{}", c),
            Datum::List(ref elems) => {
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
            Datum::DottedList(ref elems, ref tail) => {
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
            Datum::Number(x) => write!(f, "{}", x),
            Datum::Bignum(ref x) => write!(f, "{}", x),
            Datum::Str(ref s) => write!(f, "\"{}\"", s),
            Datum::Nil => write!(f, "'()"),
            Datum::Undefined => write!(f, "undefined"),
            Datum::Lambda(_) => write!(f, "<lambda>"),
            Datum::Builtin(_) => write!(f, "<builtin>"),
            Datum::Promise(Promise::Delayed(_, _)) => write!(f, "promise(?)"),
            Datum::Promise(Promise::Result(ref r)) => write!(f, "promise({})", r),
        }
    }
}

pub type Symbol = usize;

#[derive(Clone)]
pub enum Expression {
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    LambdaDef(Vec<Symbol>, Vec<Datum>, Box<Expression>, LambdaType),
    Do(Vec<Expression>, Box<Expression>),
    Quote(Box<Datum>),
    Case(Box<Expression>, BTreeMap<Datum, Expression>, Box<Expression>),
    Definition(Symbol, Box<Expression>),
    MacroDefinition(Symbol, Box<Expression>),
    Assignment(Symbol, Box<Expression>),
    ListPush(Symbol, Box<Expression>),
    ListRef(Symbol, Box<Expression>),
    ListSet(Symbol, Box<Expression>, Box<Expression>),
    DirectFunctionCall(Symbol, Vec<Expression>),
    BuiltinFunctionCall(fn(&mut [Datum])->LispResult, Vec<Expression>),
    SpecialFunctionCall(String, Vec<Expression>),
    SymbolFunctionCall(Symbol, Vec<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    SelfEvaluating(Box<Datum>),
    Symbol(Symbol),
    DottedList(Vec<Datum>, Box<Datum>),
}
