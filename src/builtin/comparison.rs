use std::collections::HashMap;
use std::cmp::Ordering;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;

use ::eval::Evaluator;
use ::EnvRef;
use ::builtin::register;

fn eq(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] != vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn neq(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    Ok(Datum::Bool(vs[0] != vs[1]))
}

fn lt(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i].compare(&vs[i+1]).unwrap() != Ordering::Less {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn gt(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i].compare(&vs[i+1]).unwrap() != Ordering::Greater {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn lte(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i+1].compare(&vs[i]).unwrap() == Ordering::Less {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn gte(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i+1].compare(&vs[i]).unwrap() == Ordering::Greater {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn max(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let mut max = vs[0].take();
    for v in vs.into_iter().skip(1) {
        if max.compare(v).unwrap() == Ordering::Less {
            max = v.take();
        }
    }
    Ok(max)
}

fn min(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let mut min = vs[0].take();
    for v in vs.into_iter().skip(1) {
        if min.compare(v).unwrap() == Ordering::Greater {
            min = v.take();
        }
    }
    Ok(min)
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "=", eq, Arity::Min(2));
    register(hm, "!=", neq, Arity::Exact(2));
    register(hm, "<", lt, Arity::Min(2));
    register(hm, ">", gt, Arity::Min(2));
    register(hm, "<=", lte, Arity::Min(2));
    register(hm, ">=", gte, Arity::Min(2));
    register(hm, "max", max, Arity::Min(2));
    register(hm, "min", min, Arity::Min(2));
}
