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

fn make_list(vs: &mut [Datum]) -> LispResult {
    if let Datum::Integer(len) = vs[0] {
        let default = vs[1].take();
        let vector = vec![default; len as usize];
        Ok(Datum::List(vector))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn nth(vs: &mut [Datum]) -> LispResult {
    if let Datum::Integer(n) = vs[0] {
        match vs[1].take() {
            Datum::List(mut elems) => {
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
        Datum::Nil => Ok(Datum::Integer(0)),
        Datum::List(ref elems) => {
            Ok(Datum::from(&elems.len()))
        },
        Datum::Str(ref s) => {
            Ok(Datum::from(&s.len()))
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
        _ => Err(InvalidTypeOfArguments),
    }
}

fn sort(vs: &mut [Datum]) -> LispResult {
    match vs[0].take() {
        Datum::List(mut elems) => {
            elems.sort();
            Ok(Datum::List(elems))
        },
        _ => Err(InvalidTypeOfArguments),
    }
}

fn permutations(vs: &mut [Datum]) -> LispResult {
    if let Datum::List(mut elems) = vs[0].take() {
        let mut result: Vec<Datum> = Vec::new();

        // Heap's algorithm
        let n = elems.len();
        let mut c = vec![0; n]; 

        result.push(Datum::List(elems.clone()));
        let mut i = 0;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    elems.swap(0, i);
                } else {
                    elems.swap(c[i], i);
                }
                result.push(Datum::List(elems.clone()));
                c[i] += 1;
                i = 0;
            } else {
                c[i] = 0;
                i += 1;
            }
        }

        Ok(Datum::List(result))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn combinations(vs: &mut [Datum]) -> LispResult {
    if let Datum::Integer(len) = vs[0].take() {
        if let Datum::List(mut elems) = vs[1].take() {
            let max = elems.len();
            let mut counters = vec![0; len as usize];
            let mut result: Vec<Datum> = Vec::new();
            let mut done = false;

            let len = len as usize;

            while !done {
                let cur : Vec<Datum> = counters.iter().map(|c| elems[*c].clone()).collect();
                result.push(Datum::List(cur));

                for i in (0..len) {
                    let new = counters[i] + 1;
                    if new >= max {
                        counters[i] = 0;
                        if i == (len - 1) {
                            done = true;
                        }
                    } else {
                        counters[i] = new;
                        break;
                    }
                }
            }

            Ok(Datum::List(result))
        } else {
            Err(InvalidTypeOfArguments)
        }
    } else {
        Err(InvalidTypeOfArguments)
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "cons", cons, Arity::Exact(2));
    register(hm, "fst", fst, Arity::Exact(1));
    register(hm, "rst", rst, Arity::Exact(1));
    register(hm, "list", list, Arity::Min(0));
    register(hm, "filled-list", make_list, Arity::Range(1, 2));
    register(hm, "nth", nth, Arity::Exact(2));
    register(hm, "length", length, Arity::Exact(1));
    register(hm, "append", append, Arity::Exact(2));
    register(hm, "push", push, Arity::Exact(2));
    register(hm, "reverse", reverse, Arity::Exact(1));
    register(hm, "sort", sort, Arity::Exact(1));
    register(hm, "permutations", permutations, Arity::Exact(1));
    register(hm, "combinations", combinations, Arity::Exact(2));
}
