//! token.rs
//!
//! A parser that turns code into a stream of tokens
//! tokens are the words that make up program source code. For example, the expression
//!
//! ```text
//! let x = if a == 0 { 1 } else { 2 };
//! ```
//!
//! Could be analysed as the following tokens:
//!
//! ```text
//! [LET, IDENTIFIER("x"), ASSIGN, IF, IDENTIFIER("a"), EQ, NUM(0), LCURLY,
//! NUM(1), RCURLY, ELSE, LCURLY, NUM(2), RCURLY, SEMICOLON]
//! ```
//!
//! Notice how these tokens say nothing about the structure of the program:
//! they are just words that the program recognises as valid

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit0;
use nom::combinator::{eof, map_res};
use nom::sequence::{delimited, terminated};
use nom::{
    character::complete::satisfy, combinator::recognize, error::Error, multi::many0,
    sequence::tuple, IResult,
};
use nom::{Finish, Parser};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub enum Token {
    Identifier(String),
    Int(i32),
    Bool(bool),

    // Keywords
    Let,
    If,
    Print,

    // Punctuation
    Assign,
    LParen,
    RParen,
    LCurly,
    RCurly,
    GT,
    Eq,
    Plus,
    Newline,
}

impl Token {
    pub fn int(&self) -> i32 {
        if let Token::Int(x) = self {
            *x
        } else {
            panic!("tried to convert non-integer token to integer")
        }
    }

    pub fn identifier(&self) -> String {
        if let Token::Identifier(s) = self {
            s.clone()
        } else {
            panic!("tried to convert non-identifier token to identifier")
        }
    }

    pub fn bool(&self) -> bool {
        if let Token::Bool(b) = self {
            *b
        } else {
            panic!("tried to convert non-boolean token to boolean")
        }
    }
}

fn keyword(input: &str) -> IResult<&str, Token> {
    alt((
        tag("let").map(|_| Token::Let),
        tag("if").map(|_| Token::If),
        tag("print").map(|_| Token::Print),
    ))(input)
}

fn punctuation(input: &str) -> IResult<&str, Token> {
    alt((
        // It's very important that we put == before = so that we don't short
        // circuit and match == as two assignment tokens
        tag("==").map(|_| Token::Eq),
        tag("=").map(|_| Token::Assign),
        tag("(").map(|_| Token::LParen),
        tag(")").map(|_| Token::RParen),
        tag("{").map(|_| Token::LCurly),
        tag("}").map(|_| Token::RCurly),
        tag(">").map(|_| Token::GT),
        tag("+").map(|_| Token::Plus),
        alt((tag("\n"), tag("\r\n"))).map(|_| Token::Newline),
    ))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(tuple((
        satisfy(|c| c.is_ascii_alphabetic()),
        many0(satisfy(|c| c.is_ascii_alphanumeric())),
    )))(input)
}

fn keyword_or_identifier(input: &str) -> IResult<&str, Token> {
    let (input, result) = identifier(input)?;

    if let Ok((_, keyword)) = terminated(keyword, eof)(result) {
        Ok((input, keyword))
    } else {
        Ok((input, Token::Identifier(result.to_string())))
    }
}

fn int(input: &str) -> IResult<&str, Token> {
    let onenine = satisfy(|c| ('1'..='9').contains(&c));

    map_res(
        alt((tag("0"), recognize(tuple((onenine, digit0))))),
        |num: &str| num.parse::<i32>().map(Token::Int),
    )(input)
}

fn bool(input: &str) -> IResult<&str, Token> {
    alt((
        tag("true").map(|_| Token::Bool(true)),
        tag("false").map(|_| Token::Bool(false)),
    ))(input)
}

fn token(input: &str) -> IResult<&str, Token> {
    alt((int, bool, keyword_or_identifier, punctuation))(input)
}

pub fn token_stream(input: &str) -> Result<Vec<Token>, Error<&str>> {
    terminated(
        many0(delimited(
            many0(satisfy(|c| " \t".contains(c))),
            token,
            many0(satisfy(|c| " \t".contains(c))),
        )),
        eof,
    )(input)
    .finish()
    .map(|(_, o)| o)
}
