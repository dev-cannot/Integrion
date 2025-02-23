use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1, multispace0},
    combinator::{map, map_res, opt},
    multi::{many0, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
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
    Block(Vec<Expr>),
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map_res(digit1, |s: &str| s.parse::<i64>().map(Expr::Number))(input)
}

fn parse_variable(input: &str) -> IResult<&str, Expr> {
    map(alphanumeric1, |s: &str| Expr::Var(s.to_string()))(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), parse_expr, char(')'))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    alt((parse_number, parse_variable, parse_parens))(input)
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_factor(input)?;
    many0(pair(alt((char('*'), char('/'))), parse_factor))(input).map(|(input, ops)| {
        let expr = ops.into_iter().fold(init, |acc, (op, rhs)| {
            if op == '*' {
                Expr::Mul(Box::new(acc), Box::new(rhs))
            } else {
                Expr::Div(Box::new(acc), Box::new(rhs))
            }
        });
        (input, expr)
    })
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    let (input, init) = parse_term(input)?;
    many0(pair(alt((char('+'), char('-'))), parse_term))(input).map(|(input, ops)| {
        let expr = ops.into_iter().fold(init, |acc, (op, rhs)| {
            if op == '+' {
                Expr::Add(Box::new(acc), Box::new(rhs))
            } else {
                Expr::Sub(Box::new(acc), Box::new(rhs))
            }
        });
        (input, expr)
    })
}

fn parse_assignment(input: &str) -> IResult<&str, Expr> {
    let (input, (var, expr)) = separated_pair(alphanumeric1, char('='), parse_expr)(input)?;
    Ok((input, Expr::Assign(var.to_string(), Box::new(expr))))
}

fn parse_statement(input: &str) -> IResult<&str, Expr> {
    alt((parse_assignment, parse_expr))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Expr> {
    let (input, statements) = separated_list0(char(';'), parse_statement)(input)?;
    Ok((input, Expr::Block(statements)))
}
