use std::collections::HashMap;

use ::Datum;
use ::Expression;
use ::LispErr;
use ::LispFn;
use ::LambdaType;
use ::Symbol;
use ::LispErr::*;
use symbol_table::SymbolTable;

use std::collections::BTreeMap;

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
    builtins: &HashMap<String, LispFn>
) -> Vec<Expression> {
    bodies.into_iter().map(
        |b| preprocess(b.clone(), symbol_table, builtins).unwrap()
    ).collect()
}

fn preprocess_sequence(
    bodies: Vec<Datum>,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>
) -> Expression {
    if bodies.len() == 0 {
        return Expression::datum_nil();
    }

    let mut exprs = preprocess_list(bodies, symbol_table, builtins);
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
    builtins: &HashMap<String, LispFn>
) -> Result<Expression, LispErr> {
    if elems.len() < 2 {
        return Err(InvalidNumberOfArguments);
    }

    let mut names;
    let defaults;
    let lambda_type: LambdaType;

    let params = elems.remove(0);
    match params {
        Datum::List(ref elems) => {
            let res = process_params(elems, symbol_table);
            names = res.0;
            defaults = res.1;
            lambda_type = LambdaType::List;
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
            lambda_type = LambdaType::DottedList;
        },
        ref other => {
            panic!("Lambda parameters must be (a b ...) or (a b ... . c), found {}", other);
        }
    }

    let body = preprocess_sequence(elems, symbol_table, builtins);
    Ok(Expression::LambdaDef(names, defaults, Box::new(body), lambda_type))
}

