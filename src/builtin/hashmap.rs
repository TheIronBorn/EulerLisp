use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::eval::Evaluator;
use ::EnvRef;
use ::builtin::register;

fn hashmap(vs: &mut[Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let keys = vs[0].clone().as_list()?;
    let values = vs[1].clone().as_list()?;

    let mut res: HashMap<Datum, Datum> = HashMap::new();

    for (k, v) in keys.into_iter().zip(values.into_iter()) {
        res.insert(k, v);
    }

    Ok(Datum::HashMap(eval.get_unique_id(), res))
}

fn get(vs: &mut[Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let key = vs[0].clone();
    let map = vs[1].clone().as_hashmap()?;

    match map.get(&key) {
        Some(v) => Ok(v.clone()),
        None => Ok(Datum::Undefined)
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "hashmap", hashmap, Arity::Exact(2));
    register(hm, "hashmap-get", get, Arity::Exact(2));
}
