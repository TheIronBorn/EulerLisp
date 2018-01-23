use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Datum;
use ::Expression;
use ::LispErr;
use ::LispFn;
use ::Symbol;
use ::LispErr::*;

use syntax_rule::SyntaxRule;
use symbol_table::SymbolTable;
use env::{AEnv, AEnvRef};

fn process_params(params: &Vec<Datum>, symbol_table: &mut SymbolTable) -> (Vec<Symbol>, Vec<Datum>) {
    let mut names = Vec::new();
    let mut defaults = Vec::new();
    let mut had_default = false;

    for param in params {
        match *param {
            Datum::Symbol(ref v) => {
                if had_default {
                    panic!("All params after one with a default must have defaults");
                }
                names.push(symbol_table.insert(v));
            }, 
            Datum::List(ref elems) => {
                let name = elems.get(0).unwrap();
                let default = elems.get(1).unwrap_or(&Datum::Nil);
                had_default = true;

                if let Datum::Symbol(ref v) = *name {
                    names.push(symbol_table.insert(v));
                } else {
                    panic!("Function parameters with defaults must have the form (name default)");
                }

                defaults.push(default.clone());
            }, 
            _ => panic!("Function parameters must have the form `name` or `(name default)`")
        }
    }

    (names, defaults)
}

fn preprocess_list(
    bodies: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<String, SyntaxRule>,
    env_ref: AEnvRef
) -> Vec<Expression> {
    bodies.into_iter().map(
        |b| preprocess(b.clone(), symbol_table, builtins, syntax_rules, env_ref.clone()).unwrap()
    ).collect()
}

fn preprocess_sequence(
    bodies: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<String, SyntaxRule>,
    env_ref: AEnvRef
) -> Expression {
    if bodies.len() == 0 {
        return Expression::datum_nil();
    }

    let mut exprs = preprocess_list(bodies, symbol_table, builtins, syntax_rules, env_ref.clone());
    if exprs.len() > 1 {
        let last = exprs.pop().unwrap();
        Expression::Do(exprs, Box::new(last))
    } else {
        exprs.pop().unwrap()
    }
}

fn preprocess_fn(
    mut elems: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<String, SyntaxRule>,
    env_ref: AEnvRef
) -> Result<Expression, LispErr> {
    if elems.len() < 2 {
        return Err(InvalidNumberOfArguments);
    }

    let mut names;
    let defaults;
    let dotted;

    let params = elems.remove(0);
    match params {
        Datum::List(ref elems) => {
            let res = process_params(elems, symbol_table);
            names = res.0;
            defaults = res.1;
            dotted = false;
        },
        Datum::DottedList(ref elems, ref tail) => {
            let res = process_params(elems, symbol_table);
            names = res.0;
            defaults = res.1;

            if let Datum::Symbol(ref v) = **tail {
                names.push(symbol_table.insert(v));
            } else {
                panic!("Dotted lambda `. rest` must be a symbol")
            }
            dotted = true;
        },
        ref other => {
            panic!("Lambda parameters must be (a b ...) or (a b ... . c), found {}", other);
        }
    }

    let mut new_env = AEnv::new(Some(env_ref.clone()));
    new_env.extend(names.clone());
    let new_env_ref = Rc::new(RefCell::new(new_env));

    let body = preprocess_sequence(elems, symbol_table, builtins, syntax_rules, new_env_ref);
    Ok(Expression::LambdaDef(names.len(), defaults, Box::new(body), dotted))
}

