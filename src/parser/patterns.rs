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
    rule_name().convert(String::from_utf8) - space() + rule_expr().convert(to_expr)
}

pub fn rule_name_expr_cand<'a>() -> Parser<'a, u8, (String, Expr)> {
    rule_name().convert(String::from_utf8) - space() + rule_expr_cand().convert(to_expr_choice)
}

// instead return Expr::Choice ?
fn to_expr_choice(parser_output: (Vec<Expr>, Expr)) -> Result<Expr, Box<dyn std::error::Error>> {
    let (mut expr_units, e) = parser_output;
    expr_units.push(e);

    let res = construct_choice(expr_units);
    Ok(res)
}

fn to_expr(parser_output: (Vec<Vec<u8>>, Vec<u8>)) -> Result<Expr, Box<dyn std::error::Error>> {
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

#[test]
fn test_construct() {
    let f = vec![97];
    let r = vec![vec![98], vec![99], vec![100]];

    assert_eq!(
        String::from("\"a\" | \"b\" | \"c\" | \"d\""),
        construct(f, r).to_string()
    );
}

fn to_expr_name(mut parser_output: Vec<u8>) -> Result<Expr, Box<dyn std::error::Error>> {
    parser_output.insert(0, b'<');
    parser_output.push(b'>');

    Ok(construct(parser_output, Vec::new()))
}

fn is_surrounded(s: &String) -> bool {
    let trimmed = s.trim();
    trimmed.starts_with('<') && trimmed.ends_with('>')
}

fn terminal(bytes: &[u8]) -> Option<Expr> {
    match String::from_utf8(bytes.to_owned()) {
        Ok(s) if is_surrounded(&s) => Some(Expr::Name(s)),
        Ok(s) => Some(Expr::Str(s)),
        Err(_) => None,
    }
}

/// If <> surrounded, Expr::Name. Else, Expr::Str
fn construct(first: Vec<u8>, rest: Vec<Vec<u8>>) -> Expr {
    match rest.len() {
        0 => terminal(trim(&first).as_bytes()).unwrap_or_default(),
        1 => {
            let last = rest.get(0).unwrap();
            let first_as_expr = terminal(trim(&first).as_bytes()).unwrap_or_default();
            let last_as_expr = terminal(trim(&last).as_bytes()).unwrap_or_default();
            Expr::Choice(Box::new(first_as_expr), Box::new(last_as_expr))
        }
        _ => {
            let (car, cdr) = rest.split_first().unwrap();
            let first_as_expr = terminal(trim(&first).as_bytes()).unwrap_or_default();
            Expr::Choice(
                Box::new(first_as_expr),
                Box::new(construct(car.to_owned(), cdr.to_owned())),
            )
        }
    }
}

// TODO
fn construct_choice(exprs: Vec<Expr>) -> Expr {
    // match rest.len() {
    //     0 => terminal(trim(&first).as_bytes()).unwrap_or_default(),
    //     1 => {
    //         let last = rest.get(0).unwrap();
    //         let first_as_expr = terminal(trim(&first).as_bytes()).unwrap_or_default();
    //         let last_as_expr = terminal(trim(&last).as_bytes()).unwrap_or_default();
    //         Expr::Choice(Box::new(first_as_expr), Box::new(last_as_expr))
    //     }
    //     _ => {
    //         let (car, cdr) = rest.split_first().unwrap();
    //         let first_as_expr = terminal(trim(&first).as_bytes()).unwrap_or_default();
    //         Expr::Choice(
    //             Box::new(first_as_expr),
    //             Box::new(construct(car.to_owned(), cdr.to_owned())),
    //         )
    //     }
    // }
    Expr::Choice(
        Box::new(Expr::default()),
        Box::new(Expr::default()),
    )
}

// TODO either Expr::Name or raw Expr::Str
// "   12333 |"
pub fn rule_name_cand<'a>() -> Parser<'a, u8, Vec<u8>> {
    sym(b'<') * none_of(b">").repeat(1..) - sym(b'>')
}

/// <some rule> |
pub fn expr_pipe<'a>() -> Parser<'a, u8, Expr> {
    space() * rule_name_cand().convert(to_expr_name) - space() - sym(b'|')
}
#[test]
fn qq() {
    let inputs = vec!["   <qq mark> |"];
    for input in inputs {
        if let Ok(s) = expr_pipe().parse(input.as_bytes()) {
            println!("{:?}", s);
        }
    }

    let inputs = vec!["::= <some satu> | <some dua> | <some tiga>"];
    for input in inputs {
        if let Ok(s) = rule_expr_cand().parse(input.as_bytes()) {
            println!("{:?}", s);
        }
    }

}

/// <some rule> | <some rule>
/// The assignment operator followed by expr-pipe that repeats zero or more times followed
/// by an expr.
pub fn rule_expr_cand<'a>() -> Parser<'a, u8, (Vec<Expr>, Expr)> {
    seq(b"::=") * space() * expr_pipe().repeat(0..) - space() + rule_name_cand().convert(to_expr_name)
}

pub fn trailing_pipe<'a>() -> Parser<'a, u8, Vec<u8>> {
    space() * none_of(b"|").repeat(0..) - space() - sym(b'|')
}
