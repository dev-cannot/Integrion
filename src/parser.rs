#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, opt},
    multi::many0,
    sequence::{delimited, preceded},
    IResult,
};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Var(String),
    Assign(String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>),
    While(Box<Expr>, Box<Expr>),
    Block(Vec<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, |s: &str| s.parse::<i64>().map(Expr::Number))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    map(alphanumeric1, |s: &str| Expr::Ident(s.to_string()))(input)
}

fn parse_parenthesized_expr(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr, char(')'))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    alt((parse_number, parse_identifier, parse_parenthesized_expr))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_term(input)?;
    let (input, op) = opt(alt((char('*'), char('/'))))(input)?;
    match op {
        Some('*') => {
            let (input, right) = parse_factor(input)?;
            Ok((input, Expr::Mul(Box::new(left), Box::new(right))))
        }
        Some('/') => {
            let (input, right) = parse_factor(input)?;
            Ok((input, Expr::Div(Box::new(left), Box::new(right))))
        }
        _ => Ok((input, left)),
    }
}

fn parse_arithmetic_expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_factor(input)?;
    let (input, op) = opt(alt((char('+'), char('-'))))(input)?;
    match op {
        Some('+') => {
            let (input, right) = parse_arithmetic_expr(input)?;
            Ok((input, Expr::Add(Box::new(left), Box::new(right))))
        }
        Some('-') => {
            let (input, right) = parse_arithmetic_expr(input)?;
            Ok((input, Expr::Sub(Box::new(left), Box::new(right))))
        }
        _ => Ok((input, left)),
    }
}

fn parse_block(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = delimited(
        preceded(multispace0, char('{')),
        many0(delimited(multispace0, parse_expr, multispace0)),
        preceded(multispace0, char('}'))
    )(input)?;
    Ok((input, Expr::Block(exprs)))
}

fn parse_if_else(input: &str) -> IResult<&str, Expr> {
    let (input, _) = preceded(multispace0, tag("if"))(input)?;
    let (input, condition) = parse_expr(input)?;
    let (input, then_block) = parse_block(input)?;
    let (input, else_block) = opt(preceded(multispace0, preceded(tag("else"), parse_block)))(input)?;
    Ok((input, Expr::If(Box::new(condition), Box::new(then_block), else_block.map(Box::new))))
}

fn parse_while(input: &str) -> IResult<&str, Expr> {
    let (input, _) = preceded(multispace0, tag("while"))(input)?;
    let (input, condition) = parse_expr(input)?;
    let (input, body) = parse_block(input)?;
    Ok((input, Expr::While(Box::new(condition), Box::new(body))))
}

/// Top-level expression parser.
/// It tries to parse control structures first; if none match,
/// it falls back to arithmetic expressions.
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_if_else,
        parse_while,
        // For assignments, let’s require them to be part of a statement,
        // but if needed, you could add a separate parser.
        parse_arithmetic_expr,
        parse_block,
        parse_identifier // As a fallback, treat an identifier as a variable lookup.
    ))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Expr> {
    let (input, exprs) = many0(delimited(multispace0, parse_expr, multispace0))(input)?;
    Ok((input, Expr::Block(exprs)))
}
