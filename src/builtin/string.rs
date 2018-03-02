use std::collections::HashMap;

use ::Datum;
use ::LispFn;
use ::LispErr;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;
use ::eval::Evaluator;
use ::EnvRef;

fn string_bytes(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].as_string()?;
    let bytes = string.as_bytes().iter().map(|b| Datum::from(b)).collect();
    Ok(Datum::make_list_from_vec(bytes))
}

fn string_length(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].as_string()?;
    Ok(Datum::Integer(string.len() as isize))
}

fn string_to_number(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].as_string()?;
    match string.parse::<isize>() {
        Ok(i) => Ok(Datum::Integer(i)),
        Err(_) => Err(InvalidTypeOfArguments)
    }
}

fn string_split(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[1].as_string()?;
    let splitter = vs[0].as_string()?;
    let lines: Vec<Datum> =
        string.split(&splitter)
        .map( |l| Datum::String(l.to_string()) )
        .collect();

    Ok(Datum::make_list_from_vec(lines))
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

fn string_trim(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].as_string()?;
    Ok(Datum::String(string.trim().to_string()))
}

fn string_to_chars(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let string = vs[0].as_string()?;
    Ok(Datum::make_list_from_vec(string.chars().map(|c| Datum::Char(c) ).collect()))
}

fn chars_to_string(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let pair = vs[0].as_pair()?;
    let chars = pair.collect_list()?;

    let s: Result<String, LispErr> = chars.into_iter().map(|c| c.as_char()).collect();
    Ok(Datum::String(s?))
}

fn char_to_integer(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Integer(c as isize))
}

fn char_to_digit(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;

    if c.is_ascii_digit() {
        // 48 is ASCII of 0
        Ok(Datum::Integer(c as isize - 48))
    } else {
        Err(InvalidNumberOfArguments)
    }
}

fn char_is_numeric(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Bool(c.is_ascii_digit()))
}

fn char_is_alphabetic(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Bool(c.is_ascii_alphabetic()))
}

fn char_is_whitespace(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Bool(c.is_ascii_whitespace()))
}

fn char_is_upper_case(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Bool(c.is_ascii_uppercase()))
}

fn char_is_lower_case(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let c = vs[0].as_char()?;
    Ok(Datum::Bool(c.is_ascii_lowercase()))
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "string-bytes", string_bytes, Arity::Exact(1));
    register(hm, "string-length", string_length, Arity::Exact(1));
    register(hm, "string-trim", string_trim, Arity::Exact(1));
    register(hm, "string->number", string_to_number, Arity::Exact(1));
    register(hm, "string->chars", string_to_chars, Arity::Exact(1));
    register(hm, "chars->string", chars_to_string, Arity::Exact(1));
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
