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
    let fst = vs[0].clone();
    let rst = vs[1].clone();
    Ok(Datum::make_pair(fst, rst))
}

fn fst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(vs[0].as_pair()?.0.clone())
}

fn rst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(vs[0].as_pair()?.1.clone())
}

fn set_fst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    vs[0].as_mut_pair()?.0 = vs[1].clone();
    Ok(Datum::Undefined)
}

fn set_rst(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    vs[0].as_mut_pair()?.1 = vs[1].clone();
    Ok(Datum::Undefined)
}

fn list(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(Datum::make_list(vs))
}

fn vector(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    Ok(Datum::make_vector(vs))
}

fn make_vector(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let len = vs[0].as_uinteger()?;
    let default = if vs.len() == 2 {
        vs[1].clone()
    } else {
        Datum::Undefined
    };
    let vector = vec![default; len as usize];
    Ok(Datum::make_vector_from_vec(vector))
}

fn sort(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].clone() {
        Datum::Pair(ptr) => {
            let mut elems = ptr.borrow().collect_list()?;
            let mut es = elems.as_mut_slice();
            let len = es.len();
            quicksort_helper(&mut es, 0, (len - 1) as isize)?;
            Ok(Datum::make_list(es))
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

// Heap's algorithm
fn permutations(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut elems = vs[0].as_pair()?.collect_list()?;
    let mut result: Vec<Datum> = Vec::new();

    let n = elems.len();
    let mut c = vec![0; n]; 

    result.push(Datum::make_list_from_vec(elems.clone()));
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                elems.swap(0, i);
            } else {
                elems.swap(c[i], i);
            }
            result.push(Datum::make_list_from_vec(elems.clone()));
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    Ok(Datum::make_list_from_vec(result))
}

fn combinations(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let len = vs[0].as_uinteger()?;
    let elems = vs[1].as_pair()?.collect_list()?;

    let max = elems.len();
    let mut counters = vec![0; len];
    let mut result: Vec<Datum> = Vec::new();
    let mut done = false;

    while !done {
        let cur : Vec<Datum> = counters.iter().map(|c| elems[*c].clone()).collect();
        result.push(Datum::make_list_from_vec(cur));

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

    Ok(Datum::make_list_from_vec(result))
}

fn uniq(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match vs[0].clone() {
        Datum::Pair(ptr) => {
            let mut elems = ptr.borrow().collect_list()?;
            elems.dedup();
            Ok(Datum::make_list_from_vec(elems))
        },
        Datum::Nil => Ok(Datum::Nil),
        _ => Err(InvalidTypeOfArguments),
    }
}

fn vector_ref(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let vector = vs[0].as_vector()?;
    match vector.get(vs[1].as_uinteger()?) {
        Some(e) => Ok(e.clone()),
        None => Err(IndexOutOfBounds)
    }
}

fn vector_set(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut vector = vs[0].as_mut_vector()?;
    let index = vs[1].as_uinteger()?;
    if index > 0 && index < vector.len() {
        vector[index] = vs[2].clone();
        Ok(Datum::Undefined)
    } else {
        Err(IndexOutOfBounds)
    }
}

fn vector_push(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut vector = vs[0].as_mut_vector()?;
    vector.push(vs[1].clone());
    Ok(Datum::Undefined)
}

fn vector_length(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let vector = vs[0].as_vector()?;
    Ok(Datum::from(&vector.len()))
}

fn list_to_vector(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let pair = vs[0].as_pair()?;
    let elems = pair.collect_list()?;
    Ok(Datum::make_vector_from_vec(elems))
}

fn vector_to_list(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let vector = vs[0].as_vector()?;
    Ok(Datum::make_list_from_vec(vector.clone()))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "cons", cons, Arity::Exact(2));
    register(hm, "pair", cons, Arity::Exact(2));
    register(hm, "fst", fst, Arity::Exact(1));
    register(hm, "rst", rst, Arity::Exact(1));
    register(hm, "set-fst!", set_fst, Arity::Exact(2));
    register(hm, "set-rst!", set_rst, Arity::Exact(2));
    register(hm, "vector-ref", vector_ref, Arity::Exact(2));
    register(hm, "vector-set!", vector_set, Arity::Exact(3));
    register(hm, "vector-push!", vector_push, Arity::Exact(2));
    register(hm, "vector-length", vector_length, Arity::Exact(1));
    register(hm, "list->vector", list_to_vector, Arity::Exact(1));
    register(hm, "vector->list", vector_to_list, Arity::Exact(1));
    register(hm, "list", list, Arity::Min(0));
    register(hm, "vector", vector, Arity::Min(0));
    register(hm, "make-vector", make_vector, Arity::Range(1, 2));
    register(hm, "sort", sort, Arity::Exact(1));
    register(hm, "permutations", permutations, Arity::Exact(1));
    register(hm, "combinations", combinations, Arity::Exact(2));
    register(hm, "uniq", uniq, Arity::Exact(1));
}
