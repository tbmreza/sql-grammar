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
    let expr_units = match parser_output {
        (mut ls, e) => {
            ls.push(e);
            ls
        }
    };
    let (first, rest) = expr_units.split_first().unwrap();
    let expr_list = construct(first.to_owned(), rest.to_owned());
    Ok(expr_list)
}

fn construct(first: Vec<u8>, rest: Vec<Vec<u8>>) -> Expr {
    match rest.len() {
        0 => Expr::Str(String::from_utf8(first).unwrap()),
        1 => {
            let last = rest.get(0).unwrap();
            let first_as_expr = Expr::Str(String::from_utf8(first).unwrap());
            let last_as_expr = Expr::Str(String::from_utf8(last.to_owned()).unwrap());
            Expr::Choice(Box::new(first_as_expr), Box::new(last_as_expr))
        }
        _ => {
            let (car, cdr) = rest.split_first().unwrap();
            let first_as_expr = Expr::Str(String::from_utf8(first).unwrap());
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
    none_of(b"|").repeat(0..) - space() - sym(b'|')
}
