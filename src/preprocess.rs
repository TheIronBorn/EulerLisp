use ::Datum;
use ::Expression;
use ::LispErr;
use ::LambdaType;
use ::Symbol;
use ::Condition;
use ::LispErr::*;
use symbol_table::SymbolTable;

use macros;

pub fn preprocess(datum: Datum, symbol_table: &mut SymbolTable) -> Result<Expression, LispErr> {
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
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::Definition(
                                    Symbol(symbol_table.insert(a)),
                                    Box::new(preprocess(body, symbol_table)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "defmacro" => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::MacroDefinition(
                                    Symbol(symbol_table.insert(a)),
                                    Box::new(preprocess(body, symbol_table)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "set!"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::Assignment(
                                    Symbol(symbol_table.insert(a)),
                                    Box::new(preprocess(body, symbol_table)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "vector-push!"       => {
                            check_arity!(args, 2);
                            if let Datum::Symbol(ref a) = args[0] {
                                let body = args.get(1).unwrap().clone();
                                Ok(Expression::VectorPush(
                                    Symbol(symbol_table.insert(a)),
                                    Box::new(preprocess(body, symbol_table)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "vector-set!"       => {
                            check_arity!(args, 3);
                            if let Datum::Symbol(ref a) = args[0] {
                                let index = args.get(1).unwrap().clone();
                                let value = args.get(2).unwrap().clone();
                                Ok(Expression::VectorSet(
                                    Symbol(symbol_table.insert(a)),
                                    Box::new(preprocess(index, symbol_table)?),
                                    Box::new(preprocess(value, symbol_table)?)))
                            } else {
                                Err(InvalidTypeOfArguments)
                            }
                        },
                        "fn"        => {
                            check_arity!(args, 2);

                            let mut params: Vec<Symbol> = Vec::new();
                            let mut lambda_type: LambdaType;

                            match args[0] {
                                Datum::Symbol(ref name) => {
                                    params.push(Symbol(symbol_table.insert(name)));
                                    lambda_type = LambdaType::Var;
                                },
                                Datum::List(ref elems) => {
                                    for a in elems {
                                        if let Datum::Symbol(ref v) = *a {
                                            params.push(Symbol(symbol_table.insert(v)));
                                        } else {
                                            return Err(InvalidTypeOfArguments);
                                        }
                                    };
                                    lambda_type = LambdaType::List;
                                },
                                Datum::DottedList(ref elems, ref tail) => {
                                    for a in elems {
                                        if let Datum::Symbol(ref v) = *a {
                                            params.push(Symbol(symbol_table.insert(v)));
                                        } else {
                                            return Err(InvalidTypeOfArguments);
                                        }
                                    }
                                    if let Datum::Symbol(ref v) = **tail {
                                        params.push(Symbol(symbol_table.insert(v)));
                                    }
                                    lambda_type = LambdaType::DottedList;
                                },
                                _ => return Err(InvalidTypeOfArguments),
                            }

                            let body = preprocess(args.get(1).unwrap().clone(), symbol_table)?;
                            Ok(Expression::LambdaDef(params, Box::new(body), lambda_type))
                        },
                        "if"        => {
                            let cond = args.get(0).unwrap().clone();
                            let cons = args.get(1).unwrap().clone();

                            if args.len() == 2 {
                                Ok(Expression::If(
                                        Box::new(preprocess(cond, symbol_table)?),
                                        Box::new(preprocess(cons, symbol_table)?),
                                        Box::new(Expression::Nil),
                                ))
                            } else if args.len() == 3 {
                                let alt = args.get(2).unwrap().clone();
                                Ok(Expression::If(
                                        Box::new(preprocess(cond, symbol_table)?),
                                        Box::new(preprocess(cons, symbol_table)?),
                                        Box::new(preprocess(alt, symbol_table)?),
                                ))
                            } else {
                                return Err(InvalidNumberOfArguments);
                            }
                        },
                        "cond"      => {
                            let mut else_case = Expression::Nil;
                            let mut conditions: Vec<Condition> = Vec::new();

                            for arg in args.into_iter() {
                                if let Datum::List(ref elems) = *arg {
                                    if elems.len() != 2 {
                                        return Err(InvalidTypeOfArguments);
                                    }

                                    let cond = elems.get(0).unwrap();
                                    let cons = elems.get(1).unwrap();

                                    // TODO this does not check if "else" comes last
                                    if *cond == Datum::Symbol("else".to_string()) {
                                        else_case = preprocess(cons.clone(), symbol_table)?;
                                        break;
                                    } else {
                                        let condition = Condition(
                                            Box::new(preprocess(cond.clone(), symbol_table)?),
                                            Box::new(preprocess(cons.clone(), symbol_table)?),
                                         );
                                        conditions.push(condition);
                                    }
                                } else {
                                    return Err(InvalidTypeOfArguments);
                                }
                            }

                            Ok(Expression::Conditional(conditions, Box::new(else_case)))
                        },
                        "do"        => {
                            if args.len() == 0 {
                                Ok(Expression::Nil)
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table)?)
                            } else {
                                // TODO: Make this less complicated
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                                let exprs = maybe_exprs?;
                                let len = exprs.len();
                                let mut exprs = exprs;

                                let last = exprs.remove(len - 1);
                                Ok(Expression::Do(exprs, Box::new(last)))
                            }
                        },
                        "and"        => {
                            if args.len() == 0 {
                                Ok(Expression::Bool(true))
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table)?)
                            } else {
                                // TODO: Make this less complicated
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                                let exprs = maybe_exprs?;
                                let len = exprs.len();
                                let mut exprs = exprs;

                                let last = exprs.remove(len - 1);
                                Ok(Expression::And(exprs, Box::new(last)))
                            }
                        },
                        "or"        => {
                            if args.len() == 0 {
                                Ok(Expression::Bool(false))
                            } else if args.len() == 1 {
                                Ok(preprocess(args.get(0).unwrap().clone(), symbol_table)?)
                            } else {
                                // TODO: Make this less complicated
                                let maybe_exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                    .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                                let exprs = maybe_exprs?;
                                let len = exprs.len();
                                let mut exprs = exprs;

                                let last = exprs.remove(len - 1);
                                Ok(Expression::Or(exprs, Box::new(last)))
                            }
                        },
                        "quote"     => {
                            check_arity!(args, 1);
                            let body = args.get(0).unwrap().clone();
                            Ok(Expression::Quote(Box::new(body)))
                        }
                        // "delay"     => self.sf_delay(args, env_ref),
                        // "force"     => self.sf_force(args, env_ref),
                        // TODO: Not sure how to handle these,
                        // they can't go into `builtin` because the need access to the evaluator
                        v @ "read" | v @ "apply" | v @ "eval" => {
                            let exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                            Ok(Expression::SpecialFunctionCall(v.to_string(), exprs?))
                        }
                        other => {
                            let fun = Symbol(symbol_table.insert(&other.to_string()));
                            let exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                                .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                            Ok(Expression::SymbolFunctionCall(fun, exprs?))
                        }
                    }
                },
                other => {
                    let fun = preprocess(other, symbol_table)?;
                    let exprs: Result<Vec<Expression>, LispErr> = args.into_iter()
                        .map( |arg| preprocess(arg.clone(), symbol_table) ).collect();
                    Ok(Expression::FunctionCall(Box::new(fun), exprs?))
                }
            }
        },
        Datum::Symbol(ref name) => {
            Ok(Expression::Symbol(Symbol(symbol_table.insert(name))))
        },
        Datum::Bool(v) => Ok(Expression::Bool(v)),
        Datum::Vector(v) => Ok(Expression::Vector(v)),
        Datum::Number(v) => Ok(Expression::Number(v)),
        Datum::Character(v) => Ok(Expression::Character(v)),
        Datum::Str(ref v) => Ok(Expression::Str(v.clone())),
        // Datum::List(vs) => Ok(Expression::List(vs)),
        // Datum::DottedList(vs, v) => Ok(Expression::DottedList(vs, v)),
        Datum::Builtin(v) => Ok(Expression::Builtin(v)),
        Datum::Promise(v) => Ok(Expression::Promise(v)),
        Datum::Undefined => Ok(Expression::Undefined),
        Datum::Nil => Ok(Expression::Nil),
        _ => Err(InvalidTypeOfArguments),
    }
}
