// src/parser.rs

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, multispace0, char},
    combinator::opt,
    sequence::{delimited, tuple, separated_pair},
};

#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Var(String),
    Assign(String, Box<Expr>),
}

// Parse an identifier (variable name) as a sequence of alphabetic characters.
pub fn parse_identifier(input: &str) -> IResult<&str, String> {
    let (input, ident) = alpha1(input)?;
    Ok((input, ident.to_string()))
}

// Parse an integer number.
pub fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, num_str) = digit1(input)?;
    let number = num_str.parse::<i64>().unwrap();
    Ok((input, Expr::Number(number)))
}

// Parse a factor: either a number or an expression in parentheses.
pub fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_number,
        delimited(
            delimited(multispace0, tag("("), multispace0),
            parse_expr, // Calls the top-level parser.
            delimited(multispace0, tag(")"), multispace0)
        )
    ))(input)
}

// Parse a term: a factor possibly followed by multiplication or division operations.
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

// Parse arithmetic expressions: a term followed by addition or subtraction operations.
pub fn parse_arithmetic_expr(input: &str) -> IResult<&str, Expr> {
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

// Parse an assignment: identifier '=' expression.
pub fn parse_assignment(input: &str) -> IResult<&str, Expr> {
    let (input, (ident, expr)) = separated_pair(
        parse_identifier,
        delimited(multispace0, char('='), multispace0),
        parse_expr
    )(input)?;
    Ok((input, Expr::Assign(ident, Box::new(expr))))
}

// Parse a variable usage: a standalone identifier that becomes a variable.
pub fn parse_variable(input: &str) -> IResult<&str, Expr> {
    let (input, ident) = parse_identifier(input)?;
    Ok((input, Expr::Var(ident)))
}

// Top-level expression parser: first tries to parse an assignment,
// then an arithmetic expression, and finally a variable usage.
pub fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_assignment, parse_arithmetic_expr, parse_variable))(input)
}
