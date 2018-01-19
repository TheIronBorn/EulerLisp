use std::collections::HashMap;

use ::LispFn;
use ::Datum;
use ::LispErr::*;
use ::LispResult;
use ::Arity;

use ::bignum;
use ::builtin::register;
use ::bignum::Bignum;
use ::eval::Evaluator;
use ::EnvRef;

fn bg_add(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::Bignum(a) => {
            match vs[1].take() {
                Datum::Bignum(b) => Ok(Datum::Bignum(a + b)),
                other => Err(TypeError("bignum+, second argument", "bignum", other))
            }
        },
        other => Err(TypeError("bignum+, first argument", "bignum", other))
    }
}

fn bg_mul(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    match vs[0].take() {
        Datum::Bignum(a) => {
            match vs[1].take() {
                Datum::Bignum(b) => Ok(Datum::Bignum(a * b)),
                other => Err(TypeError("bignum*, second argument", "bignum", other))
            }
        },
        other => Err(TypeError("bignum*, first argument", "bignum", other))
    }
}

fn number_to_bignum(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::Bignum(Bignum::new(a)))
    }
    Err(TypeError("number->bignum", "integer", vs[0].take()))
}

fn bignum_num_digits(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        return Ok(Datum::Integer(a.num_digits()))
    }
    Err(TypeError("bignum-num-digits", "bignum", vs[0].take()))
}

fn bignum_digits(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        let digits = a.digits();
        return Ok(Datum::List(
                digits.into_iter().map(|d| Datum::Integer(d)).collect()
        ));
    }
    Err(TypeError("bignum-digits", "bignum", vs[0].take()))
}

fn bignum_from_digits(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::List(ref digits) = vs[0] {
        let mut chunks = Vec::new();
        let mut pow = 1;
        let mut result = 0;

        for digit in digits {
            if let Datum::Integer(n) = *digit {
                result += n * pow;
                pow *= 10;
            } else {
                return Err(TypeError("digits->bignum, in the list", "integer", digit.clone()))
            }

            if pow == (bignum::CHUNK as isize) {
                pow = 1;
                chunks.push(result as usize);
                result = 0;
            }
        }
        if result != 0 {
            chunks.push(result as usize);
        }

        Ok(Datum::Bignum(
            Bignum::from_chunks(chunks)
        ))
    } else {
        return Err(TypeError("digits->bignum", "list", vs[0].take()))
    }
}

fn bignum_chunks(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        let digits = a.chunks();
        return Ok(Datum::List(
                digits.into_iter().map(|d| Datum::Integer(d)).collect()
        ));
    }
    return Err(TypeError("bignum-chunks", "bignum", vs[0].take()))
}

fn bignum_from_chunks(vs: &mut [Datum], eval: &mut Evaluator, env_ref: EnvRef) -> LispResult {
    if let Datum::List(chunks) = vs[0].take() {
        let mut result = Vec::new();

        for chunk in chunks {
            if let Datum::Integer(n) = chunk {
                result.push(n as usize);
            } else {
                return Err(TypeError("chunks->bignum, in the list", "integer", chunk.clone()))
            }
        }

        Ok(Datum::Bignum(
            Bignum::from_chunks(result)
        ))
    } else {
        Err(TypeError("chunks->bignum", "list", vs[0].take()))
    }
}

pub fn load(hm: &mut HashMap<String, LispFn>) {
    register(hm, "bignum+", bg_add, Arity::Exact(2));
    register(hm, "bignum*", bg_mul, Arity::Exact(2));
    register(hm, "bignum", number_to_bignum, Arity::Exact(1));
    register(hm, "bignum-num-digits", bignum_num_digits, Arity::Exact(1));
    register(hm, "bignum-digits", bignum_digits, Arity::Exact(1));
    register(hm, "digits->bignum", bignum_from_digits, Arity::Exact(1));
    register(hm, "bignum-chunks", bignum_chunks, Arity::Exact(1));
    register(hm, "chunks->bignum", bignum_from_chunks, Arity::Exact(1));
}
