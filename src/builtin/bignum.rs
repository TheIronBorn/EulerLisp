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

fn number_to_bignum(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Integer(a) = vs[0] {
        return Ok(Datum::Bignum(Bignum::new(a)))
    }
    Err(TypeError("number->bignum", "integer", vs[0].take()))
}

fn bignum_from_digits(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Pair(ref ptr) = vs[0] {
        let digits = ptr.borrow().collect_list()?;
        let mut chunks = Vec::new();
        let mut pow = 1;
        let mut result = 0;

        for digit in digits {
            if let Datum::Integer(n) = digit {
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

fn bignum_chunks(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Bignum(a) = vs[0].take() {
        let digits = a.chunks();
        return Ok(Datum::make_list_from_vec(
            digits.into_iter().map(|d| Datum::Integer(d)).collect()
        ));
    }
    return Err(TypeError("bignum-chunks", "bignum", vs[0].take()))
}

fn bignum_from_chunks(vs: &mut [Datum], _eval: &mut Evaluator, _env_ref: EnvRef) -> LispResult {
    if let Datum::Pair(ptr) = vs[0].take() {
        let chunks = ptr.borrow().collect_list()?;
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
    register(hm, "bignum", number_to_bignum, Arity::Exact(1));
    register(hm, "digits->bignum", bignum_from_digits, Arity::Exact(1));
    register(hm, "bignum-chunks", bignum_chunks, Arity::Exact(1));
    register(hm, "chunks->bignum", bignum_from_chunks, Arity::Exact(1));
}
