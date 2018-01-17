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
mod numbers;

use env::EnvRef;

use std::fmt;
use std::cmp::Ordering;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::mem;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

use numbers::Rational;

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
    Integer(isize),
    Rational(numbers::Rational),
    Bignum(bignum::Bignum),
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
}

impl Add for Datum {
    type Output = Datum;

    fn add(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a + b),
            (Datum::Rational(a), Datum::Integer(b)) => (a + b).reduce(),
            (Datum::Integer(a), Datum::Rational(b)) => (a + b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a + b).reduce(),
            (a, b) => panic!("Addition not implemented for {} and {}", a, b)
        }
    }
}

impl Sub for Datum {
    type Output = Datum;

    fn sub(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a - b),
            (Datum::Rational(a), Datum::Integer(b)) => (a - b).reduce(),
            (Datum::Integer(a), Datum::Rational(b)) => (a - b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a - b).reduce(),
            (a, b) => panic!("Subtraction not implemented for {} and {}", a, b)
        }
    }
}

impl Neg for Datum {
    type Output = Datum;

    fn neg(self) -> Datum {
        match self {
            Datum::Integer(a) => Datum::Integer(-a),
            a => panic!("Negation not implemented for {}", a)
        }
    }
}

impl Mul for Datum {
    type Output = Datum;

    fn mul(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a * b),
            (Datum::Integer(a), Datum::Rational(b)) => (a * b).reduce(),
            (Datum::Rational(a), Datum::Integer(b)) => (a * b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a * b).reduce(),
            (a, b) => panic!("Multiplication not implemented for {} and {}", a, b)
        }
    }
}

impl Div for Datum {
    type Output = Datum;

    fn div(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => {
                if a % b == 0 {
                    Datum::Integer(a / b)
                } else {
                    Datum::Rational(Rational::new(a, b))
                }
            },
            (Datum::Integer(a), Datum::Rational(b)) => (a / b).reduce(),
            (Datum::Rational(a), Datum::Integer(b)) => (a / b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a / b).reduce(),
            (a, b) => panic!("Division not implemented for {} and {}", a, b)
        }
    }
}

impl Rem for Datum {
    type Output = Datum;

    fn rem(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a % b),
            (a, b) => panic!("Remainder not implemented for {} and {}", a, b)
        }
    }
}

impl Rem<isize> for Datum {
    type Output = isize;

    fn rem(self, other: isize) -> isize {
        match (self, other) {
            (Datum::Integer(a), b) => a % b,
            (a, b) => panic!("Remainder not implemented for {} and {}", a, b)
        }
    }
}

trait ToDatum {
    fn to_datum(&self) -> Datum;
}

impl ToDatum for isize {
    fn to_datum(&self) -> Datum {
        Datum::Integer(*self)
    }
}

impl ToDatum for usize {
    fn to_datum(&self) -> Datum {
        Datum::Integer(*self as isize)
    }
}

// TODO: Fix this
impl Datum {
    fn from(other: &ToDatum) -> Datum {
        other.to_datum()
    }

    fn is_pair(&self) -> bool {
        match *self {
          Datum::DottedList(_, _) => true,
          Datum::List(_) => true,
          _ => false,
        }
    }

    fn is_nil(&self) -> bool {
        match *self {
          Datum::Nil => true,
          _ => false,
        }
    }

    fn is_list(&self) -> bool {
        match *self {
          Datum::List(_) => true,
          _ => false,
        }
    }

    fn take(&mut self) -> Datum {
        mem::replace(self, Datum::Undefined)
    }

    fn replace(&mut self, other: Datum) {
        mem::replace(self, other);
    }

    fn push(&mut self, value: Datum) {
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
            Datum::Integer(x) => write!(f, "{}", x),
            Datum::Rational(ref x) => write!(f, "{}", x),
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
