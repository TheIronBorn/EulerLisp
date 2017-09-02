use std::collections::HashMap;
use std::rc::Rc;

use ::Value;
use ::LispErr::*;

use ::builtin::register;

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "cons", Rc::new(|vs| {
        check_arity!(vs, 2);
        // TODO: Can this be done without clone?
        let fst = vs[0].clone();
        let rst = vs[1].clone();

        match rst {
            Value::Nil => Ok(Value::List(vec![fst])),
            Value::DottedList(ref elems) => {
                let mut new = elems.clone();
                new.insert(0, fst);
                Ok(Value::DottedList(new))
            },
            Value::List(ref elems) => {
                let mut new = elems.clone();
                new.insert(0, fst);
                Ok(Value::List(new))
            },
            other => Ok(Value::DottedList(vec![fst, other])),
        }
    }));

    register(hm, "fst", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // TODO: find some way to ensure dotted list size >= 2
            Value::DottedList(ref elems) => {
                Ok(elems.first().unwrap().clone())
            },
            Value::List(ref elems) => {
                Ok(elems.first().unwrap().clone())
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));

    register(hm, "rst", Rc::new(|vs| {
        check_arity!(vs, 1);
        match vs[0] {
            // TODO: find some way to ensure dotted list size >= 2
            Value::DottedList(ref elems) => {
                if elems.len() == 2 {
                    Ok(elems.get(1).unwrap().clone())
                } else {
                    let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                    Ok(Value::DottedList(rest))
                }
            },
            Value::List(ref elems) => {
                if elems.len() == 1 {
                    Ok(Value::Nil)
                } else {
                    let rest: Vec<Value> = elems[1..].iter().map(|v| v.clone()).collect();
                    Ok(Value::List(rest))
                }
            },
            _ => Err(InvalidTypeOfArguments)
        }
    }));
}
