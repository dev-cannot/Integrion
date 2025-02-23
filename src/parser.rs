#![allow(dead_code)]

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    combinator::{map, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
    Block(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Expr>),
    FuncDef(String, Vec<String>, Box<Expr>),
    FuncCall(String, Vec<Expr>),
    Lambda(Vec<String>, Box<Expr>),
    ClassDef(String, Vec<(String, Expr)>),
    MethodCall(Box<Expr>, String, Vec<Expr>),
}

// Parses variable/function names
fn parse_ident(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alphanumeric1(input)?;
    Ok((input, Expr::Ident(name.to_string())))
}

// Parses a block of expressions { expr; expr; }
fn parse_block(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = delimited(
        char('{'),
        many0(preceded(multispace0, parse_expr)),
        char('}')
    )(input)?;

    Ok((input, Expr::Block(exprs)))
}

// Parses function definitions
fn parse_func_def(input: &str) -> IResult<&str, Expr> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = alphanumeric1(input)?;
    let (input, params) = delimited(char('('), separated_list0(char(','), alphanumeric1), char(')'))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = parse_block(input)?;

    Ok((input, Expr::FuncDef(name.to_string(), params.iter().map(|p| p.to_string()).collect(), Box::new(body))))
}

// Parses function calls
fn parse_func_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = alphanumeric1(input)?;
    let (input, args) = delimited(char('('), separated_list0(char(','), parse_expr), char(')'))(input)?;

    Ok((input, Expr::FuncCall(name.to_string(), args)))
}

// Parses expressions
fn parse_expr(input: &str) -> IResult<&str, Expr> {
    parse_func_def(input).or_else(|_| parse_func_call(input)).or_else(|_| parse_ident(input))
}

pub fn parse_program(input: &str) -> IResult<&str, Expr> {
    parse_expr(input)
}
