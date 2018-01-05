use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::builtin::register;
use ::bignum::Bignum;

fn bg_add(vs: &mut [Datum]) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        if let Datum::Bignum(b) = vs[1].take() {
            return Ok(Datum::Bignum(a + b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn bg_mul(vs: &mut [Datum]) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        if let Datum::Bignum(b) = vs[1].take() {
            return Ok(Datum::Bignum(a * b));
        }
    }
    Err(InvalidTypeOfArguments)
}

fn number_to_bignum(vs: &mut [Datum]) -> LispResult {
    if let Datum::Number(a) = vs[0] {
        return Ok(Datum::Bignum(Bignum::new(a)))
    }
    Err(InvalidTypeOfArguments)
}

fn bignum_num_digits(vs: &mut [Datum]) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        return Ok(Datum::Number(a.num_digits()))
    }
    Err(InvalidTypeOfArguments)
}

fn bignum_digits(vs: &mut [Datum]) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        let digits = a.digits();
        return Ok(Datum::List(
                digits.into_iter().map(|d| Datum::Number(d)).collect()
        ));
    }
    Err(InvalidTypeOfArguments)
}
// fn bg_subtract(vs: &mut [Datum]) -> LispResult {
//     if let Datum::Number(a) = vs[0] {
//         if let Datum::Number(b) = vs[1] {
//             return Ok(Datum::Number(a - b));
//         }
//     }
//     Err(InvalidTypeOfArguments)
// }

// fn bg_mult(vs: &mut [Datum]) -> LispResult {
//     if let Datum::Number(a) = vs[0] {
//         if let Datum::Number(b) = vs[1] {
//             return Ok(Datum::Number(a * b));
//         }
//     }
//     Err(InvalidTypeOfArguments)
// }

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "bignum+", bg_add, Arity::Exact(2));
    register(hm, "bignum*", bg_mul, Arity::Exact(2));
    // register(hm, "bg-", bg_subtract, Arity::Exact(2));
    register(hm, "bignum", number_to_bignum, Arity::Exact(1));
    register(hm, "bignum-num-digits", bignum_num_digits, Arity::Exact(1));
    register(hm, "bignum-digits", bignum_digits, Arity::Exact(1));
}
