use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;

use ::builtin::register;

fn cons(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    // TODO: Can this be done without clone?
    let fst = vs[0].clone();
    let rst = vs[1].clone();

    match rst {
        Datum::Nil => Ok(Datum::List(vec![fst])),
        Datum::DottedList(ref elems, ref tail) => {
            let mut new = elems.clone();
            new.insert(0, fst);
            Ok(Datum::DottedList(new, tail.clone()))
        },
        Datum::List(ref elems) => {
            let mut new = elems.clone();
            new.insert(0, fst);
            Ok(Datum::List(new))
        },
        other => Ok(Datum::DottedList(vec!(fst), Box::new(other))),
    }
}

fn fst(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    match vs[0] {
        // TODO: find some way to ensure dotted list size >= 2
        Datum::DottedList(ref elems, _) => {
            Ok(elems.first().unwrap().clone())
        },
        Datum::List(ref elems) => {
            Ok(elems.first().unwrap().clone())
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn rst(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    match vs[0] {
        // TODO: find some way to ensure dotted list size >= 2
        Datum::DottedList(ref elems, ref tail) => {
            if elems.len() == 1 {
                // What is this strange creature?
                // ** unboxes a ref to a box
                Ok((**tail).clone())
            } else {
                let rest: Vec<Datum> = elems[1..].iter().map(|v| v.clone()).collect();
                Ok(Datum::DottedList(rest, tail.clone()))
            }
        },
        Datum::List(ref elems) => {
            if elems.len() == 1 {
                Ok(Datum::Nil)
            } else {
                let rest: Vec<Datum> = elems[1..].iter().map(|v| v.clone()).collect();
                Ok(Datum::List(rest))
            }
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn list(vs: Vec<Datum>) -> LispResult {
    if vs.len() == 0 {
        Ok(Datum::Nil)
    } else {
        Ok(Datum::List(vs))
    }
}

fn make_vector(vs: Vec<Datum>) -> LispResult {
    if vs.len() < 1 || vs.len() > 2 {
        return Err(InvalidNumberOfArguments);
    }

    if let Datum::Number(len) = *vs.get(0).unwrap() {
        let default = vs.get(1).unwrap_or(&Datum::Undefined);
        let vector = vec![default.clone(); len as usize];
        Ok(Datum::Vector(vector))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn nth(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    // TODO: Handle index out of bounds error
    if let Datum::Number(n) = vs[0] {
        match vs[1] {
            Datum::List(ref elems) => {
                Ok(elems.get(n as usize).expect("Index out of bounds").clone())
            },
            Datum::Vector(ref elems) => {
                Ok(elems.get(n as usize).expect("Index out of bounds").clone())
            },
            _ => {
                Err(InvalidTypeOfArguments)
            }
        }
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn length(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
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

fn append(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    match vs[0] {
        Datum::Nil => {
            Ok(vs[1].clone())
        },
        Datum::List(ref elems) => {
            match vs[1] {
                Datum::List(ref elems2) => {
                    let mut new_elems = elems.clone();
                    new_elems.extend(elems2.iter().cloned());
                    Ok(Datum::List(new_elems))
                },
                Datum::DottedList(ref elems2, ref tail) => {
                    let mut new_elems = elems.clone();
                    new_elems.extend(elems2.iter().cloned());
                    Ok(Datum::DottedList(new_elems, tail.clone()))
                },
                Datum::Nil => {
                    Ok(Datum::List(elems.clone()))
                },
                ref other => {
                    Ok(Datum::DottedList(elems.clone(), Box::new(other.clone())))
                }
            }
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn push(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    match vs[0] {
        Datum::Nil => {
            Ok(Datum::List(vec!(vs[1].clone())))
        },
        Datum::List(ref elems) => {
            let mut new_elems = elems.clone();
            new_elems.push(vs[1].clone());
            Ok(Datum::List(new_elems))
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn reverse(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);

    match vs[0] {
        Datum::List(ref elems) => {
            let new_elems = elems.iter().rev().cloned().collect();
            Ok(Datum::List(new_elems))
        },
        Datum::Vector(ref elems) => {
            let new_elems = elems.iter().rev().cloned().collect();
            Ok(Datum::Vector(new_elems))
        },
        _ => Err(InvalidTypeOfArguments),
    }
}

fn sort(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);

    match vs[0] {
        Datum::List(ref elems) => {
            let mut new_elems = elems.clone();
            new_elems.sort();
            Ok(Datum::List(new_elems))
        },
        Datum::Vector(ref elems) => {
            let mut new_elems = elems.clone();
            new_elems.sort();
            Ok(Datum::Vector(new_elems))
        },
        _ => Err(InvalidTypeOfArguments),
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "cons", cons);
    register(hm, "fst", fst);
    register(hm, "rst", rst);
    register(hm, "list", list);
    register(hm, "make-vector", make_vector);
    register(hm, "nth", nth);
    register(hm, "length", length);
    register(hm, "append", append);
    register(hm, "push", push);
    register(hm, "reverse", reverse);
    register(hm, "sort", sort);
}
