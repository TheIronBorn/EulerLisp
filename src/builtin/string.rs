use std::collections::HashMap;

use ::Datum;
use ::LispErr::*;
use ::LispResult;

use ::builtin::register;

fn string_bytes(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    if let Datum::Str(ref string) = vs[0] {
        let bytes = string.as_bytes().iter().map(
            |b| Datum::Number(*b as i64)
            ).collect();
        return Ok(Datum::List(bytes));
    }
    Err(InvalidTypeOfArguments)
}

fn string_length(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    if let Datum::Str(ref string) = vs[0] {
        return Ok(Datum::Number(string.len() as i64));
    }
    Err(InvalidTypeOfArguments)
}

fn string_to_number(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 1);
    if let Datum::Str(ref string) = vs[0] {
        match string.parse::<i64>() {
            Ok(i) => {
                return Ok(Datum::Number(i));
            },
            Err(_) => {
                return Err(InvalidTypeOfArguments)
            }
        }
    }
    Err(InvalidTypeOfArguments)
}

fn string_split(vs: Vec<Datum>) -> LispResult {
    check_arity!(vs, 2);
    if let Datum::Str(ref string) = vs[0] {
        if let Datum::Str(ref splitter) = vs[1] {
            let lines: Vec<Datum> =
                string.split(splitter)
                .map( |l| Datum::Str(l.to_string()) )
                .collect();
            return Ok(Datum::List(lines));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn string_join(vs: Vec<Datum>) -> LispResult {
    let mut result = String::new();

    for v in vs.into_iter() {
        match v {
            Datum::Str(s) => result += &s,
            other => result += &other.to_string(),
        }
    }
    return Ok(Datum::Str(result));
}

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "string-bytes", string_bytes);
    register(hm, "string-length", string_length);
    register(hm, "string->number", string_to_number);
    register(hm, "string-split", string_split);
    register(hm, "str", string_join);
}
