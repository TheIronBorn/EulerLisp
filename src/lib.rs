#![feature(io)]
#![feature(i128_type)]

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
mod symbol_table;
mod preprocess;
mod bignum;
mod numbers;
mod syntax_rule;
mod lexer;
mod stream;

use stream::LispIterator;
use env::EnvRef;

use std::fmt;
use std::cmp::Ordering;
use std::boxed::Box;
use std::mem;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

use numbers::Rational;
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};

pub type Fsize = f64;
pub type LispResult = Result<Datum, LispErr>;

#[derive(Debug, PartialEq)]
pub enum LispErr {
    InvalidNumberOfArguments,
    InvalidList,
    InvalidTypeOfArguments,
    IndexOutOfBounds,
    DefinitionAlreadyDefined,
    DefinitionNotFound,
    IOError,
    TypeError(&'static str, &'static str, Datum)
}

impl fmt::Display for LispErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispErr::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            LispErr::InvalidList => write!(f, "Invalid list"),
            LispErr::InvalidTypeOfArguments => write!(f, "Invalid types of arguments"),
            LispErr::IndexOutOfBounds => write!(f, "Index out of bounds"),
            LispErr::DefinitionNotFound => write!(f, "Definition not found"),
            LispErr::DefinitionAlreadyDefined => write!(f, "Definition is already defined"),
            LispErr::IOError => write!(f, "IO Error"),
            LispErr::TypeError(fun, expected, ref got) => {
                write!(f, "Type error evaluating {}: expected {}, got {:?}", fun, expected, got)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Arity {
    Exact(usize),
    Range(usize, usize),
    Min(usize)
}

impl Arity {
    fn check(&self, provided: usize, name: &String) {
        match *self {
            Arity::Exact(a) => {
                if a != provided {
                    panic!("{} expected {} arguments, got {}", name, a, provided);
                }
            },
            Arity::Min(a) => {
                if a > provided {
                    panic!("{} expected at least {} arguments, got {}", name, a, provided);
                }
            },
            Arity::Range(a, b) => {
                if provided < a || provided > b {
                    panic!("{} expected between {} and {} arguments, got {}", name, a, b, provided);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct LispFn(fn(&mut [Datum], &mut eval::Evaluator, EnvRef) -> LispResult, Arity, String);

impl Hash for LispFn {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.2.hash(state);
        self.1.hash(state);
    }
}

impl fmt::Debug for LispFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LispFn with arity {:?}", self.1)
    }
}

impl PartialEq for LispFn {
    fn eq(&self, _other: &LispFn) -> bool {
        false
    }
}

// #[derive(Clone, Debug, PartialEq)]
// pub enum Promise {
//     Delayed(EnvRef, Box<Datum>),
//     Result(Box<Datum>),
// }

#[derive(Clone)]
pub struct Lambda {
    id: usize,
    env: EnvRef,
    arity: usize,
    defaults: Vec<Datum>,
    body: Box<Meaning>,
    dotted: bool
}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lambda({:?})", self.arity)
    }
}

impl PartialEq for Lambda {
    fn eq(&self, _other: &Lambda) -> bool {
        false
    }
}

// TODO: Why do I need so many levels of indirection
// to get this to compile?
pub type Stream = Rc<RefCell<LispIterator>>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Pair(Datum, Datum);

impl Pair {
    pub fn compare(&self, other: &Pair) -> Result<Ordering, LispErr> {
        let res1 = self.0.compare(&other.0)?;
        if res1 == Ordering::Equal {
            self.1.compare(&other.1)
        } else {
            Ok(res1)
        }
    }

    fn collect(&self) -> Vec<Datum> {
        match &self.1 {
            &Datum::Pair(ref ptr) => {
                let mut rest = ptr.borrow().collect();
                rest.insert(0, self.0.clone());
                rest
            },
            other => {
                vec![self.0.clone(), other.clone()]
            }
        }
    }

    fn collect_list(&self) -> Result<Vec<Datum>, LispErr> {
        let mut v = self.collect();
        let last = v.pop().unwrap();

        if Datum::Nil == last {
            Ok(v)
        } else {
            Err(LispErr::InvalidList)
        }
    }
}

pub type PairRef = Rc<RefCell<Pair>>;

pub type Vector = Vec<Datum>;
pub type VectorRef = Rc<RefCell<Vector>>;

#[derive(Clone, Debug)]
pub enum Datum {
    Bool(bool),
    Integer(isize),
    Rational(numbers::Rational),
    Float(Fsize),
    Bignum(bignum::Bignum),
    Char(char),
    String(String),
    Symbol(Symbol),
    Pair(PairRef),
    Vector(VectorRef),
    Lambda(Lambda),
    Builtin(LispFn),
    Stream(usize, Box<Stream>),
    // Promise(Promise),
    HashMap(usize, HashMap<Datum, Datum>),
    Undefined,
    Nil,
}

impl PartialEq for Datum {
    fn eq(&self, other: &Datum) -> bool {
        match (self, other) {
            (&Datum::Bool(a), &Datum::Bool(b)) => a == b,
            (&Datum::Char(a), &Datum::Char(b)) => a == b,
            (&Datum::Symbol(a), &Datum::Symbol(b)) => a == b,
            (&Datum::String(ref a), &Datum::String(ref b)) => a == b,
            (&Datum::Integer(a), &Datum::Integer(b)) => a == b,
            (&Datum::Rational(ref a), &Datum::Rational(ref b)) => a == b,
            (&Datum::Bignum(ref a), &Datum::Bignum(ref b)) => a == b,
            (&Datum::Float(a), &Datum::Float(b)) => {
                // This is pretty bit hacky but better than not allowing floats
                // to be used as hash keys.
                // This eq is only meant to be used for hashmaps,
                // so it's not that bad.
                a.to_string() == b.to_string()
            },
            (&Datum::Pair(ref a1), &Datum::Pair(ref b1)) => {
                a1 == b1
            },
            (&Datum::Vector(ref a), &Datum::Vector(ref b)) => a == b,
            (&Datum::Lambda(ref a), &Datum::Lambda(ref b)) => a.id == b.id,
            (&Datum::Builtin(ref a), &Datum::Builtin(ref b)) => a == b,
            (&Datum::HashMap(id_a, _), &Datum::HashMap(id_b, _)) => id_a == id_b,
            (&Datum::Stream(id_a, _), &Datum::Stream(id_b, _)) => id_a == id_b,
            (&Datum::Undefined, &Datum::Undefined) => true,
            (&Datum::Nil, &Datum::Nil) => true,
            _ => false
        }
    }
}
impl Eq for Datum {}

// NOTE: The strings are there so that (pair a b) != (cons a b) != (list a b)
impl Hash for Datum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Datum::Bool(v) => v.hash(state),
            Datum::Integer(v) => v.hash(state),
            Datum::Rational(ref v) => v.hash(state),
            Datum::Bignum(ref v) => v.hash(state),
            Datum::Char(v) => v.hash(state),
            Datum::String(ref v) => {
                "string".hash(state);
                v.hash(state)
            },
            Datum::Symbol(v) => v.hash(state),
            Datum::Pair(ref ptr) => {
                "pair".hash(state);
                ptr.borrow().hash(state);
            },
            Datum::Vector(ref ptr) => {
                "vector".hash(state);
                for a in ptr.borrow().iter() {
                    a.hash(state);
                }
            },
            Datum::Undefined => {
                "undefined".hash(state);
            },
            Datum::Nil => {
                "nil".hash(state);
            },
            Datum::Lambda(ref l) => {
                "lambda".hash(state);
                l.id.hash(state);
            },
            Datum::Builtin(ref f) => {
                f.hash(state);
            },
            Datum::HashMap(id, _) => {
                "hashmap".hash(state);
                id.hash(state);
            },
            Datum::Stream(id, _) => {
                "stream".hash(state);
                id.hash(state);
            },
            Datum::Float(v) => {
                // This is pretty bit hacky but better than not allowing floats
                // to be used as hash keys.
                // This eq is only meant to be used for hashmaps,
                // so it's not that bad.
                "float".hash(state);
                v.to_string().hash(state);
            }
        }
    }
}

