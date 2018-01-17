use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispResult;
use ::Arity;

use ::eval::Evaluator;
use ::EnvRef;
use ::builtin::register;

fn pair_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    Ok(Datum::Bool(vs[0].is_pair()))
}

fn list_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Bool(_) = vs[0] {
        Ok(Datum::Bool(true))
    } else {
        Ok(Datum::Bool(false))
    }
}

fn nil_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Nil = vs[0] {
        Ok(Datum::Bool(true))
    } else {
        Ok(Datum::Bool(false))
    }
}

fn integer_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(_) = vs[0] {
        Ok(Datum::Bool(true))
    } else {
        Ok(Datum::Bool(false))
    }
}

fn rational_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Rational(_) = vs[0] {
        Ok(Datum::Bool(true))
    } else {
        Ok(Datum::Bool(false))
    }
}

fn bignum_questionmark(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(_) = vs[0] {
        Ok(Datum::Bool(true))
    } else {
        Ok(Datum::Bool(false))
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "pair?", pair_questionmark, Arity::Exact(1));
    register(hm, "list?", list_questionmark, Arity::Exact(1));
    register(hm, "nil?", nil_questionmark, Arity::Exact(1));
    register(hm, "integer?", integer_questionmark, Arity::Exact(1));
    register(hm, "rational?", rational_questionmark, Arity::Exact(1));
    register(hm, "bignum?", bignum_questionmark, Arity::Exact(1));
}
