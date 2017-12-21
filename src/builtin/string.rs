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
}