impl Add for Datum {
    type Output = Datum;

    // TODO: Allow these to return errors
    fn add(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => {
                match a.checked_add(b) {
                    Some(r) => Datum::Integer(r),
                    None => {
                       Datum::Bignum(
                           bignum::Bignum::new(a) +
                           bignum::Bignum::new(b)
                        )
                    }
                }
            },
            (Datum::Integer(a), Datum::Bignum(b)) => {
                Datum::Bignum(bignum::Bignum::new(a) + b)
            },
            (Datum::Bignum(a), Datum::Integer(b)) => {
                Datum::Bignum(a + bignum::Bignum::new(b))
            },
            (Datum::Bignum(a), Datum::Bignum(b)) => {
                Datum::Bignum(a + b)
            },
            (Datum::Rational(a), Datum::Integer(b)) => (a + b).reduce(),
            (Datum::Integer(a), Datum::Rational(b)) => (a + b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a + b).reduce(),
            (Datum::Float(f), other) => Datum::Float(f + other.as_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(f + other.as_float().unwrap()),
            (a, b) => panic!("Addition not implemented for {:?} and {:?}", a, b)
        }
    }
}

impl Sub for Datum {
    type Output = Datum;

    fn sub(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a - b),
            (Datum::Integer(a), Datum::Bignum(b)) => {
                Datum::Bignum(bignum::Bignum::new(a) - b)
            },
            (Datum::Bignum(a), Datum::Integer(b)) => {
                Datum::Bignum(a - bignum::Bignum::new(b))
            },
            (Datum::Bignum(a), Datum::Bignum(b)) => {
                Datum::Bignum(a - b)
            },
            (Datum::Rational(a), Datum::Integer(b)) => (a - b).reduce(),
            (Datum::Integer(a), Datum::Rational(b)) => (a - b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a - b).reduce(),
            (Datum::Float(f), other) => Datum::Float(f - other.as_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(other.as_float().unwrap() - f),
            (a, b) => panic!("Subtraction not implemented for {:?} and {:?}", a, b)
        }
    }
}

