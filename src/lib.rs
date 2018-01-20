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

use env::EnvRef;

use std::fmt;
use std::cmp::Ordering;
use std::boxed::Box;
use std::mem;

use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Rem;

use numbers::Rational;

pub type Fsize = f64;

#[derive(PartialEq, Debug, Clone)]
pub enum Stream {
    Range(RangeStream),
    Step(StepStream),
    Map(MapStream),
    Select(SelectStream)
}

impl Stream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        match self {
            &mut Stream::Range(ref mut rs) => rs.next(eval, env_ref),
            &mut Stream::Step(ref mut rs) => rs.next(eval, env_ref),
            &mut Stream::Map(ref mut rs) => rs.next(eval, env_ref),
            &mut Stream::Select(ref mut rs) => rs.next(eval, env_ref)
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct RangeStream {
    from: isize,
    to: isize,
    step: isize,
    current: isize
}

#[derive(PartialEq, Clone, Debug)]
pub struct StepStream {
    from: isize,
    step: isize,
    current: isize
}

#[derive(PartialEq, Clone, Debug)]
pub struct MapStream {
    source: Box<Stream>,
    fun: Box<Datum>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct SelectStream {
    source: Box<Stream>,
    fun: Box<Datum>,
}

trait LispIterator {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum>;
}

impl RangeStream {
    pub fn new(from: isize, to: isize, step: isize) -> RangeStream {
        RangeStream {
            from: from,
            to: to,
            step: step,
            current: from
        }
    }
}

impl LispIterator for RangeStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        if self.current > self.to {
            None
        } else {
            let ret = self.current;
            self.current += self.step;
            Some(Datum::Integer(ret))
        }
    }
}

impl StepStream {
    pub fn new(from: isize, step: isize) -> StepStream {
        StepStream {
            from: from,
            step: step,
            current: from
        }
    }
}

impl LispIterator for StepStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        let ret = self.current;
        self.current += self.step;
        Some(Datum::Integer(ret))
    }
}

impl LispIterator for MapStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        let next = self.source.next(eval, env_ref.clone());

        match next {
            Some(v) => Some(eval.full_apply((*self.fun).clone(), vec![v], env_ref)),
            None => None
        }
    }
}

impl LispIterator for SelectStream {
    fn next(&mut self, eval: &mut eval::Evaluator, env_ref: EnvRef) -> Option<Datum> {
        loop {
            let next = self.source.next(eval, env_ref.clone());
            match next {
                Some(v) => {
                    if let Datum::Bool(true) = eval.full_apply((*self.fun).clone(), vec![v.clone()], env_ref.clone()) {
                        return Some(v);
                    }
                },
                None => return None
            }
        }
    }
}

pub type LispResult = Result<Datum, LispErr>;

#[derive(Debug, PartialEq)]
pub enum LispErr {
    InvalidNumberOfArguments,
    InvalidTypeOfArguments,
    DefinitionAlreadyDefined,
    DefinitionNotFound,
    IOError,
    TypeError(&'static str, &'static str, Datum)
}

