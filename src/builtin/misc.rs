use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use ::Datum;
use ::LispFn;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;

fn println(vs: &mut [Datum]) -> LispResult {
    for v in vs.iter() {
        match *v {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}", x),
            ref other => print!("{}", other),
        };
    }
    print!("\n");
    Ok(Datum::Undefined)
}

fn print(vs: &mut [Datum]) -> LispResult {
    for v in vs.iter() {
        match *v {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}", x),
            ref other => print!("{}", other),
        };
    }
    print!("\n");
    Ok(Datum::Undefined)
}

fn inspect(vs: &mut [Datum]) -> LispResult {
    println!("{:?}", vs[0]);
    Ok(Datum::Undefined)
}

fn not(vs: &mut [Datum]) -> LispResult {
    if let Datum::Bool(b) = vs[0] {
        return Ok(Datum::Bool(!b));
    }
    Err(InvalidTypeOfArguments)
}

fn file_read(vs: &mut [Datum]) -> LispResult {
    if let Datum::Str(ref b) = vs[0] {
        match File::open(b) {
            Ok(ref mut file) => {
                let mut result = String::new();
                match file.read_to_string(&mut result) {
                    Ok(_) => return Ok(Datum::Str(result)),
                    Err(_) => return Err(IOError),
                };
            },
            Err(_) => return Err(IOError)
        }
    }
    Err(InvalidTypeOfArguments)
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "println", println, Arity::Min(0));
    register(hm, "print", print, Arity::Min(0));
    register(hm, "inspect", inspect, Arity::Exact(1));
    register(hm, "not", not, Arity::Exact(1));
    register(hm, "file-read", file_read, Arity::Exact(1));
}