impl Neg for Datum {
    type Output = Datum;

    fn neg(self) -> Datum {
        match self {
            Datum::Integer(a) => Datum::Integer(-a),
            Datum::Float(a) => Datum::Float(-a),
            Datum::Rational(a) => Datum::Rational(-a),
            a => panic!("Negation not implemented for {:?}", a)
        }
    }
}

impl Mul for Datum {
    type Output = Datum;

    fn mul(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => {
                match a.checked_mul(b) {
                    Some(r) => Datum::Integer(r),
                    None => {
                       Datum::Bignum(
                           bignum::Bignum::new(a) *
                           bignum::Bignum::new(b)
                        )
                    }
                }
            },
            (Datum::Integer(a), Datum::Rational(b)) => (a * b).reduce(),
            (Datum::Integer(a), Datum::Bignum(b)) => {
                Datum::Bignum(bignum::Bignum::new(a) * b)
            },
            (Datum::Bignum(a), Datum::Integer(b)) => {
                Datum::Bignum(a * bignum::Bignum::new(b))
            },
            (Datum::Bignum(a), Datum::Bignum(b)) => {
                Datum::Bignum(a * b)
            },
            (Datum::Rational(a), Datum::Integer(b)) => (a * b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a * b).reduce(),
            (Datum::Float(f), other) => Datum::Float(f * other.as_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(f * other.as_float().unwrap()),
            (a, b) => panic!("Multiplication not implemented for {:?} and {:?}", a, b)
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
            (Datum::Float(f), other) => Datum::Float(f / other.as_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(other.as_float().unwrap() / f),
            (a, b) => panic!("Division not implemented for {:?} and {:?}", a, b)
        }
    }
}

impl Rem for Datum {
    type Output = Datum;

    fn rem(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a % b),
            (a, b) => panic!("Remainder not implemented for {:?} and {:?}", a, b)
        }
    }
}

