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

pub fn preprocess(
    datum: Datum,
    symbol_table: &mut SymbolTable,
    builtins: &HashMap<String, LispFn>
) -> Result<Expression, LispErr> {
    match datum {
        Datum::List(ref elems) => {
            if elems.len() == 0 {
                return Err(InvalidNumberOfArguments)
            }
            let args = &elems[1..];
            match elems[0].clone() {
                Datum::Symbol(s) => {
                    match s.as_ref() {
                        "def"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                if builtins.contains_key(a) {
                                    panic!("{} is a reserved name", a);
                                }
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::Definition(symbol_table.insert(a),
                                    Box::new(preprocess(body, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "defmacro" => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                if builtins.contains_key(a) {
                                    panic!("{} is a reserved name", a);
                                }
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::MacroDefinition(
                                    symbol_table.insert(a),
                                    Box::new(preprocess(body, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set!"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::Assignment(
                                    symbol_table.insert(a),
                                    Box::new(preprocess(body, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "list-ref"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::ListRef(
                                    symbol_table.insert(a),
                                    Box::new(preprocess(body, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "push!"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::ListPush(
                                    symbol_table.insert(a),
                                    Box::new(preprocess(body, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set-nth!"       => {
                            check_arity!(args, 3);
                            if let Datum::Symbol(ref a) = args[0] {
                                let index = args.get(1).unwrap().clone();
                                let value = args.get(2).unwrap().clone();
                                Ok(Expression::ListSet(
                                    symbol_table.insert(a),
                                    Box::new(preprocess(index, symbol_table, builtins)?),
                                    Box::new(preprocess(value, symbol_table, builtins)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "fn"        => {
                            check_arity!(args, 2);

                            let mut params: Vec<Symbol> = Vec::new();
                            let lambda_type: LambdaType;

                            match args[0] {
                                Datum::Symbol(ref name) => {
                                    params.push(symbol_table.insert(name));
                                    lambda_type = LambdaType::Var;
                                },
                                Datum::List(ref elems) => {
                                    for a in elems {
                                        if let Datum::Symbol(ref v) = *a {
                                            params.push(symbol_table.insert(v));
                                        } else {
                                            return Err(InvalidTypeOfArguments);
                                        }
                                    };
                                    lambda_type = LambdaType::List;
                                },
                                Datum::DottedList(ref elems, ref tail) => {
                                    for a in elems {
                                        if let Datum::Symbol(ref v) = *a {
                                            params.push(symbol_table.insert(v));
                                        } else {
                                            return Err(InvalidTypeOfArguments);
                                        }
                                    }
                                    if let Datum::Symbol(ref v) = **tail {
                                        params.push(symbol_table.insert(v));
                                    }
                                    lambda_type = LambdaType::DottedList;
                                },
                                _ => return Err(InvalidTypeOfArguments),
                            }

                            let body = preprocess(args.get(1).unwrap().clone(), symbol_table, builtins)?;
                            Ok(Expression::LambdaDef(params, Box::new(body), lambda_type))
                        },
                        "if"        => {
                            let cond = args.get(0).unwrap().clone();
                            let cons = args.get(1).unwrap().clone();

                            if args.len() == 2 {
                                Ok(Expression::If(
                                        Box::new(preprocess(cond, symbol_table, builtins)?),
                                        Box::new(preprocess(cons, symbol_table, builtins)?),
                                        Box::new(Expression::SelfEvaluating(Box::new(Datum::Nil))),
                                ))
                            } else if args.len() == 3 {
                                let alt = args.get(2).unwrap().clone();
                                Ok(Expression::If(
                                        Box::new(preprocess(cond, symbol_table, builtins)?),
                                        Box::new(preprocess(cons, symbol_table, builtins)?),
                                        Box::new(preprocess(alt, symbol_table, builtins)?),
                                ))
                            } else {
                                return Err(InvalidNumberOfArguments);
                            }
                        },
                        "cond"      => {
                            let mut else_case = Expression::SelfEvaluating(Box::new(Datum::Nil));
                            let mut conditions: Vec<(Expression, Expression)> = Vec::new();

                            if args.len() % 2 == 1 {
                                panic!("cond takes an even number of arguments");
                            }

                            for elems in args.chunks(2).into_iter() {
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
                        "case"      => {
                            let mut else_case = Expression::SelfEvaluating(Box::new(Datum::Nil));
                            let mut cases: BTreeMap<Datum, Expression> = BTreeMap::new();

                            let expr_ = args.get(0).unwrap();
                            let expr = preprocess(expr_.clone(), symbol_table, builtins)?;

                            for arg in args.into_iter().skip(1) {
                                if let Datum::List(ref elems) = *arg {
                                    if elems.len() != 2 {
                                        return Err(InvalidTypeOfArguments);
                                    }

                                    let cond = elems.get(0).unwrap();
                                    let cons = elems.get(1).unwrap();

                                    // TODO this does not check if "else" comes last
                                    if *cond == Datum::Symbol("else".to_string()) {
                                        else_case = preprocess(cons.clone(), symbol_table, builtins)?;
                                        break;
                                    } else {
                                        cases.insert(
                                            cond.clone(),
                                            preprocess(cons.clone(), symbol_table, builtins)?,
                                        );
                                    }
                                } else {
                                    return Err(InvalidTypeOfArguments);
                                }
                            }

                            Ok(Expression::Case(Box::new(expr), cases, Box::new(else_case)))
                        },
                        "do"        => {
                            if args.len() == 0 {
                                Ok(Expression::SelfEvaluating(Box::new(Datum::Nil)))
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table, builtins)?)
                            } else {
                                // TODO: Make this less complicated
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                                let exprs = maybe_exprs?;
                                let len = exprs.len();
                                let mut exprs = exprs;

                                let last = exprs.remove(len - 1);
                                Ok(Expression::Do(exprs, Box::new(last)))
                            }
                        },
                        "and"        => {
                            if args.len() == 0 {
                                Ok(Expression::SelfEvaluating(Box::new(Datum::Bool(true))))
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table, builtins)?)
                            } else {
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                                let exprs = maybe_exprs?.into_iter().rev();

                                let mut cur = Expression::SelfEvaluating(Box::new(Datum::Bool(true)));
                                for e in exprs {
                                    cur = Expression::If(
                                        Box::new(e),
                                        Box::new(cur),
                                        Box::new(Expression::SelfEvaluating(Box::new(Datum::Bool(false))))
                                    );
                                }

                                Ok(cur)
                            }
                        },
                        "or"        => {
                            if args.len() == 0 {
                                Ok(Expression::SelfEvaluating(Box::new(Datum::Bool(false))))
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table, builtins)?)
                            } else {
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                                let exprs = maybe_exprs?.into_iter().rev();
                                let mut cur = Expression::SelfEvaluating(Box::new(Datum::Bool(false)));

                                for e in exprs {
                                    cur = Expression::If(
                                        Box::new(e),
                                        Box::new(Expression::SelfEvaluating(Box::new(Datum::Bool(true)))),
                                        Box::new(cur)
                                    );
                                }
                                Ok(cur)
                            }
                        },
                        "quote"     => {
                            check_arity!(args, 1);
                            let body = args.get(0).unwrap().clone();
                            Ok(Expression::Quote(Box::new(body)))
                        },
                        "~>" => {
                            if args.len() == 0 {
                                Err(InvalidNumberOfArguments)
                            } else {
                                let mut cur = args.get(0).unwrap().clone();

                                for fun in args.into_iter().skip(1) {
                                    match *fun {
                                        Datum::List(ref elems) => {
                                            let mut new_elems = elems.clone();
                                            new_elems.push(cur);
                                            cur = Datum::List(new_elems);
                                        },
                                        ref s @ Datum::Symbol(_) => {
                                            cur = Datum::List(vec!(
                                               s.clone(), cur
                                            ));
                                        },
                                        _ => panic!("Arguments to ~> must be lists")
                                    }
                                }

                                Ok(preprocess(cur, symbol_table, builtins)?)
                            }
                        },
                        // "delay"     => self.sf_delay(args, env_ref),
                        // "force"     => self.sf_force(args, env_ref),
                        // TODO: Not sure how to handle these,
                        // they can't go into `builtin` because the need access to the evaluator
                        v @ "read" | v @ "apply" | v @ "eval" | v @ "map" => {
                            let exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                            Ok(Expression::SpecialFunctionCall(v.to_string(), exprs?))
                        }
                        other => {
                            let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                            let exprs = maybe_exprs?;
                            match builtins.get(other.clone()) {
                                Some(&LispFn(ref fun, ref arity)) => {
                                    arity.check(exprs.len());
                                    Ok(Expression::BuiltinFunctionCall(*fun, exprs))
                                },
                                None => {
                                    let fun = symbol_table.insert(&other.to_string());
                                    Ok(Expression::FunctionCall(
                                        Box::new(Expression::Symbol(fun)),
                                        exprs
                                    ))
                                }
                            }
                        }
                    }
                },
                other => {
                    let fun = preprocess(other, symbol_table, builtins)?;
                    let exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                        .map( |arg| preprocess(arg.clone(), symbol_table, builtins) ).collect();
                    Ok(Expression::FunctionCall(Box::new(fun), exprs?))
                }
            }
        },
        Datum::Symbol(ref name) => {
            match builtins.get(name) {
                Some(fun) => {
                    Ok(Expression::SelfEvaluating(Box::new(Datum::Builtin(fun.clone()))))
                },
                None => {
                    Ok(Expression::Symbol(symbol_table.insert(name)))
                }
            }

        },
        Datum::DottedList(_, _) => panic!("Malformed expression"),
        other => Ok(Expression::SelfEvaluating(Box::new(other))),
    }
}
