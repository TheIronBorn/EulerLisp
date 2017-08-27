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
        Number(1)
    );
    assert_eq!(
        ev.eval_str("b"),
        Number(2)
    );
}

#[test]
fn redefinitions() {
    let mut ev = Evaluator::new();
    ev.eval_str("(def a 1)");
    ev.eval_str("(set! a 2)");
    assert_eq!(
        ev.eval_str("a"),
        Number(2)
    );
}