impl Rem<isize> for Datum {
    type Output = isize;

    fn rem(self, other: isize) -> isize {
        match (self, other) {
            (Datum::Integer(a), b) => a % b,
            (a, b) => panic!("Remainder not implemented for {:?} and {:?}", a, b)
        }
    }
}

trait ToDatum {
    fn to_datum(&self) -> Datum;
}

impl ToDatum for u8 {
    fn to_datum(&self) -> Datum {
        Datum::Integer(*self as isize)
    }
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

impl Datum {
    fn make_list(elems: &mut [Datum]) -> Datum {
        let mut res = Datum::Nil;
        for next in elems.into_iter().rev() {
            let pair = Pair(next.take(), res);
            res = Datum::Pair(Rc::new(RefCell::new(pair)));
        }
        res
    }

    fn make_list_from_vec(elems: Vec<Datum>) -> Datum {
        let mut res = Datum::Nil;
        for mut next in elems.into_iter().rev() {
            let pair = Pair(next.take(), res);
            res = Datum::Pair(Rc::new(RefCell::new(pair)));
        }
        res
    }

    fn make_vector(elems: &mut [Datum]) -> Datum {
        Datum::Vector(Rc::new(RefCell::new(elems.to_vec())))
    }

    fn make_vector_from_vec(elems: Vec<Datum>) -> Datum {
        Datum::Vector(Rc::new(RefCell::new(elems)))
    }

    fn make_dotted_list_from_vec(elems: Vec<Datum>, tail: Datum) -> Datum {
        let mut res = tail;
        for mut next in elems.into_iter().rev() {
            let pair = Pair(next.take(), res);
            res = Datum::Pair(Rc::new(RefCell::new(pair)));
        }
        res
    }

    fn make_pair(fst: Datum, rst: Datum) -> Datum {
        let pair = Pair(fst, rst);
        Datum::Pair(Rc::new(RefCell::new(pair)))
    }

    fn from(other: &ToDatum) -> Datum {
        other.to_datum()
    }

    fn is_pair(&self) -> bool {
        match *self {
          Datum::Pair(_) => true,
          _ => false,
        }
    }

    fn is_nil(&self) -> bool {
        match *self {
          Datum::Nil => true,
          _ => false,
        }
    }

    fn take(&mut self) -> Datum {
        mem::replace(self, Datum::Undefined)
    }

    fn as_float(&self) -> Result<Fsize, LispErr> {
        match self {
            &Datum::Integer(n) => Ok(n as Fsize),
            &Datum::Rational(ref r) => Ok((r.num as Fsize) / (r.denom as Fsize)),
            &Datum::Float(r) => Ok(r),
            other => Err(LispErr::TypeError("convert", "float", other.clone()))
        }
    }

    fn as_integer(&self) -> Result<isize, LispErr> {
        match self {
            &Datum::Integer(n) => Ok(n),
            other => Err(LispErr::TypeError("convert", "integer", other.clone()))
        }
    }

    fn as_uinteger(&self) -> Result<usize, LispErr> {
        match self {
            &Datum::Integer(n) => {
                if n >= 0 {
                    Ok(n as usize)
                } else {
                    Err(LispErr::TypeError("convert", "uinteger", self.clone()))
                }
            },
            other => Err(LispErr::TypeError("convert", "uinteger", other.clone()))
        }
    }

    fn as_symbol(&self) -> Result<Symbol, LispErr> {
        match self {
            &Datum::Symbol(n) => Ok(n),
            other => Err(LispErr::TypeError("convert", "symbol", other.clone()))
        }
    }

    fn as_string(&self) -> Result<String, LispErr> {
        match self {
            &Datum::String(ref n) => Ok(n.clone()),
            other => Err(LispErr::TypeError("convert", "string", other.clone()))
        }
    }

    fn as_char(&self) -> Result<char, LispErr> {
        match self {
            &Datum::Char(n) => Ok(n),
            other => Err(LispErr::TypeError("convert", "char", other.clone()))
        }
    }

