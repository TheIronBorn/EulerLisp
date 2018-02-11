use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use ::Datum;
use ::Meaning;
use ::LispErr;
use ::LispFn;
use ::Symbol;
use ::LispErr::*;

use syntax_rule::SyntaxRule;
use symbol_table::SymbolTable;
use env::{AEnv, AEnvRef};

fn process_params(params: &Vec<Datum>) -> Result<(Vec<Symbol>, Vec<Datum>), LispErr> {
    let mut names = Vec::new();
    let mut defaults = Vec::new();
    let mut had_default = false;

    for param in params {
        match *param {
            Datum::Symbol(v) => {
                if had_default {
                    panic!("All params after one with a default must have defaults");
                }
                names.push(v);
            }, 
            Datum::Pair(ref ptr) => {
                let elems = ptr.borrow().collect_list()?;

                let name = elems.get(0).unwrap();
                let default = elems.get(1).unwrap_or(&Datum::Nil);
                had_default = true;

                if let Datum::Symbol(v) = *name {
                    names.push(v);
                } else {
                    panic!("Function parameters with defaults must have the form (name default)");
                }

                defaults.push(default.clone());
            }, 
            _ => panic!("Function parameters must have the form `name` or `(name default)`")
        }
    }

    Ok((names, defaults))
}

fn preprocess_list(
    bodies: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<Symbol, SyntaxRule>,
    env_ref: AEnvRef
) -> Vec<Meaning> {
    bodies.into_iter().map(
        |b| preprocess(b.clone(), symbol_table, builtins, syntax_rules, env_ref.clone()).unwrap()
    ).collect()
}

fn preprocess_sequence(
    bodies: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<Symbol, SyntaxRule>,
    env_ref: AEnvRef
) -> Meaning {
    if bodies.len() == 0 {
        return Meaning::datum_nil();
    }

    let mut exprs = preprocess_list(bodies, symbol_table, builtins, syntax_rules, env_ref.clone());
    if exprs.len() > 1 {
        let last = exprs.pop().unwrap();
        Meaning::Do(exprs, Box::new(last))
    } else {
        exprs.pop().unwrap()
    }
}

fn preprocess_fn(
    mut elems: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<Symbol, SyntaxRule>,
    env_ref: AEnvRef
) -> Result<Meaning, LispErr> {
    if elems.len() < 2 {
        panic!("Invalid fn {:?}", elems);
    }

    let mut names;
    let defaults;
    let dotted;

    let params = elems.remove(0);
    match params {
        Datum::Pair(ref ptr) => {
            let mut elems = ptr.borrow().collect();
            let tail = elems.pop().unwrap();

            if tail.is_nil() {
                let res = process_params(&elems)?;
                names = res.0;
                defaults = res.1;
                dotted = false;
            } else {
                let res = process_params(&elems)?;
                names = res.0;
                defaults = res.1;

                if let Datum::Symbol(v) = tail {
                    names.push(v);
                } else {
                    panic!("Dotted lambda `. rest` must be a symbol")
                }
                dotted = true;
            }
        },
        Datum::Nil => {
            names = vec![];
            defaults = vec![];
            dotted = false;
        }
        ref other => {
            panic!(
                "Lambda parameters must be (a b ...) or (a b ... . c), found {}",
                other.to_string(symbol_table)
            );
        }
    }

    let mut new_env = AEnv::new(Some(env_ref.clone()));
    new_env.extend(names.clone());
    let new_env_ref = Rc::new(RefCell::new(new_env));

    let body = preprocess_sequence(elems, symbol_table, builtins, syntax_rules, new_env_ref);
    Ok(Meaning::LambdaDef(names.len(), defaults, Box::new(body), dotted))
}

