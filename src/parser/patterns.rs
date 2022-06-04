use crate::parser::ast::Expr;
use pom::parser::*;

pub fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn rule_name<'a>() -> Parser<'a, u8, Vec<u8>> {
    sym(b'<') * none_of(b">").repeat(1..) - sym(b'>')
}

/// The assignment operator and everything that comes after it.
pub fn rule_expr<'a>() -> Parser<'a, u8, (Vec<Vec<u8>>, Vec<u8>)> {
    seq(b"::=") * space() * trailing_pipe().repeat(0..) - space() + none_of(b"|").repeat(0..)
}

pub fn rule_name_expr<'a>() -> Parser<'a, u8, (String, Expr)> {
    rule_name().convert(String::from_utf8) - space() + rule_expr().convert(converter)
}

fn converter(parser_output: (Vec<Vec<u8>>, Vec<u8>)) -> Result<Expr, Box<dyn std::error::Error>> {
    let (mut expr_units, e) = parser_output;
    expr_units.push(e);

    let (first, rest) = expr_units.split_first().unwrap();
    let expr_list = construct(first.to_owned(), rest.to_owned());
    Ok(expr_list)
}

// TODO? Automatic space() insertion in between Parsers should render this
// function unnecessary. I wrote this function only because I don't know where
// the trailing spaces in the output are coming from.
fn trim(bytes: &[u8]) -> String {
    match String::from_utf8(bytes.to_owned()) {
        Ok(s) => (&s.trim()).to_string(),
        _ => String::from(""),
    }
}

fn construct(first: Vec<u8>, rest: Vec<Vec<u8>>) -> Expr {
    match rest.len() {
        0 => Expr::Str(trim(&first)),
        1 => {
            let last = rest.get(0).unwrap();
            let first_as_expr = Expr::Str(trim(&first));
            let last_as_expr = Expr::Str(trim(last));
            Expr::Choice(Box::new(first_as_expr), Box::new(last_as_expr))
        }
        _ => {
            let (car, cdr) = rest.split_first().unwrap();
            let first_as_expr = Expr::Str(trim(&first));
            Expr::Choice(
                Box::new(first_as_expr),
                Box::new(construct(car.to_owned(), cdr.to_owned())),
            )
        }
    }
}

#[test]
fn test_construct() {
    let f = vec![97];
    let r = vec![vec![98], vec![99], vec![100]];

    assert_eq!(
        String::from("\"a\" | \"b\" | \"c\" | \"d\""),
        construct(f, r).to_string()
    );
}

pub fn trailing_pipe<'a>() -> Parser<'a, u8, Vec<u8>> {
    space() * none_of(b"|").repeat(0..) - space() - sym(b'|')
}