    fn as_pair(&self) -> Result<Ref<Pair>, LispErr> {
        match self {
            &Datum::Pair(ref ptr) => Ok(ptr.borrow()),
            other => Err(LispErr::TypeError("convert", "pair", other.clone()))
        }
    }

    fn as_mut_pair(&self) -> Result<RefMut<Pair>, LispErr> {
        match self {
            &Datum::Pair(ref ptr) => Ok(ptr.borrow_mut()),
            other => Err(LispErr::TypeError("convert", "pair", other.clone()))
        }
    }

    fn as_vector(&self) -> Result<Ref<Vector>, LispErr> {
        match self {
            &Datum::Vector(ref ptr) => Ok(ptr.borrow()),
            other => Err(LispErr::TypeError("convert", "vector", other.clone()))
        }
    }

    fn as_mut_vector(&self) -> Result<RefMut<Vector>, LispErr> {
        match self {
            &Datum::Vector(ref ptr) => Ok(ptr.borrow_mut()),
            other => Err(LispErr::TypeError("convert", "vector", other.clone()))
        }
    }

    // TODO: Remove some borrow & clones
    fn as_list(&self) -> Result<Vec<Datum>, LispErr> {
        match self {
            &Datum::Pair(ref ptr) => {
                let mut cur = ptr.clone();
                let mut res = Vec::new();
                loop {
                    res.push(cur.borrow().0.clone());
                    let rst = cur.borrow().1.clone();
                    match rst {
                        Datum::Pair(ptr) => {
                            cur = ptr.clone();
                            continue
                        },
                        Datum::Nil => {
                            break
                        },
                        _ => {
                            return Err(LispErr::InvalidList)
                        }
                    }
                }
                Ok(res)
            },
            &Datum::Nil => Ok(Vec::new()),
            a => panic!("Can't convert {:?} to a list", a)
        }
    }

    // TODO: Find a way to do this without clone
    fn as_hashmap(&self) -> Result<HashMap<Datum, Datum>, LispErr> {
        match self {
            &Datum::HashMap(_, ref n) => Ok(n.clone()),
            a => panic!("Can't convert {:?} to a hashmap", a)
        }
    }

    fn is_false(&self) -> bool {
        match self {
            &Datum::Nil => true,
            &Datum::Bool(false) => true,
            _ => false
        }
    }

    fn is_true(&self) -> bool {
        match self {
            &Datum::Nil => false,
            &Datum::Bool(false) => false,
            _ => true
        }
    }

    // TODO: Better error handling
    fn compare(&self, other: &Datum) -> Result<Ordering, LispErr> {
        match (self, other) {
            (&Datum::Integer(ref a), &Datum::Integer(ref b)) => Ok(a.cmp(b)),
            (&Datum::Bignum(ref a), &Datum::Bignum(ref b)) => Ok(a.cmp(b)),
            (&Datum::Integer(a), &Datum::Bignum(ref b)) => Ok(bignum::Bignum::new(a).cmp(b)),
            (&Datum::Bignum(ref a), &Datum::Integer(b)) => Ok(a.cmp(&bignum::Bignum::new(b))),
            (&Datum::Rational(ref a), &Datum::Rational(ref b)) => Ok(
                (a.num * b.denom).cmp(&(b.num * a.denom))
            ),
            (&Datum::Integer(ref a), &Datum::Rational(ref b)) => Ok(
                (a *  b.denom).cmp(&(b.num))
            ),
            (&Datum::Rational(ref a), &Datum::Integer(ref b)) => Ok(
                a.num.cmp(&(b *  a.denom))
            ),
            (ref other, &Datum::Float(ref b)) => {
                Ok((other.as_float()?).partial_cmp(b).unwrap())
            },
            (&Datum::Float(ref b), ref other) => {
                Ok(b.partial_cmp(&other.as_float()?).unwrap())
            },
            (&Datum::String(ref a), &Datum::String(ref b)) => Ok(a.cmp(b)),
            (&Datum::Char(ref a), &Datum::Char(ref b)) => Ok(a.cmp(b)),
            (&Datum::Pair(ref a), &Datum::Pair(ref b)) => a.borrow().compare(&b.borrow()),
            (a, b) => panic!("Can't compare {:?} and {:?}", a, b)
        }
    }

