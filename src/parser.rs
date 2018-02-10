use std::iter::Peekable;
use std::result::Result;

use lexer::{Lexer, LexerError, Token, Literal, Position};
use symbol_table::SymbolTable;

use ::Datum;

#[derive(Debug)]
pub struct ParserError {
    start: Position,
    end: Position,
    error: ParserErrorType
}

#[derive(Debug)]
pub enum ParserErrorType {
    UnexpectedEndOfInput,
    UnexpectedToken,
    UnexpectedDot,
    UnbalancedBracket,
    InvalidDottedList,
    InvalidNumberLiteral,
    InvalidInfixList
}

use self::ParserErrorType::*;

#[derive(Debug)]
pub enum LispError {
    LexerError(LexerError),
    ParserError(ParserError)
}

impl From<LexerError> for LispError {
    fn from(error: LexerError) -> Self {
        LispError::LexerError(error)
    }
}

impl From<ParserError> for LispError {
    fn from(error: ParserError) -> Self {
        LispError::ParserError(error)
    }
}

pub struct Parser<'a> {
    input: Peekable<Lexer<'a>>,
    end: Position,
}

impl<'a> Parser<'a> {
    pub fn from_string(string : &'a String) -> Self {
        let lexer = Lexer::from_string(string);
        Parser {
            input: lexer.peekable(),
            end: Position(0, 0),
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>, LispError> {
        if let Some(lit_err) = self.input.next() {
            let lit = lit_err?;
            self.end = lit.end.clone();
            Ok(Some(lit))
        } else {
            Ok(None)
        }
    }

    pub fn next_datum(&mut self, st: &mut SymbolTable) -> Result<Option<Datum>, LispError> {
        if let Some(t) = self.next()? {
            match t.literal {
                Literal::Bool(v) => Ok(Some(Datum::Bool(v))),
                Literal::Char(v) => Ok(Some(Datum::Char(v))),
                Literal::String(v) => Ok(Some(Datum::String(v))),
                Literal::Identifier(v) => {
                    let id = st.insert(&v);
                    Ok(Some(Datum::Symbol(id)))
                },
                Literal::Number(sign, base, body) => {
                    match isize::from_str_radix(&body.replace("_", ""), base as u32) {
                        Ok(i) => {
                            let number = if sign { i } else { -i };
                            Ok(Some(Datum::Integer(number)))
                        },
                        Err(_err) => Err(ParserError {
                            start: t.start.clone(),
                            end: self.end.clone(),
                            error: InvalidNumberLiteral
                        })?
                    }
                }
                Literal::LRoundBracket => {
                    Ok(Some(self.process_list(t.start.clone(), Literal::RRoundBracket, false, st)?))
                },
                Literal::LSquareBracket => {
                    Ok(Some(self.process_list(t.start.clone(), Literal::RSquareBracket, false, st)?))
                },
                Literal::HashLRoundBracket => {
                    Ok(Some(self.process_list(t.start.clone(), Literal::RRoundBracket, true, st)?))
                },
                Literal::HashLSquareBracket => {
                    Ok(Some(self.process_list(t.start.clone(), Literal::RSquareBracket, true, st)?))
                },
                Literal::AmpersandLRoundBracket => {
                    let body = self.process_simple_list(t.start.clone(), Literal::RRoundBracket, st)?;
                    Ok(Some(self.convert_hole_lambda_to_lambda(body, st)))
                },
                Literal::AmpersandLSquareBracket => {
                    let body = self.process_simple_list(t.start.clone(), Literal::RSquareBracket, st)?;
                    Ok(Some(self.convert_hole_lambda_to_lambda(body, st)))
                },
                Literal::LCurlyBracket => {
                    let body = self.process_simple_list(t.start.clone(), Literal::RCurlyBracket, st)?;
                    match self.convert_infix_to_prefix(body) {
                        Ok(res) => Ok(Some(res)),
                        Err(error) => {
                            Err(ParserError {
                                start: t.start,
                                end: t.end,
                                error: error
                            })?
                        }
                    }
                },
                Literal::RRoundBracket => {
                    Err(ParserError {
                        start: t.start,
                        end: t.end,
                        error: UnbalancedBracket
                    })?
                },
                Literal::RSquareBracket => {
                    Err(ParserError {
                        start: t.start,
                        end: t.end,
                        error: UnbalancedBracket
                    })?
                },
                Literal::RCurlyBracket => {
                    Err(ParserError {
                        start: t.start,
                        end: t.end,
                        error: UnbalancedBracket
                    })?
                },
                Literal::Quote => {
                    match self.next_datum(st)? {
                        Some(d) => {
                            match d {
                                Datum::Pair(elems) => {
                                    Ok(Some(Datum::make_list_from_vec(
                                        vec![self.make_symbol("quote", st), Datum::Pair(elems)]
                                    )))
                                },
                                // TODO: Fix this, using next_datum here is not good
                                // because it uses `make_pair` and that returns Nil for empty pairs
                                Datum::Nil => {
                                    Ok(Some(Datum::Nil))
                                },
                                other => {
                                    Ok(Some(Datum::make_list_from_vec(vec![self.make_symbol("quote", st), other])))
                                }
                            }
                        },
                        None => {
                            Err(ParserError {
                                start: t.start.clone(),
                                end: self.end.clone(),
                                error: UnexpectedEndOfInput
                            })?
                        }
                    }
                },
                Literal::Quasiquote => {
                    match self.next_datum(st)? {
                        Some(d) => {
                            Ok(Some(Datum::make_list_from_vec(vec![self.make_symbol("quasiquote", st), d])))
                        },
                        None => {
                            Err(ParserError {
                                start: t.start.clone(),
                                end: self.end.clone(),
                                error: UnexpectedEndOfInput
                            })?
                        }
                    }
                },
                Literal::Unquote => {
                    match self.next_datum(st)? {
                        Some(d) => {
                            Ok(Some(Datum::make_list_from_vec(vec![self.make_symbol("unquote", st), d])))
                        },
                        None => {
                            Err(ParserError {
                                start: t.start.clone(),
                                end: self.end.clone(),
                                error: UnexpectedEndOfInput
                            })?
                        }
                    }
                },
                Literal::UnquoteSplicing => {
                    match self.next_datum(st)? {
                        Some(d) => {
                            Ok(Some(Datum::make_list_from_vec(vec![self.make_symbol("unquote-splicing", st), d])))
                        },
                        None => {
                            Err(ParserError {
                                start: t.start.clone(),
                                end: self.end.clone(),
                                error: UnexpectedEndOfInput
                            })?
                        }
                    }
                },
                _ => {
                    // NOTE: The `?` is necessary to convert this error
                    // to a LispError
                    Err(ParserError {
                        start: t.start,
                        end: t.end,
                        error: UnexpectedToken
                    })?
                }
            }
        } else {
            Ok(None)
        }
    }

    fn is_peek_none(&mut self) -> bool {
        self.input.peek().is_none()
    }

    fn is_peek_eq(&mut self, literal: &Literal) -> Result<bool, LispError> {
        match self.input.peek() {
            None => Ok(false),
            Some(&Err(ref err)) => Err(err.clone())?,
            Some(&Ok(ref l)) => Ok(l.literal == *literal)
        }
    }

    fn process_simple_list(&mut self, start: Position, closing: Literal, st: &mut SymbolTable)
        -> Result<Vec<Datum>, LispError> {
        let mut res = Vec::new();

        loop {
            if self.is_peek_none() {
                return Err(ParserError {
                    start: start.clone(),
                    end: self.end.clone(),
                    error: UnexpectedEndOfInput
                })?
            }

            if self.is_peek_eq(&closing)? {
                self.next()?;
                break
            } else if self.is_peek_eq(&Literal::Dot)? {
                return Err(ParserError {
                    start: start.clone(),
                    end: self.end.clone(),
                    error: UnexpectedDot
                })?
            } else {
                if let Some(n) = self.next_datum(st)? {
                    res.push(n);
                } else { 
                    panic!("Unexpected end of input")
                }
            }
        }

        Ok(res)
    }

    fn process_list(&mut self, start: Position, closing: Literal, is_vector: bool, st: &mut SymbolTable)
        -> Result<Datum, LispError> {
        let mut res = Vec::new();

        loop {
            if self.is_peek_none() {
                return Err(ParserError {
                    start: start.clone(),
                    end: self.end.clone(),
                    error: UnexpectedEndOfInput
                })?
            }

            if self.is_peek_eq(&closing)? {
                self.next()?;
                break
            } else if self.is_peek_eq(&Literal::Dot)? {
                if is_vector {
                    return Err(ParserError {
                        start: start.clone(),
                        end: self.end.clone(),
                        error: UnexpectedDot
                    })?
                }
                // skip dot
                self.next()?;

                let tail = match self.next_datum(st)? {
                    Some(d) => d,
                    None => {
                        return Err(ParserError {
                            start: start.clone(),
                            end: self.end.clone(),
                            error: InvalidDottedList
                        })?
                    }
                };

                if self.is_peek_eq(&closing)? {
                    self.next()?;
                } else {
                    return Err(ParserError {
                        start: start.clone(),
                        end: self.end.clone(),
                        error: InvalidDottedList
                    })?
                }

                return Ok(Datum::make_dotted_list_from_vec(res, tail));
            } else {
                if let Some(n) = self.next_datum(st)? {
                    res.push(n);
                } else { 
                    panic!("Unexpected end of input")
                }
            }
        }

        if is_vector {
            Ok(Datum::make_vector_from_vec(res))
        } else {
            Ok(Datum::make_list_from_vec(res))
        }
    }

    fn make_symbol(&mut self, sym: &str, st: &mut SymbolTable) -> Datum {
        let id = st.insert(&String::from(sym));
        Datum::Symbol(id)
    }

    fn find_max_hole(&mut self, datum: &Datum, st: &mut SymbolTable) -> isize {
        let mut max = 0;
        match datum {
            &Datum::Pair(ref ptr) => {
                let elems = ptr.borrow().collect_list().unwrap();
                for d in elems {
                    let res = self.find_max_hole(&d, st);
                    if res > max {
                        max = res;
                    }
                }
            }
            &Datum::Symbol(id) => {
                let mut tmp = st.lookup(id).clone();
                let first = tmp.remove(0);

                if first == '&' {
                    let res = tmp.parse::<isize>().expect("Could not parse hole index");
                    if res > max {
                        max = res
                    }
                }
            }
            _ => ()
        }
        max
    }

    fn convert_hole_lambda_to_lambda(&mut self, datums: Vec<Datum>, st: &mut SymbolTable) -> Datum {
        let body = Datum::make_list_from_vec(datums);
        let max = self.find_max_hole(&body, st);

        let mut params: Vec<Datum> = Vec::new();

        for i in 1..(max + 1) {
            let param = format!("&{}", i);
            params.push(self.make_symbol(&param, st));
        }

        Datum::make_list_from_vec(vec![self.make_symbol("fn", st), Datum::make_list_from_vec(params), body])
    }

    // Converts a list of the form {1 + 2 + 3} to (+ 1 2 3)
    fn convert_infix_to_prefix(&mut self, datums: Vec<Datum>) -> Result<Datum, ParserErrorType> {
        // Infix lists must have an odd number of elements
        // and at least 3
        if datums.len() < 3 || (datums.len() % 2 == 0) {
            return Err(InvalidInfixList);
        }

        let op = datums.get(1).unwrap().clone();
        let mut args = vec![
            op.clone(), 
            datums.get(0).unwrap().clone(),
            datums.get(2).unwrap().clone()
        ];

        for i in 3..datums.len() {
            if i % 2 == 0 {
                args.push(datums.get(i).unwrap().clone());
            } else {
                if datums.get(i).unwrap() != &op {
                    return Err(InvalidInfixList);
                }
            }
        }

        Ok(Datum::make_list_from_vec(args))
    }
}
