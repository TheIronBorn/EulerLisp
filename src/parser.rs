use std::fs::File;
use std::str::FromStr;
use std::str::from_utf8;
use std::str;
use std::iter;

use nom::*;

use ::Value;

static identifier_start: &'static str = "abcdefghijklmnopqrstuvwxyz!$%&*+-./:<=>?@^_~";
static identifier_main: &'static str = "abcdefghijklmnopqrstuvwxyz0123456789!$%&*+-./:<=>?@^_~";

named!(
    identifier<&[u8], String>,
    do_parse!(
        word: recognize!(
            do_parse!(
                one_of!(identifier_start) >>
                many0!(one_of!(identifier_main)) >>
                ()
            )
        ) >>
        (String::from_utf8(word.to_vec()).unwrap())
    )
);

named!(
    number<&[u8], i64>,
    map_res!(
        map_res!(
            digit,
            ::std::str::from_utf8
        ),
        (::std::str::FromStr::from_str)
    )
);

named!(list<&[u8], Vec<Value>>,
    do_parse!(
        tag!("(") >>
        elements: ws!(many0!(value)) >>
        tag!(")") >>
        (elements)
    )
);

named!(boolean<&[u8], bool>,
    alt!(
        map!(tag!("true"), |_| true) |
        map!(tag!("false"), |_| false)
    )
);

// named!(string<&[u8], String>,
//     do_parse!(
//         tag!("\"") >>
//         body: alphanumeric >>
//         tag!("\"") >>
//         (String::from_utf8(body.to_vec()).unwrap())
//     )
// );

named!(not_escaped_seq<&[u8], &[u8]>, take_until_either!(&b"\\\""[..]));
named!(escaped_seq, alt!(tag!("\\r") | tag!("\\n") | tag!("\\t") | tag!("\\\"") | tag!("\\\\")));
named!(string<&[u8], String>,
       do_parse!(
           tag!("\"") >>
           s: many0!(map_res!(alt!(escaped_seq | not_escaped_seq), from_utf8)) >>
           tag!("\"") >>
           ({
               str_lit(&s.into_iter().fold(String::new(),
               |mut accum, slice| {
                   accum.push_str(slice);
                   accum
               })[..])
           })
       )
  );

// TODO: `char_lit` and `str_lit` were taken from
// https://github.com/rust-lang/rust/blob/master/src/libsyntax/parse/mod.rs,
//
// In one of the next versions, they might be accessible publicly

/// Parse a string representing a character literal into its final form.
/// Rather than just accepting/rejecting a given literal, unescapes it as
/// well. Can take any slice prefixed by a character escape. Returns the
/// character and the number of characters consumed.
pub fn char_lit(lit: &str) -> (char, isize) {
    use std::char;

    // Handle non-escaped chars first.
    if lit.as_bytes()[0] != b'\\' {
        // If the first byte isn't '\\' it might part of a multi-byte char, so
        // get the char with chars().
        let c = lit.chars().next().unwrap();
        return (c, 1);
    }

    // Handle escaped chars.
    match lit.as_bytes()[1] as char {
        '"' => ('"', 2),
        'n' => ('\n', 2),
        'r' => ('\r', 2),
        't' => ('\t', 2),
        '\\' => ('\\', 2),
        '\'' => ('\'', 2),
        '0' => ('\0', 2),
        'x' => {
            let v = u32::from_str_radix(&lit[2..4], 16).unwrap();
            let c = char::from_u32(v).unwrap();
            (c, 4)
        }
        'u' => {
            assert_eq!(lit.as_bytes()[2], b'{');
            let idx = lit.find('}').unwrap();
            let v = u32::from_str_radix(&lit[3..idx], 16).unwrap();
            let c = char::from_u32(v).unwrap();
            (c, (idx + 1) as isize)
        }
        _ => panic!("lexer should have rejected a bad character escape {}", lit)
    }
}

/// Parse a string representing a string literal into its final form. Does
/// unescaping.
pub fn str_lit(lit: &str) -> String {
    // debug!("parse_str_lit: given {}", escape_default(lit));
    let mut res = String::with_capacity(lit.len());

    // FIXME #8372: This could be a for-loop if it didn't borrow the iterator
    let error = |i| format!("lexer should have rejected {} at {}", lit, i);

    /// Eat everything up to a non-whitespace
    fn eat<'a>(it: &mut iter::Peekable<str::CharIndices<'a>>) {
        loop {
            match it.peek().map(|x| x.1) {
                Some(' ') | Some('\n') | Some('\r') | Some('\t') => {
                    it.next();
                },
                _ => { break; }
            }
        }
    }

    let mut chars = lit.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '\\' => {
                let ch = chars.peek().unwrap_or_else(|| {
                    panic!("{}", error(i))
                }).1;

                if ch == '\n' {
                    eat(&mut chars);
                } else if ch == '\r' {
                    chars.next();
                    let ch = chars.peek().unwrap_or_else(|| {
                        panic!("{}", error(i))
                    }).1;

                    if ch != '\n' {
                        panic!("lexer accepted bare CR");
                    }
                    eat(&mut chars);
                } else {
                    // otherwise, a normal escape
                    let (c, n) = char_lit(&lit[i..]);
                    for _ in 0..n - 1 { // we don't need to move past the first \
                        chars.next();
                    }
                    res.push(c);
                }
            },
            '\r' => {
                let ch = chars.peek().unwrap_or_else(|| {
                    panic!("{}", error(i))
                }).1;

                if ch != '\n' {
                    panic!("lexer accepted bare CR");
                }
                chars.next();
                res.push('\n');
            }
            c => res.push(c),
        }
    }

    res.shrink_to_fit(); // probably not going to do anything, unless there was an escape.
    // debug!("parse_str_lit: returning {}", res);
    res
}

named!(quote<&[u8], Vec<Value>>,
    do_parse!(
        tag!("'") >>
        value: value >>
        (vec![Value::Atom(String::from("quote")), value])
    )
);

named!(value<&[u8], Value>,
    alt!(
      map!(ws!(boolean), |x| Value::Bool(x)) |
      map!(ws!(list), |x| Value::List(x)) |
      map!(ws!(number), |x| Value::Number(x)) |
      map!(ws!(tag!("'()")), |x| Value::Nil) |
      map!(ws!(string), |x| Value::Str(x)) |
      map!(ws!(identifier), |x| Value::Atom(x)) |
      map!(ws!(quote), |x| Value::List(x))
    )
);

pub fn parse(s: &str) -> Value {
    match value(s.as_bytes()) {
      IResult::Done(_, v) => v,
      _ => panic!("Failed to parse value")
    }
}