impl fmt::Display for LispErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LispErr::InvalidNumberOfArguments => write!(f, "Invalid number of arguments"),
            LispErr::InvalidTypeOfArguments => write!(f, "Invalid types of arguments"),
            LispErr::DefinitionNotFound => write!(f, "Definition not found"),
            LispErr::DefinitionAlreadyDefined => write!(f, "Definition is already defined"),
            LispErr::IOError => write!(f, "IO Error"),
            LispErr::TypeError(fun, expected, ref got) => {
                write!(f, "Type error evaluating {}: expected {}, got {:?}", fun, expected, got)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
pub struct LispFn(fn(&mut [Datum], &mut eval::Evaluator, EnvRef) -> LispResult, Arity);

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
impl Eq for LispFn {}

#[derive(Clone, Debug, PartialEq)]
pub enum Promise {
    Delayed(EnvRef, Box<Datum>),
    Result(Box<Datum>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq)]
pub enum Datum {
    Bool(bool),
    Integer(isize),
    Rational(numbers::Rational),
    Float(Fsize),
    Bignum(bignum::Bignum),
    Character(char),
    Str(String),
    Symbol(String),
    List(Vec<Datum>),
    DottedList(Vec<Datum>, Box<Datum>),
    Lambda(Lambda),
    Builtin(LispFn),
    Promise(Promise),
    Stream(Stream),
    Undefined,
    Nil,
}

impl Add for Datum {
    type Output = Datum;

    // TODO: Allow these to return errors
    fn add(self, other: Datum) -> Datum {
        match (self, other) {
            (Datum::Integer(a), Datum::Integer(b)) => Datum::Integer(a + b),
            (Datum::Rational(a), Datum::Integer(b)) => (a + b).reduce(),
            (Datum::Integer(a), Datum::Rational(b)) => (a + b).reduce(),
            (Datum::Rational(a), Datum::Rational(b)) => (a + b).reduce(),
            (Datum::Float(f), other) => Datum::Float(f + other.to_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(f + other.to_float().unwrap()),
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
            (Datum::Float(f), other) => Datum::Float(f - other.to_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(other.to_float().unwrap() - f),
            (a, b) => panic!("Subtraction not implemented for {} and {}", a, b)
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
            (Datum::Float(f), other) => Datum::Float(f * other.to_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(f * other.to_float().unwrap()),
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
            (Datum::Float(f), other) => Datum::Float(f / other.to_float().unwrap()),
            (other, Datum::Float(f)) => Datum::Float(other.to_float().unwrap() / f),
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

    fn to_float(&self) -> Result<Fsize, LispErr> {
        match self {
            &Datum::Integer(n) => Ok(n as Fsize),
            &Datum::Rational(ref r) => Ok((r.num as Fsize) / (r.denom as Fsize)),
            &Datum::Float(r) => Ok(r),
            a => panic!("Can't convert {} to float", a)
        }
    }

    // TODO: Better error handling
    fn compare(&self, other: &Datum) -> Result<Ordering, LispErr> {
        match (self, other) {
            (&Datum::Integer(ref a), &Datum::Integer(ref b)) => Ok(a.cmp(b)),
            (&Datum::Rational(ref a), &Datum::Rational(ref b)) => Ok(
                (a.num * b.denom).cmp(&(b.num * a.denom))
            ),
            (ref other, &Datum::Float(ref b)) => {
                Ok((other.to_float()?).partial_cmp(b).unwrap())
            },
            (&Datum::Float(ref b), ref other) => {
                Ok(b.partial_cmp(&other.to_float()?).unwrap())
            },
            (a, b) => panic!("Can't compare {} and {}", a, b)
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
            Datum::Float(x) => write!(f, "{}", x),
            Datum::Str(ref s) => write!(f, "\"{}\"", s),
            Datum::Nil => write!(f, "'()"),
            Datum::Undefined => write!(f, "undefined"),
            Datum::Lambda(_) => write!(f, "<lambda>"),
            Datum::Builtin(_) => write!(f, "<builtin>"),
            Datum::Stream(_) => write!(f, "<stream>"),
            Datum::Promise(Promise::Delayed(_, _)) => write!(f, "promise(?)"),
            Datum::Promise(Promise::Result(ref r)) => write!(f, "promise({})", r),
        }
    }
}

pub type Symbol = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct BindingRef(usize, usize);

#[derive(Clone)]
pub enum Expression {
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    // TODO: No need to keep the param names in the lambda
    LambdaDef(Vec<Symbol>, Vec<Datum>, Box<Expression>, LambdaType),
    Do(Vec<Expression>, Box<Expression>),
    Quote(Box<Datum>),
    Definition(BindingRef, Box<Expression>),
    Assignment(BindingRef, Box<Expression>),
    ListPush(BindingRef, Box<Expression>),
    ListRef(BindingRef, Box<Expression>),
    ListSet(BindingRef, Box<Expression>, Box<Expression>),
    BuiltinFunctionCall(fn(&mut [Datum], &mut eval::Evaluator, EnvRef)->LispResult, Vec<Expression>),
    FunctionCall(Box<Expression>, Vec<Expression>),
    SelfEvaluating(Box<Datum>),
    BindingRef(BindingRef),
    DottedList(Vec<Datum>, Box<Datum>),
    // Case(Box<Expression>, BTreeMap<Datum, Expression>, Box<Expression>),
    // MacroDefinition(BindingRef, Box<Expression>),
}

impl Expression {
    pub fn make_self_evaluating(datum: Datum) -> Expression {
        Expression::SelfEvaluating(Box::new(datum))
    }

    pub fn make_if(cond: Expression, cons: Expression, alt: Expression) -> Expression {
        Expression::If(Box::new(cond), Box::new(cons), Box::new(alt))
    }

    pub fn datum_nil() -> Expression {
        Expression::SelfEvaluating(Box::new(Datum::Nil))
    }

    pub fn datum_true() -> Expression {
        Expression::SelfEvaluating(Box::new(Datum::Bool(true)))
    }

    pub fn datum_false() -> Expression {
        Expression::SelfEvaluating(Box::new(Datum::Bool(false)))
    }
}
