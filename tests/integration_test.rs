extern crate lisp;

use lisp::eval::Evaluator;
use lisp::Value::*;

#[test]
fn definitions() {
    let mut ev = Evaluator::new();
    ev.eval_str("(def a 1)");
    ev.eval_str("(def b 2)");
    assert_eq!(
        ev.eval_str("a"),
        Ok(Number(1))
    );
    assert_eq!(
        ev.eval_str("b"),
        Ok(Number(2))
    );
}

#[test]
fn redefinitions() {
    let mut ev = Evaluator::new();
    ev.eval_str("(def a 1)");
    ev.eval_str("(set! a 2)");
    assert_eq!(
        ev.eval_str("a"),
        Ok(Number(2))
    );
}

#[test]
fn builtin_read() {
    let mut ev = Evaluator::new();
    assert_eq!(
        ev.eval_str("(read \"1\")"),
        Ok(Number(1))
    );
}

#[test]
fn builtin_eval() {
    let mut ev = Evaluator::new();
    assert_eq!(
        ev.eval_str("(eval '(+ 1 2 3))"),
        Ok(Number(6))
    );
    assert_eq!(
        ev.eval_str("(eval (read \"(+ 1 2 3)\"))"),
        Ok(Number(6))
    );
}
