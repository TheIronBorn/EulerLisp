use std::collections::HashMap;

use ::Value;
use ::Value::Builtin;
use ::LispFn;
use ::LispResult;
use ::LispErr::*;

use std::rc::Rc;

// The difference between builtins and special forms is,
// that special forms choose if they want to eval their arguments themselves,
// builtins are called with evaluated arguments


fn register(hm: &mut HashMap<String, Value>, name: &str,
            f: Rc<Fn(Vec<Value>)->LispResult>) {
    hm.insert(name.to_string(), Builtin(LispFn(f)));
}

pub fn load(hm: &mut HashMap<String, Value>) {
    register(hm, "pair?", Rc::new(|vs| Ok(Value::Bool(vs[0].is_pair()))));
    register(hm, "list?", Rc::new(|vs| Ok(Value::Bool(vs[0].is_list()))));
    register(hm, "nil?", Rc::new(|vs| Ok(Value::Bool(vs[0].is_nil()))));
    register(hm, "=", Rc::new(|vs| Ok(Value::Bool(vs[0] == vs[1]))));
    // TODO: What should happen when compairing two different types?
    register(hm, ">", Rc::new(|vs| Ok(Value::Bool(vs[0] > vs[1]))));
    register(hm, "<", Rc::new(|vs| Ok(Value::Bool(vs[0] < vs[1]))));
    register(hm, ">=", Rc::new(|vs| Ok(Value::Bool(vs[0] >= vs[1]))));
    register(hm, "<=", Rc::new(|vs| Ok(Value::Bool(vs[0] <= vs[1]))));
    register(hm, "puts", Rc::new(|vs| {
        match vs[0] {
            // Print string without " around them
            Value::Str(ref x) => print!("{}\n", x),
            ref other => println!("{}", other),
        };
        Ok(Value::Undefined)
    }));
    register(hm, "inspect", Rc::new(|vs| {
        println!("{:?}", vs[0]);
        Ok(Value::Undefined)
    }));
    register(hm, "+", Rc::new(|vs| {
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a + b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "*", Rc::new(|vs| {
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a * b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "/", Rc::new(|vs| {
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a / b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "%", Rc::new(|vs| {
        if let Value::Number(a) = vs[0] {
            if let Value::Number(b) = vs[1] {
                return Ok(Value::Number(a % b));
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "-", Rc::new(|vs| {
        if let Value::Number(a) = vs[0] {
            if vs.len() == 2 {
                if let Value::Number(b) = vs[1] {
                    return Ok(Value::Number(a - b));
                } else {
                    return Ok(Value::Number(-a));
                }
            }
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "not", Rc::new(|vs| {
        if let Value::Bool(b) = vs[0] {
            return Ok(Value::Bool(!b));
        }
        Err(InvalidTypeOfArguments)
    }));
    register(hm, "cons", Rc::new(|vs| {
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
