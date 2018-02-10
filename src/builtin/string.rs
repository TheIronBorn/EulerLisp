use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;
use ::eval::Evaluator;
use ::EnvRef;

fn string_bytes(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::String(ref string) = vs[0] {
        let bytes = string.as_bytes().iter().map(
            |b| Datum::Integer(*b as isize)
            ).collect();
        return Ok(Datum::make_list_from_vec(bytes));
    }
    Err(InvalidTypeOfArguments)
}

fn string_length(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::String(ref string) = vs[0] {
        return Ok(Datum::Integer(string.len() as isize));
    }
    Err(InvalidTypeOfArguments)
}

fn string_to_number(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::String(ref string) = vs[0] {
        match string.parse::<isize>() {
            Ok(i) => {
                return Ok(Datum::Integer(i));
            },
            Err(_) => {
                return Err(InvalidTypeOfArguments)
            }
        }
    }
    Err(InvalidTypeOfArguments)
}

fn string_split(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::String(ref splitter) = vs[0] {
        if let Datum::String(ref string) = vs[1] {
            let lines: Vec<Datum> =
                string.split(splitter)
                .map( |l| Datum::String(l.to_string()) )
                .collect();
            return Ok(Datum::make_list_from_vec(lines));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn string_join(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut result = String::new();

    for v in vs.into_iter() {
        match v {
            &mut Datum::String(ref s) => result += s,
            other => result += &other.to_string(&mut eval.symbol_table),
        }
    }
    return Ok(Datum::String(result));
}


fn string_to_chars(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].take().as_string().unwrap();

    Ok(Datum::make_list_from_vec(string.chars().map(|c| Datum::Char(c) ).collect()))
}

fn char_to_integer(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();
    Ok(Datum::Integer(c as isize))
}

fn char_to_digit(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();

    if c.is_ascii_digit() {
        // 48 is ASCII of 0
        Ok(Datum::Integer(c as isize - 48))
    } else {
        Err(InvalidNumberOfArguments)
    }
}

fn char_is_numeric(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();

    Ok(Datum::Bool(c.is_ascii_digit()))
}

fn char_is_alphabetic(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();
    Ok(Datum::Bool(c.is_ascii_alphabetic()))
}

fn char_is_whitespace(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();
    Ok(Datum::Bool(c.is_ascii_whitespace()))
}

fn char_is_upper_case(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();
    Ok(Datum::Bool(c.is_ascii_uppercase()))
}

fn char_is_lower_case(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].take().as_char().unwrap();
    Ok(Datum::Bool(c.is_ascii_lowercase()))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "string-bytes", string_bytes, Arity::Exact(1));
    register(hm, "string-length", string_length, Arity::Exact(1));
    register(hm, "string->number", string_to_number, Arity::Exact(1));
    register(hm, "string->chars", string_to_chars, Arity::Exact(1));
    register(hm, "string-split", string_split, Arity::Exact(2));
    register(hm, "str", string_join, Arity::Min(0));
    register(hm, "char-alphabetic?", char_is_alphabetic, Arity::Exact(1));
    register(hm, "char-numeric?", char_is_numeric, Arity::Exact(1));
    register(hm, "char-whitespace?", char_is_whitespace, Arity::Exact(1));
    register(hm, "char-upper-case?", char_is_upper_case, Arity::Exact(1));
    register(hm, "char-lower-case?", char_is_lower_case, Arity::Exact(1));
    register(hm, "char->integer", char_to_integer, Arity::Exact(1));
    register(hm, "char->digit", char_to_digit, Arity::Exact(1));
}
