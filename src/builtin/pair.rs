use std::collections::HashMap;

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

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "cons", cons);
    register(hm, "fst", fst);
    register(hm, "rst", rst);
}