pub fn preprocess(
    datum: Datum,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<String, SyntaxRule>,
    env_ref: AEnvRef
    ) -> Result<Expression, LispErr> {
    match datum {
        Datum::List(mut elems) => {
            if elems.len() == 0 {
                return Err(InvalidNumberOfArguments)
            }

            let name = elems.remove(0);
            match name {
                Datum::Symbol(s) => {
                    match s.as_ref() {
                        "fn" => preprocess_fn(elems, symbol_table, builtins, syntax_rules, env_ref.clone()),
                        "do" => Ok(preprocess_sequence(elems, symbol_table, builtins, syntax_rules, env_ref.clone())),
                        "quote" => Ok(Expression::Quote(Box::new(elems.remove(0)))),
                        "defsyntax" => {
                            let name = elems.remove(0).as_symbol().unwrap();
                            let literals = elems.remove(0).as_list().unwrap();
                            let rules = elems.remove(0).as_list().unwrap();
                            let syntax_rule = SyntaxRule::parse(name.clone(), literals, rules);

                            Ok(Expression::SyntaxRuleDefinition(name, Box::new(syntax_rule)))
                        }
                        "def" => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                if builtins.contains_key(a) {
                                    panic!("{} is a reserved name", a);
                                }

                                let symbol = symbol_table.insert(a);

                                let foo = env_ref.borrow_mut().insert(&symbol);
                                if foo.is_some() {
                                    let value = preprocess_sequence(elems, symbol_table, builtins, syntax_rules, env_ref.clone());
                                    Ok(Expression::Definition(Box::new(value)))
                                } else {
                                    panic!("Trying to redefine existing variable {}", a);
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "list-ref"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                let symbol = symbol_table.insert(a);

                                let foo = env_ref.borrow_mut().lookup(&symbol);
                                if let Some(binding) = foo {
                                    let index = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                                    Ok(Expression::ListRef(binding, Box::new(index)))
                                } else {
                                    panic!("Trying to list-ref undefined variable");
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set!"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                let symbol = symbol_table.insert(a);

                                let foo = env_ref.borrow_mut().lookup(&symbol);
                                if let Some(binding) = foo {
                                    let value = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                                    Ok(Expression::Assignment(binding, Box::new(value)))
                                } else {
                                    panic!("Trying to set! undefined variable");
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "push!"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                let symbol = symbol_table.insert(a);

                                let foo = env_ref.borrow_mut().lookup(&symbol);
                                if let Some(binding) = foo {
                                    let value = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                                    Ok(Expression::ListPush(binding, Box::new(value)))
                                } else {
                                    panic!("Trying to set! undefined variable");
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set-nth!"       => {
                            check_arity!(elems, 3);

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                let symbol = symbol_table.insert(a);

                                let foo = env_ref.borrow_mut().lookup(&symbol);
                                if let Some(binding) = foo {
                                    let index = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                                    let value = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;

                                    Ok(Expression::ListSet(binding, Box::new(index), Box::new(value)))
                                } else {
                                    panic!("Trying to set! undefined variable");
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "if"        => {
                            if elems.len() < 2 {
                                return Err(InvalidNumberOfArguments);
                            }

                            let cond = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                            let cons = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                            let alt = match elems.pop() {
                                Some(v) => preprocess(v, symbol_table, builtins, syntax_rules, env_ref.clone())?,
                                None => Expression::datum_nil()
                            };

                            Ok(Expression::If(Box::new(cond), Box::new(cons), Box::new(alt)))
                        },
                        other => {
                            if let Some(sr) = syntax_rules.get(other.clone()) {
                                let expanded = sr.apply(elems);
                                return preprocess(expanded, symbol_table, builtins, syntax_rules, env_ref.clone());
                            }

                            let exprs = preprocess_list(elems, symbol_table, builtins, syntax_rules, env_ref.clone());
                            match builtins.get(other.clone()) {
                                Some(&LispFn(ref fun, ref arity)) => {
                                    arity.check(exprs.len());
                                    Ok(Expression::BuiltinFunctionCall(*fun, exprs))
                                },
                                None => {
                                    let symbol = symbol_table.insert(&other.to_string());
                                    if let Some(binding) = env_ref.borrow_mut().lookup(&symbol) {
                                        Ok(Expression::FunctionCall(Box::new(Expression::BindingRef(binding)), exprs))
                                    } else {
                                        panic!("Trying to use undefined variable {}", other);
                                    }
                                }
                            }
                        }
                    }
                },
                other => {
                    let fun = preprocess(other, symbol_table, builtins, syntax_rules, env_ref.clone())?;
                    let exprs = preprocess_list(elems, symbol_table, builtins, syntax_rules, env_ref.clone());
                    Ok(Expression::FunctionCall(Box::new(fun), exprs))
                }
            }
        },
        Datum::Symbol(ref name) => {
            match builtins.get(name) {
                Some(fun) => Ok(Expression::make_self_evaluating(Datum::Builtin(fun.clone()))),
                None => {
                    let symbol = symbol_table.insert(name);
                    if let Some(binding) = env_ref.borrow_mut().lookup(&symbol) {
                        Ok(Expression::BindingRef(binding))
                    } else {
                        panic!("Trying to use undefined variable {}", name);
                    }
                }
            }

        },
        Datum::DottedList(_, _) => panic!("Malformed expression"),
        other => Ok(Expression::make_self_evaluating(other)),
    }
}
