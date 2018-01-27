use std::collections::HashMap;
use std::cmp::Ordering;

use ::LispFn;
use ::Datum;
use ::LispErr;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::eval::Evaluator;
use ::EnvRef;
use ::builtin::register;

fn cons(vs: &mut[Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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

fn pair(vs: &mut[Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let fst = vs[0].take();
    let rst = vs[1].take();

    Ok(Datum::Pair(Box::new(fst), Box::new(rst)))
}

fn fst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::DottedList(mut elems, _) => {
            Ok(elems[0].take())
        },
        Datum::List(mut elems) => {
            Ok(elems[0].take())
        },
        Datum::Pair(fst, _rst) => {
            Ok(*fst)
        },
        _ => panic!("fst only works on pairs")
    }
}

fn rst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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
        Datum::Pair(_fst, rst) => {
            Ok(*rst)
        },
        _ => panic!("fst only works on pairs")
    }
}

fn list(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if vs.len() == 0 {
        Ok(Datum::Nil)
    } else {
        Ok(Datum::List(vs.to_vec()))
    }
}

fn make_list(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(len) = vs[0] {
        let default = vs[1].take();
        let vector = vec![default; len as usize];
        Ok(Datum::List(vector))
    } else {
        Err(InvalidTypeOfArguments)
    }
}

fn nth(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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

fn length(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0] {
        Datum::Nil => Ok(Datum::Integer(0)),
        Datum::List(ref elems) => {
            Ok(Datum::from(&elems.len()))
        },
        Datum::String(ref s) => {
            Ok(Datum::from(&s.len()))
        },
        _ => Err(InvalidTypeOfArguments)
    }
}

fn append(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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

fn push(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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

fn reverse(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::List(mut elems) => {
            elems.reverse();
            Ok(Datum::List(elems))
        },
        Datum::Nil => Ok(Datum::Nil),
        _ => Err(InvalidTypeOfArguments),
    }
}

fn sort(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::List(mut elems) => {
            let es = elems.as_mut_slice();
            let len = es.len();
            quicksort_helper(es, 0, (len - 1) as isize)?;
            Ok(Datum::List(es.to_vec()))
        },
        Datum::Nil => Ok(Datum::Nil),
        _ => Err(InvalidTypeOfArguments),
    }
}

fn quicksort_helper (arr: &mut [Datum], left: isize, right: isize) -> Result<bool, LispErr> {
    if right <= left {
        return Ok(true);
    }

    let mut i: isize = left - 1;
    let mut j: isize = right;
    let mut p: isize = i;
    let mut q: isize = j;
    unsafe {
        let v: *mut Datum = &mut arr[right as usize];
        loop {
            i += 1;
            while (&arr[i as usize]).compare(&*v).unwrap() == Ordering::Less {
                i += 1
            }
            j -= 1;
            while (&*v).compare(&arr[j as usize]).unwrap() == Ordering::Less {
                if j == left {
                    break
                }
                j -= 1;
            }
            if i >= j {
                break
            }
            arr.swap(i as usize, j as usize);
            if (&arr[i as usize]).compare(&*v).unwrap() == Ordering::Equal {
                p += 1;
                arr.swap(p as usize, i as usize)
            }
            if (&*v).compare(&arr[j as usize]).unwrap() == Ordering::Equal {
                q -= 1;
                arr.swap(j as usize, q as usize)
            }
        }
    }

    arr.swap(i as usize, right as usize);
    j = i - 1;
    i += 1;
    let mut k: isize = left;
    while k < p {
        arr.swap(k as usize, j as usize);
        k += 1;
        j -= 1;
        assert!(k < arr.len() as isize);
    }
    k = right - 1;
    while k > q {
        arr.swap(i as usize, k as usize);
        k -= 1;
        i += 1;
        assert!(k != 0);
    }

    quicksort_helper(arr, left, j)?;
    quicksort_helper(arr, i, right)?;

    Ok(true)
}

