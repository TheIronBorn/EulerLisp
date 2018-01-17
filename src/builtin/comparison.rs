use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;

use ::builtin::register;

fn eq(vs: &mut [Datum]) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] != vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn neq(vs: &mut [Datum]) -> LispResult {
    Ok(Datum::Bool(vs[0] != vs[1]))
}

fn lt(vs: &mut [Datum]) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] >= vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn gt(vs: &mut [Datum]) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] <= vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn lte(vs: &mut [Datum]) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] > vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn gte(vs: &mut [Datum]) -> LispResult {
    for i in 0..(vs.len() - 1) {
        if vs[i] < vs[i+1] {
            return Ok(Datum::Bool(false));
        }
    }
    Ok(Datum::Bool(true))
}

fn max(vs: &mut [Datum]) -> LispResult {
    let mut max = vs[0].take();
    for v in vs.into_iter().skip(1) {
        if *v > max {
            max = v.take();
        }
    }
    Ok(max)
}

fn min(vs: &mut [Datum]) -> LispResult {
    let mut min = vs[0].take();
    for v in vs.into_iter().skip(1) {
        if *v < min {
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
