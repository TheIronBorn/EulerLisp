use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "puts", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Value::Str(ref x) => print!("{}\n", x),
            ref other => println!("{}", other),
        };
        Ok(Value::Undefined)
    }));
    register(hm, "print", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Value::Str(ref x) => print!("{}", x),
            ref other => print!("{}", other),
        };
        Ok(Value::Undefined)
    }));
    register(hm, "inspect", Rc::new(|vs| {
        check_arity!(vs, 1);
        println!("{:?}", vs[0]);
        Ok(Value::Undefined)
    }));
    register(hm, "not", Rc::new(|vs| {
        check_arity!(vs, 1);
        if let Value::Bool(b) = vs[0] {
            return Ok(Value::Bool(!b));
        }
        Err(InvalidTypeOfArguments)
    }));
}
