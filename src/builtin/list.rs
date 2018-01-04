use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;

fn cons(vs: &mut[Datum]) -> LispResult {
    let fst = vs[0].take();
    let rst = vs[1].take();

    match rst {
        Datum::Nil => Ok(Datum::List(vec![fst])),
        Datum::DottedList(mut elems, tail) => {
            elems.insert(0, fst);
            Ok(Datum::DottedList(elems, tail))
        },
        Datum::List(mut elems) => {
            elems.insert(0, fst);
            Ok(Datum::List(elems))
        },
        other => Ok(Datum::DottedList(vec!(fst), Box::new(other))),
    }
}

fn fst(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        // TODO: find some way to ensure dotted list size >= 2
        Datum::DottedList(mut elems, _) => {
            Ok(elems[0].take())
        },
        Datum::List(mut elems) => {
            Ok(elems[0].take())
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn rst(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        // TODO: find some way to ensure dotted list size >= 2
        Datum::DottedList(elems, tail) => {
            if elems.len() == 1 {
                Ok(*tail)
            } else {
                let rest: Vec<Datum> = elems[1..].to_vec();
                Ok(Datum::DottedList(rest, tail))
            }
        },
        Datum::List(ref elems) => {
            if elems.len() == 1 {
                Ok(Datum::Nil)
            } else {
                let rest: Vec<Datum> = elems[1..].to_vec();
                Ok(Datum::List(rest))
            }
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn list(vs: &mut [Datum]) -> LispResult {
    if vs.len() == 0 {
        Ok(Datum::Nil)
    } else {
        Ok(Datum::List(vs.to_vec()))
    }
}

fn make_vector(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(len) = vs[0] {
        let default = vs[1].take();
        let vector = vec![default; len as usize];
        Ok(Datum::Vector(vector))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn nth(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(n) = vs[0] {
        match vs[1].take() {
            Datum::List(mut elems) => {
                Ok(elems.get_mut(n as usize).expect("Index out of bounds").take())
            },
            Datum::Vector(mut elems) => {
                Ok(elems.get_mut(n as usize).expect("Index out of bounds").take())
            },
            _ => {
                Err(InvalidTypeOfArguments)
            }
        }
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn length(vs: &mut [Datum]) -> LispResult {
    match vs[0] {
        Datum::Nil => Ok(Datum::Number(0)),
        Datum::List(ref elems) => {
            Ok(Datum::Number(elems.len() as i64))
        },
        Datum::Vector(ref elems) => {
            Ok(Datum::Number(elems.len() as i64))
        },
        Datum::Str(ref s) => {
            Ok(Datum::Number(s.len() as i64))
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn append(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        Datum::Nil => {
            Ok(vs[1].take())
        },
        Datum::List(mut elems) => {
            match vs[1].take() {
                Datum::List(elems2) => {
                    elems.extend(elems2);
                    Ok(Datum::List(elems))
                },
                Datum::DottedList(elems2, tail) => {
                    elems.extend(elems2);
                    Ok(Datum::DottedList(elems, tail))
                },
                Datum::Nil => {
                    Ok(Datum::List(elems))
                },
                other => {
                    Ok(Datum::DottedList(elems, Box::new(other)))
                }
            }
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn push(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        Datum::Nil => {
            Ok(Datum::List(vec!(vs[1].take())))
        },
        Datum::List(mut elems) => {
            elems.push(vs[1].take());
            Ok(Datum::List(elems))
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn reverse(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        Datum::List(mut elems) => {
            elems.reverse();
            Ok(Datum::List(elems))
        },
        Datum::Vector(mut elems) => {
            elems.reverse();
            Ok(Datum::Vector(elems))
        },
        _ => Err(InvalidTypeOfArguments),
    }
}

fn sort(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        Datum::List(mut elems) => {
            elems.sort();
            Ok(Datum::List(elems))
        },
        Datum::Vector(mut elems) => {
            elems.sort();
            Ok(Datum::Vector(elems))
        },
        _ => Err(InvalidTypeOfArguments),
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "cons", cons, Arity::Exact(2));
    register(hm, "fst", fst, Arity::Exact(1));
    register(hm, "rst", rst, Arity::Exact(1));
    register(hm, "list", list, Arity::Min(0));
    register(hm, "make-vector", make_vector, Arity::Range(1, 2));
    register(hm, "nth", nth, Arity::Exact(2));
    register(hm, "length", length, Arity::Exact(1));
    register(hm, "append", append, Arity::Exact(2));
    register(hm, "push", push, Arity::Exact(2));
    register(hm, "reverse", reverse, Arity::Exact(1));
    register(hm, "sort", sort, Arity::Exact(1));
}