pub fn preprocess(
    datum: Datum,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>
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
                        "def" => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);
                            let value = preprocess_sequence(elems, symbol_table, builtins);

                            if let Datum::Symbol(ref a) = key {
                                if builtins.contains_key(a) {
                                    panic!("{} is a reserved name", a);
                                }
                                Ok(Expression::Definition(symbol_table.insert(a), Box::new(value)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "defn" => {
                            if elems.len() < 3 {
                                return Err(InvalidNumberOfArguments);
                            }

                            let key = elems.remove(0);

                            if let Datum::Symbol(ref a) = key {
                                if builtins.contains_key(a) {
                                    panic!("{} is a reserved name", a);
                                }
                                let value = preprocess_fn(elems, symbol_table, builtins)?;
                                Ok(Expression::Definition(symbol_table.insert(a), Box::new(value)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        // "defmacro" => {
                        //     check_arity!(args, 2);
                        //     if let Datum::Symbol(ref a) = args[0] {
                        //         if builtins.contains_key(a) {
                        //             panic!("{} is a reserved name", a);
                        //         }
                        //         let body = args.get(1).unwrap().clone();
                        //         Ok(Expression::MacroDefinition(
                        //             symbol_table.insert(a),
                        //             Box::new(preprocess(body, symbol_table, builtins)?)))
                        //     } else {
                        //         Err(InvalidTypeOfArguments)
                        //     }
                        // },
                        "set!"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);
                            let value = preprocess_sequence(elems, symbol_table, builtins);

                            if let Datum::Symbol(ref a) = key {
                                Ok(Expression::Assignment(symbol_table.insert(a), Box::new(value)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "list-ref"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);
                            let index = preprocess(elems.remove(0), symbol_table, builtins)?;

                            if let Datum::Symbol(ref a) = key {
                                Ok(Expression::ListRef(symbol_table.insert(a), Box::new(index)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "push!"       => {
                            check_arity!(elems, 2);

                            let key = elems.remove(0);
                            let value = preprocess(elems.remove(0), symbol_table, builtins)?;

                            if let Datum::Symbol(ref a) = key {
                                Ok(Expression::ListPush(symbol_table.insert(a), Box::new(value)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set-nth!"       => {
                            check_arity!(elems, 3);

                            let key = elems.remove(0);
                            let index = preprocess(elems.remove(0), symbol_table, builtins)?;
                            let value = preprocess(elems.remove(0), symbol_table, builtins)?;

                            if let Datum::Symbol(ref a) = key {
                                Ok(Expression::ListSet(
                                        symbol_table.insert(a),
                                        Box::new(index),
                                        Box::new(value)
                                        ))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        // TODO: Refactor & clean up
                        // TODO: I'm not sure how to handle default args in
                        // in combination with dotted list lambdas.
                        // For now, the `. rest` can't have a default
                        "fn"        => preprocess_fn(elems, symbol_table, builtins),
                        "if"        => {
                            if elems.len() < 2 {
                                return Err(InvalidNumberOfArguments);
                            }

                            let cond = preprocess(elems.remove(0), symbol_table, builtins)?;
                            let cons = preprocess(elems.remove(0), symbol_table, builtins)?;
                            let alt = match elems.pop() {
                                Some(v) => preprocess(v, symbol_table, builtins)?,
                                None => Expression::datum_nil()
                            };

                            Ok(Expression::If(Box::new(cond), Box::new(cons), Box::new(alt)))
                        },
                        "cond"      => {
                            let mut else_case = Expression::datum_nil();
                            let mut conditions: Vec<(Expression, Expression)> = Vec::new();

                            if elems.len() % 2 == 1 {
                                panic!("cond takes an even number of arguments");
                            }

                            for elems in elems.chunks(2).into_iter() {
                                let cond = elems.get(0).unwrap();
                                let cons = elems.get(1).unwrap();

                                // TODO this does not check if "else" comes last
                                if *cond == Datum::Symbol("else".to_string()) {
                                    else_case = preprocess(cons.clone(), symbol_table, builtins)?;
                                    break;
                                } else {
                                    let condition = (
                                        preprocess(cond.clone(), symbol_table, builtins)?,
                                        preprocess(cons.clone(), symbol_table, builtins)?,
                                        );
                                    conditions.push(condition);
                                }
                            }

                            let mut cur = else_case;

                            for (cond, cons) in conditions.into_iter().rev() {
                                cur = Expression::If(
                                    Box::new(cond),
                                    Box::new(cons),
                                    Box::new(cur)
                                    );
                            }

                            Ok(cur)
                        },
                        "do"         => Ok(preprocess_sequence(elems, symbol_table, builtins)),
                        "and"        => {
                            if elems.len() == 0 {
                                Ok(Expression::datum_true())
                            } else if elems.len() == 1 {
                                Ok(preprocess(elems.remove(0), symbol_table, builtins)?)
                            } else {
                                let exprs = preprocess_list(elems, symbol_table, builtins);
                                let mut cur = Expression::datum_true();

                                for e in exprs.into_iter().rev() {
                                    cur = Expression::make_if(e, cur, Expression::datum_false());
                                }

                                Ok(cur)
                            }
                        },
                        "or"        => {
                            if elems.len() == 0 {
                                Ok(Expression::datum_false())
                            } else if elems.len() == 1 {
                                Ok(preprocess(elems.remove(0), symbol_table, builtins)?)
                            } else {
                                let exprs = preprocess_list(elems, symbol_table, builtins);
                                let mut cur = Expression::datum_false();

                                for e in exprs.into_iter().rev() {
                                    cur = Expression::make_if(e, Expression::datum_true(), cur);
                                }

                                Ok(cur)
                            }
                        },
                        "quote"     => Ok(Expression::Quote(Box::new(elems.remove(0)))),
                        "~>" => {
                            if elems.len() == 0 {
                                Err(InvalidNumberOfArguments)
                            } else {
                                let mut cur = elems.remove(0);
                                for fun in elems.into_iter() {
                                    match fun {
                                        Datum::List(ref elems) => {
                                            let mut new_elems = elems.clone();
                                            new_elems.push(cur);
                                            cur = Datum::List(new_elems);
                                        },
                                        s @ Datum::Symbol(_) => {
                                            cur = Datum::List(vec!(s, cur));
                                        },
                                        _ => panic!("Arguments to ~> must be lists or symbols")
                                    }
                                }
                                Ok(preprocess(cur, symbol_table, builtins)?)
                            }
                        },
                        "let" => {
                            let bindings = elems.remove(0);

                            let mut variables: Vec<Datum> = Vec::new();
                            let mut values: Vec<Datum> = Vec::new();

                            if let Datum::List(ref elements) = bindings {
                                if elements.len() % 2 == 1 {
                                    panic!("The bindings of let must have an even length")
                                }
                                for varval in elements.chunks(2) {
                                    variables.push(varval[0].clone());
                                    values.push(varval[1].clone());
                                }
                            } else {
                                panic!("First argument of let must be a list")
                            }

                            let mut result: Vec<Datum> = Vec::new();
                            let mut fun: Vec<Datum> = Vec::new();
                            fun.push(Datum::Symbol("fn".to_string()));
                            fun.push(Datum::List(variables));
                            fun.append(&mut elems);

                            result.push(Datum::List(fun));
                            result.append(&mut values);

                            preprocess(Datum::List(result), symbol_table, builtins)
                        },
                        "let*" => {
                            let bindings = elems.remove(0);

                            let mut variables: Vec<Datum> = Vec::new();
                            let mut values: Vec<Datum> = Vec::new();

                            if let Datum::List(ref elements) = bindings {
                                if elements.len() % 2 == 1 {
                                    panic!("The bindings of let must have an even length")
                                }
                                for varval in elements.chunks(2) {
                                    variables.push(varval[0].clone());
                                    values.push(varval[1].clone());
                                }
                            } else {
                                panic!("First argument of let must be a list")
                            }

                            let mut tmp = vec![Datum::Symbol("do".to_string())];
                            tmp.append(&mut elems);

                            let mut result: Datum = Datum::List(tmp);
                            for (key, value) in variables.into_iter().zip(values.into_iter()).rev() {
                                let new = Datum::List(vec![
                                                      Datum::Symbol("let".to_string()),
                                                      Datum::List(vec![key, value]),
                                                      result
                                ]);
                                result = new;
                            }

                            preprocess(result, symbol_table, builtins)
                        },
                        other => {
                            let exprs = preprocess_list(elems, symbol_table, builtins);
                            match builtins.get(other.clone()) {
                                Some(&LispFn(ref fun, ref arity)) => {
                                    arity.check(exprs.len());
                                    Ok(Expression::BuiltinFunctionCall(*fun, exprs))
                                },
                                None => {
                                    let fun = symbol_table.insert(&other.to_string());
                                    Ok(Expression::FunctionCall(Box::new(Expression::Symbol(fun)), exprs))
                                }
                            }
                        }
                    }
                },
                other => {
                    let fun = preprocess(other, symbol_table, builtins)?;
                    let exprs = preprocess_list(elems, symbol_table, builtins);
                    Ok(Expression::FunctionCall(Box::new(fun), exprs))
                }
            }
        },
        Datum::Symbol(ref name) => {
            match builtins.get(name) {
                Some(fun) => Ok(Expression::make_self_evaluating(Datum::Builtin(fun.clone()))),
                None => Ok(Expression::Symbol(symbol_table.insert(name)))
            }

        },
        Datum::DottedList(_, _) => panic!("Malformed expression"),
        other => Ok(Expression::make_self_evaluating(other)),
    }
}
