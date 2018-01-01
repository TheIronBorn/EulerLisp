use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use ::Datum;
use ::LispErr::*;
use ::LispResult;

use ::builtin::register;

fn println(vs: Vec<Datum>) -> LispResult {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}\n", x),
            ref other => println!("{}", other),
        };
        Ok(Datum::Undefined)
}

fn print(vs: Vec<Datum>) -> LispResult {
        check_arity!(vs, 1);
        match vs[0] {
            // Print string without " around them
            Datum::Str(ref x) => print!("{}", x),
            ref other => print!("{}", other),
        };
        Ok(Datum::Undefined)
}

fn inspect(vs: Vec<Datum>) -> LispResult {
        check_arity!(vs, 1);
        println!("{:?}", vs[0]);
        Ok(Datum::Undefined)
}

fn not(vs: Vec<Datum>) -> LispResult {
        check_arity!(vs, 1);
        if let Datum::Bool(b) = vs[0] {
            return Ok(Datum::Bool(!b));
        }
        Err(InvalidTypeOfArguments)
}

fn file_read(vs: Vec<Datum>) -> LispResult {
        check_arity!(vs, 1);
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

pub fn load(hm: &mut HashMap<String, Datum>) {
    register(hm, "println", println);
    register(hm, "print", print);
    register(hm, "inspect", inspect);
    register(hm, "not", not);
    register(hm, "file-read", file_read);
}
