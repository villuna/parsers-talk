//! parser.rs
//!
//! A parser that turns a stream of tokens into an abstract syntax tree.
//!
//! Parsing a stream of tokens is much easier than parsing a big blob of text,
//! so we write two parsers to get the job done. This is the second,
//! and it defines the structure of the language and parses it into an abstract
//! syntax tree which can then be interpreted or compiled.

use nom::{
    branch::alt,
    combinator::eof,
    multi::many0,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Finish, IResult, Parser,
};

use crate::{
    ast::{Ast, BinaryOperation, Expression, Statement},
    token::Token,
};

// Aioi Lang - grammar
//
// program ::= block
//
// endl ::= NEWLINE {NEWLINE}
//
// block ::= {NEWLINE} {statement}
//
// statement ::= declaration | if_statement | print
//
// declaration ::= LET IDENTIFIER ASSIGN expression endl
//
// print ::= PRINT expression endl
//
// if_statement ::= IF expression LCURLY block RCURLY
//
// expression ::= binary_op | term
//
// term ::= LPAREN expression RPAREN | IDENTIFIER | INT | BOOL
//
// binary_op ::= term BINOP expression

#[derive(PartialEq, Eq, Debug)]
pub enum ErrorKind {
    NomError(nom::error::ErrorKind),
    Satisfies,
}

#[derive(PartialEq, Eq, Debug)]
pub struct AioiParseError<'a> {
    input: &'a [Token],
    kind: ErrorKind,
}

impl<'a> nom::error::ParseError<&'a [Token]> for AioiParseError<'a> {
    fn from_error_kind(input: &'a [Token], kind: nom::error::ErrorKind) -> Self {
        Self {
            input,
            kind: ErrorKind::NomError(kind),
        }
    }

    fn append(_input: &'a [Token], _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

type TLResult<'a, O> = IResult<&'a [Token], O, AioiParseError<'a>>;

// -- Some helper functions / combinators --

/// If the next token is a binary operator, returns its type
fn binop(input: &[Token]) -> TLResult<BinaryOperation> {
    alt((
        token(Token::GT).map(|_| BinaryOperation::GreaterThan),
        token(Token::Eq).map(|_| BinaryOperation::EqualTo),
        token(Token::Plus).map(|_| BinaryOperation::Add),
    ))(input)
}

/// Returns the next token if it is equal to the argument
fn token(token: Token) -> impl FnMut(&[Token]) -> TLResult<&Token> {
    tsatisfies(move |t| *t == token)
}

/// Returns the next token if it satisfies a predicate
fn tsatisfies<P>(mut predicate: P) -> impl FnMut(&[Token]) -> TLResult<&Token>
where
    P: FnMut(&Token) -> bool,
{
    move |input| {
        if let Some(true) = input.get(0).map(|t| predicate(t)) {
            Ok((&input[1..], &input[0]))
        } else {
            Err(nom::Err::Error(AioiParseError {
                input,
                kind: ErrorKind::Satisfies,
            }))
        }
    }
}

// -- AST parsing functions --

/// binary_op ::= term BINOP expression
fn binary_operation(input: &[Token]) -> TLResult<Expression> {
    // This could be a map but i think this is nicer looking actually
    let (input, (value1, operation, value2)) = tuple((term, binop, expression))(input)?;

    Ok((
        input,
        Expression::BinaryOperation(operation, Box::new(value1), Box::new(value2)),
    ))
}

/// If the next token is an identifier, returns its name.
fn identifier(input: &[Token]) -> TLResult<String> {
    tsatisfies(|t| matches!(t, Token::Identifier(_)))
        .map(|t| t.identifier())
        .parse(input)
}

/// term ::= LPAREN expression RPAREN | IDENTIFIER | INT | BOOL
fn term(input: &[Token]) -> TLResult<Expression> {
    alt((
        delimited(token(Token::LParen), expression, token(Token::RParen)),
        identifier.map(|s| Expression::Variable(s)),
        tsatisfies(|t| matches!(t, Token::Int(_))).map(|t| Expression::Int(t.int())),
        tsatisfies(|t| matches!(t, Token::Bool(_))).map(|t| Expression::Bool(t.bool())),
    ))(input)
}

/// expression ::= binary_op | term
fn expression(input: &[Token]) -> TLResult<Expression> {
    alt((binary_operation, term))(input)
}

/// endl ::= NEWLINE {NEWLINE}
fn endl(mut input: &[Token]) -> TLResult<()> {
    (input, _) = token(Token::Newline)(input)?;

    while let Ok((i, _)) = token(Token::Newline)(input) {
        input = i;
    }

    Ok((input, ()))
}

/// declaration ::= LET IDENTIFIER ASSIGN expression endl
fn declaration(input: &[Token]) -> TLResult<Statement> {
    let (input, (name, rvalue)) = delimited(
        token(Token::Let),
        separated_pair(identifier, token(Token::Assign), expression),
        endl,
    )(input)?;

    Ok((input, Statement::Declaration { id: name, rvalue }))
}

/// if_statement ::= IF expression LCURLY block RCURLY
fn if_statement(input: &[Token]) -> TLResult<Statement> {
    let (input, (condition, block)) = preceded(
        token(Token::If),
        pair(
            expression,
            delimited(token(Token::LCurly), block, token(Token::RCurly)),
        ),
    )(input)?;

    Ok((input, Statement::IfStatement { condition, block }))
}

/// print ::= PRINT expression endl
fn print_statement(input: &[Token]) -> TLResult<Statement> {
    let (input, value) = delimited(token(Token::Print), expression, endl)(input)?;

    Ok((input, Statement::Print { value }))
}

/// statement ::= declaration | if_statement | print
fn statement(input: &[Token]) -> TLResult<Statement> {
    terminated(
        alt((declaration, if_statement, print_statement)),
        many0(token(Token::Newline)),
    )(input)
}

/// block ::= {NEWLINE} {statement}
fn block(input: &[Token]) -> TLResult<Vec<Statement>> {
    preceded(many0(token(Token::Newline)), many0(statement))(input)
}

/// program ::= block
pub fn parse_program(input: &[Token]) -> Result<Ast, AioiParseError> {
    terminated(block.map(|statements| Ast { statements }), eof)(input)
        .finish()
        .map(|(_, o)| o)
}
