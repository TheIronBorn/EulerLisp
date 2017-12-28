use std::collections::HashMap;
use std::rc::Rc;

use ::Datum;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "string-bytes", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Str(ref string) = vs[0] {
            let bytes = string.as_bytes().iter().map(
                |b| Datum::Number(*b as i64)
            ).collect();
            return Ok(Datum::List(bytes));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "string-length", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Str(ref string) = vs[0] {
            return Ok(Datum::Number(string.len() as i64));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "string->number", Rc::new(|vs| {
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
    }));
    register(hm, "lines", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Str(ref string) = vs[0] {
            let lines: Vec<Datum> =
                string.split("\n")
                .map( |l| Datum::Str(l.to_string()) )
                .collect();
            return Ok(Datum::List(lines));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "words", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Datum::Str(ref string) = vs[0] {
            let lines: Vec<Datum> =
                string.split(" ")
                .map( |l| Datum::Str(l.to_string()) )
                .collect();
            return Ok(Datum::List(lines));
        }
        Err(InvalidTypeOfArguments)
    }));
}
