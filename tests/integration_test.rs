extern crate lisp;

use lisp::eval::Evaluator;
use lisp::Value::*;

#[test]
fn definitions() {
    let mut ev = Evaluator::new();
    let main_env = ev.make_env(None);

    ev.eval_str("(def a 1)", main_env);
    ev.eval_str("(def b 2)", main_env);
    assert_eq!(
        ev.eval_str("a", main_env),
        Ok(Number(1))
    );
    assert_eq!(
        ev.eval_str("b", main_env),
        Ok(Number(2))
    );
}

#[test]
fn redefinitions() {
    let mut ev = Evaluator::new();
    let main_env = ev.make_env(None);

    ev.eval_str("(def a 1)", main_env);
    ev.eval_str("(set! a 2)", main_env);
    assert_eq!(
        ev.eval_str("a", main_env),
        Ok(Number(2))
    );
}

#[test]
fn builtin_read() {
    let mut ev = Evaluator::new();
    let main_env = ev.make_env(None);
    assert_eq!(
        ev.eval_str("(read \"1\")", main_env),
        Ok(Number(1))
    );
}

#[test]
fn builtin_eval() {
    let mut ev = Evaluator::new();
    let main_env = ev.make_env(None);
    assert_eq!(
        ev.eval_str("(eval '(+ 1 2 3))", main_env),
        Ok(Number(6))
    );
    assert_eq!(
        ev.eval_str("(eval (read \"(+ 1 2 3)\"))", main_env),
        Ok(Number(6))
    );
}

#[test]
fn recursion_test() {
    let mut ev = Evaluator::new();
    let main_env = ev.make_env(None);
    ev.eval_str("(defn fac (n) (if (= n 0) 1 (* n (fac (- n 1)))))", main_env);
    assert_eq!(
        ev.eval_str("(fac 3)", main_env),
        Ok(Number(6))
    )
}