fn permutations(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
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

fn combinations(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(len) = vs[0].take() {
        if let Datum::List(elems) = vs[1].take() {
            let max = elems.len();
            let mut counters = vec![0; len as usize];
            let mut result: Vec<Datum> = Vec::new();
            let mut done = false;

            let len = len as usize;

            while !done {
                let cur : Vec<Datum> = counters.iter().map(|c| elems[*c].clone()).collect();
                result.push(Datum::List(cur));

                for i in 0..len {
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

fn map(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let fun = vs[0].take();
    let mut lists = Vec::new();
    let mut len = 0;

    for i in 1..vs.len() {
        match vs[i].take() {
            Datum::List(elems) => {
                if i == 1 {
                    len = elems.len()
                } else {
                    if elems.len() != len {
                        panic!("All lists passed to map must have the same length");
                    }
                }
                lists.push(elems);
            },
            Datum::Nil => {
                if len != 0 {
                    panic!("All lists passed to map must have the same length");
                }
                return Ok(Datum::Nil);
            }
            _ => return Err(InvalidTypeOfArguments)
        }
    }

    let mut result = Vec::new();
    for i in 0..len {
        let args = lists.iter_mut().map(|l| l[i].take() ).collect();
        result.push(eval.full_apply(fun.clone(), args, env_ref.clone()))
    }

    Ok(Datum::List(result))
}

fn flatmap(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let fun = vs[0].take();
    let mut lists = Vec::new();
    let mut len = 0;

    for i in 1..vs.len() {
        match vs[i].take() {
            Datum::List(elems) => {
                if i == 1 {
                    len = elems.len()
                } else {
                    if elems.len() != len {
                        panic!("All lists passed to map must have the same length");
                    }
                }
                lists.push(elems);
            },
            Datum::Nil => {
                if len != 0 {
                    panic!("All lists passed to map must have the same length");
                }
                return Ok(Datum::Nil);
            }
            _ => return Err(InvalidTypeOfArguments)
        }
    }

    let mut result = Vec::new();
    for i in 0..len {
        let args = lists.iter_mut().map(|l| l[i].take() ).collect();
        let mut res = eval.full_apply(fun.clone(), args, env_ref.clone()).as_list()?;
        result.append(&mut res);
    }

    Ok(Datum::List(result))
}

fn count(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let pred = vs[0].take();
    let mut list = vs[1].take();

    match list {
        Datum::List(ref mut elems) => {
            let mut result = 0;

            for e in elems.iter_mut() {
                let res = eval.full_apply(pred.clone(), vec![e.take()], env_ref.clone());
                if res.is_true() {
                    result += 1
                }
            }
            Ok(Datum::Integer(result))
        },
        Datum::Nil => Ok(Datum::Integer(0)),
        _ => Err(InvalidTypeOfArguments)
    }
}

fn any(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let pred = vs[0].take();
    let mut list = vs[1].take();

    match list {
        Datum::List(ref mut elems) => {
            for e in elems.iter_mut() {
                let res = eval.full_apply(pred.clone(), vec![e.take()], env_ref.clone());
                if res.is_true() {
                    return Ok(Datum::Bool(true))
                }
            }
            Ok(Datum::Bool(false))
        },
        Datum::Nil => Ok(Datum::Bool(false)),
        _ => Err(InvalidTypeOfArguments)
    }
}

fn reduce(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let pred = vs[0].take();
    let mut result = vs[1].take();
    let mut list = vs[2].take();

    match list {
        Datum::List(ref mut elems) => {
            for e in elems.iter_mut() {
                result = eval.full_apply(pred.clone(), vec![e.take(), result], env_ref.clone());
            }
            Ok(result)
        },
        Datum::Nil => Ok(result),
        other => panic!("Reduce expected a list or nil but got {}", other)
    }
}

fn uniq(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut list = vs[0].take().as_list()?;
    list.dedup();
    Ok(Datum::List(list))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "cons", cons, Arity::Exact(2));
    register(hm, "pair", pair, Arity::Exact(2));
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
    register(hm, "map", map, Arity::Min(2));
    register(hm, "flatmap", flatmap, Arity::Min(2));
    register(hm, "reduce", reduce, Arity::Exact(3));
    register(hm, "count", count, Arity::Exact(2));
    register(hm, "any?", any, Arity::Exact(2));
    register(hm, "uniq", uniq, Arity::Exact(1));
}