pub fn preprocess(
    datum: Datum,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>,
    syntax_rules: &HashMap<Symbol, SyntaxRule>,
    env_ref: AEnvRef
    ) -> Result<Meaning, LispErr> {
    match datum {
        Datum::Pair(ptr) => {
            let mut elems = ptr.borrow().collect_list()?;
            if elems.len() == 0 {
                panic!("Empty lists are not allowed");
            }

            let name = elems.remove(0);
            match name {
                Datum::Symbol(s) => {
                    match symbol_table.lookup(s).as_ref() {
                        "fn" => preprocess_fn(elems, symbol_table, builtins, syntax_rules, env_ref.clone()),
                        "do" => Ok(preprocess_sequence(elems, symbol_table, builtins, syntax_rules, env_ref.clone())),
                        "quote" => Ok(Meaning::Quote(Box::new(elems.remove(0)))),
                        "defsyntax" => {
                            let name = elems.remove(0).as_symbol().unwrap();
                            let literals = elems.remove(0).as_list().unwrap();
                            let rules = elems.remove(0).as_list().unwrap();
                            let syntax_rule = SyntaxRule::parse(name, literals, rules, symbol_table);

                            Ok(Meaning::SyntaxRuleDefinition(name, Box::new(syntax_rule)))
                        }
                        "def" => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(symbol) = key {
                                let name = symbol_table.lookup(symbol);
                                if builtins.contains_key(&name) {
                                    panic!("{} is a reserved name", name);
                                }

                                let foo = env_ref.borrow_mut().insert(&symbol);
                                if foo.is_some() {
                                    let value = preprocess_sequence(elems, symbol_table, builtins, syntax_rules, env_ref.clone());
                                    Ok(Meaning::Definition(Box::new(value)))
                                } else {
                                    panic!("Trying to redefine existing variable {}", name);
                                }
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set!"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);

                            if let Datum::Symbol(symbol) = key {
                                let foo = env_ref.borrow_mut().lookup(&symbol);
                                if let Some(binding) = foo {
                                    let value = preprocess(elems.remove(0), symbol_table, builtins, syntax_rules, env_ref.clone())?;
                                    Ok(Meaning::Assignment(binding, Box::new(value)))
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
                                None => Meaning::datum_nil()
                            };

                            Ok(Meaning::If(Box::new(cond), Box::new(cons), Box::new(alt)))
                        },
                        other => {
                            if let Some(sr) = syntax_rules.get(&s) {
                                // println!("\nConverted {}",
                                //          Datum::make_list_from_vec(elems.clone()).to_string(symbol_table)
                                //          );
                                let expanded = sr.apply(elems.clone());
                                match expanded {
                                    Some(ex) => {
                                        // println!("To {}", ex.to_string(symbol_table));
                                        return preprocess(ex, symbol_table, builtins, syntax_rules, env_ref.clone());
                                    },
                                    None => {
                                        panic!("No matching in {} pattern for {}",
                                               symbol_table.lookup(s),
                                               Datum::make_list_from_vec(elems).to_string(symbol_table));
                                    }
                                }
                            }

                            let exprs = preprocess_list(elems, symbol_table, builtins, syntax_rules, env_ref.clone());
                            match builtins.get(other.clone()) {
                                Some(&LispFn(ref fun, ref arity, ref name)) => {
                                    arity.check(exprs.len(), name);
                                    Ok(Meaning::BuiltinFunctionCall(*fun, exprs))
                                },
                                None => {
                                    if let Some(binding) = env_ref.borrow_mut().lookup(&s) {
                                        Ok(Meaning::FunctionCall(Box::new(Meaning::BindingRef(binding)), exprs))
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
                    Ok(Meaning::FunctionCall(Box::new(fun), exprs))
                }
            }
        },
        Datum::Symbol(symbol) => {
            let name = symbol_table.lookup(symbol);
            match builtins.get(&name) {
                Some(fun) => Ok(Meaning::self_evaluating(Datum::Builtin(fun.clone()))),
                None => {
                    if let Some(binding) = env_ref.borrow_mut().lookup(&symbol) {
                        Ok(Meaning::BindingRef(binding))
                    } else {
                        panic!("Trying to use undefined variable {}", name);
                    }
                }
            }

        },
        other => Ok(Meaning::self_evaluating(other)),
    }
}
