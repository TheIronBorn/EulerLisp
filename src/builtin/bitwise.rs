use std::collections::HashMap;

use LispFn;
use Datum;
use LispResult;
use Arity;
use builtin::register;

use ::eval::Evaluator;
use ::EnvRef;

fn bitwise_and(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].as_integer()?;
    for v in &mut vs[1..] {
        res = res & v.as_integer()?;
    }
    Ok(Datum::Integer(res))
}

fn bitwise_or(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].as_integer()?;
    for v in &mut vs[1..] {
        res = res | v.as_integer()?;
    }
    Ok(Datum::Integer(res))
}

fn bitwise_xor(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut res = vs[0].as_integer()?;
    for v in &mut vs[1..] {
        res = res ^ v.as_integer()?;
    }
    Ok(Datum::Integer(res))
}

fn bitwise_not(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let res = vs[0].as_integer()?;
    Ok(Datum::Integer(!res))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "bitwise-and", bitwise_and, Arity::Min(2));
    register(hm, "bitwise-or", bitwise_or, Arity::Min(2));
    register(hm, "bitwise-xor", bitwise_xor, Arity::Min(2));
    register(hm, "bitwise-not", bitwise_not, Arity::Exact(1));
}