    fn to_string(&self, symbol_table: &mut symbol_table::SymbolTable) -> String {
        match *self {
            Datum::Symbol(x) => symbol_table.lookup(x),
            Datum::Bool(x) => {
                if x {
                    String::from("#t")
                } else {
                    String::from("#f")
                }
            },
            Datum::Char(c) => format!("#\\{}", c),
            Datum::Pair(ref ptr) => {
                let pair = ptr.borrow();
                let elems = pair.collect();
                let head = &elems[..(elems.len() - 1)];
                let tail = &elems[elems.len() - 1];

                let mut result = String::new();
                result.push_str("(");

                for (i, e) in head.iter().enumerate() {
                    if i != 0 {
                        result.push_str(" ");
                    }
                    result.push_str(&e.to_string(symbol_table));
                }

                match tail {
                    &Datum::Nil => {
                        result.push_str(")");
                    },
                    other => {
                        result.push_str(" . ");
                        result.push_str(&other.to_string(symbol_table));
                        result.push_str(")");
                    },
                }

                format!("{}", result)
            }
            Datum::Vector(ref elems) => {
                let mut result = String::new();
                result.push_str("#(");
                for (i, e) in elems.borrow().iter().enumerate() {
                    if i != 0 {
                        result.push_str(" ");
                    }
                    result.push_str(&e.to_string(symbol_table));
                }
                result.push_str(")");
                format!("{}", result)
            },
            Datum::Integer(x) => format!("{}", x),
            Datum::Rational(ref x) => format!("{}", x),
            Datum::Bignum(ref x) => format!("{}", x),
            Datum::Float(x) => format!("{}", x),
            Datum::String(ref s) => format!("\"{}\"", s),
            Datum::Nil => format!("'()"),
            Datum::Undefined => format!("undefined"),
            Datum::Lambda(_) => format!("<lambda>"),
            Datum::Builtin(_) => format!("<builtin>"),
            Datum::Stream(_, _) => format!("<stream>"),
            Datum::HashMap(_, _) => format!("<hashmap>"),
            // Datum::Promise(Promise::Delayed(_, _)) => write!(f, "promise(?)"),
            // Datum::Promise(Promise::Result(ref r)) => write!(f, "promise({})", r),
        }
    }
}

pub type Symbol = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct BindingRef(usize, usize);

#[derive(Clone)]
pub enum Meaning {
    If(Box<Meaning>, Box<Meaning>, Box<Meaning>),
    LambdaDef(usize, Vec<Datum>, Box<Meaning>, bool),
    Do(Vec<Meaning>, Box<Meaning>),
    Quote(Box<Datum>),
    Definition(Box<Meaning>),
    Assignment(BindingRef, Box<Meaning>),
    BuiltinFunctionCall(fn(&mut [Datum], &mut eval::Evaluator, EnvRef)->LispResult, Vec<Meaning>),
    FunctionCall(Box<Meaning>, Vec<Meaning>),
    SelfEvaluating(Box<Datum>),
    BindingRef(BindingRef),
    SyntaxRuleDefinition(Symbol, Box<syntax_rule::SyntaxRule>),
}

impl Meaning {
    pub fn self_evaluating(datum: Datum) -> Meaning {
        Meaning::SelfEvaluating(Box::new(datum))
    }

    pub fn make_if(cond: Meaning, cons: Meaning, alt: Meaning) -> Meaning {
        Meaning::If(Box::new(cond), Box::new(cons), Box::new(alt))
    }

    pub fn datum_nil() -> Meaning {
        Meaning::SelfEvaluating(Box::new(Datum::Nil))
    }

    pub fn datum_true() -> Meaning {
        Meaning::SelfEvaluating(Box::new(Datum::Bool(true)))
    }

    pub fn datum_false() -> Meaning {
        Meaning::SelfEvaluating(Box::new(Datum::Bool(false)))
    }
}
