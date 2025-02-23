// src/parser.rs
use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, multispace0},
    combinator::{map_res, opt},
    sequence::{delimited, tuple},
};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

// Parse an integer number
pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, num_str) = digit1(input)?;
    let number = num_str.parse::<i64>().unwrap();
    Ok((input, Expr::Number(number)))
}

// Parse a factor: a number or an expression in parentheses
pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_number,
        delimited(
            delimited(multispace0, tag("("), multispace0),
            parse_expr,
            delimited(multispace0, tag(")"), multispace0)
        )
    ))(input)
}

// Parse term: factor followed by multiplication or division operations
pub fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    let mut result = init;
    let mut rest = input;
    
    loop {
        let (next_input, op_opt) = opt(tuple((
            delimited(multispace0, alt((tag("*"), tag("/"))), multispace0),
            parse_factor
        )))(rest)?;
        
        match op_opt {
            Some((op, next_expr)) => {
                result = match op {
                    "*" => Expr::Mul(Box::new(result), Box::new(next_expr)),
                    "/" => Expr::Div(Box::new(result), Box::new(next_expr)),
                    _ => result,
                };
                rest = next_input;
            },
            None => break,
        }
    }
    Ok((rest, result))
}

// Parse expression: term followed by addition or subtraction
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    let mut result = init;
    let mut rest = input;
    
    loop {
        let (next_input, op_opt) = opt(tuple((
            delimited(multispace0, alt((tag("+"), tag("-"))), multispace0),
            parse_term
        )))(rest)?;
        
        match op_opt {
            Some((op, next_expr)) => {
                result = match op {
                    "+" => Expr::Add(Box::new(result), Box::new(next_expr)),
                    "-" => Expr::Sub(Box::new(result), Box::new(next_expr)),
                    _ => result,
                };
                rest = next_input;
            },
            None => break,
        }
    }
    Ok((rest, result))
}
