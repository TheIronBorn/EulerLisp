use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use ::Datum;
use ::LispFn;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;
use ::eval::Evaluator;
use ::EnvRef;

fn println(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    let mut output = eval.output.borrow_mut();
    for v in vs.iter() {
        match *v {
            Datum::String(ref x) => {
                if let Err(_err) = write!(output, "{}", x) {
                    return Err(IOError)
                }
            },
            ref other => {
                if let Err(_err) = write!(output, "{}", other.to_string(&mut eval.symbol_table)) {
                    return Err(IOError)
                }
            }
        };
    }
    if let Err(_err) = write!(output, "\n") {
        return Err(IOError)
    }
    Ok(Datum::Undefined)
}

fn inspect(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    match writeln!(eval.output.borrow_mut(), "{:?}", vs[0]) {
        Err(_err) => Err(IOError),
        Ok(_) => Ok(Datum::Undefined)
    }
}

fn not(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Bool(b) = vs[0] {
        return Ok(Datum::Bool(!b));
    }
    Err(InvalidTypeOfArguments)
}

fn file_read(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::String(ref b) = vs[0] {
        match File::open(b) {
            Ok(ref mut file) => {
                let mut result = String::new();
                match file.read_to_string(&mut result) {
                    Ok(_) => return Ok(Datum::String(result)),
                    Err(_) => return Err(IOError),
                };
            },
            Err(_) => return Err(IOError)
        }
    }
    Err(InvalidTypeOfArguments)
}

fn apply(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    let f = vs[0].clone();
    let args = vs[1].clone().as_list()?;
    Ok(eval.full_apply(f, args, env_ref))
}

// fn read(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
//     let arg = vs.get(0).unwrap();
//     if let Datum::String(ref input) = *arg {
//         let result = parser::parse_datum(input.as_ref());
//         Ok(result)
//     } else {
//         Err(InvalidTypeOfArguments)
//     }
// }

// TODO: Fix the way environments are handled here
// fn eval(vs: &mut [Datum], eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
//     let env = eval.root_env.clone();
//     eval.eval_datum(vs[0].clone(), env)
// }

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "println", println, Arity::Min(0));
    register(hm, "inspect", inspect, Arity::Exact(1));
    register(hm, "not", not, Arity::Exact(1));
    register(hm, "file-read", file_read, Arity::Exact(1));
    register(hm, "apply", apply, Arity::Exact(2));
    // register(hm, "read", read, Arity::Exact(1));
    // register(hm, "eval", eval, Arity::Exact(1));
}
