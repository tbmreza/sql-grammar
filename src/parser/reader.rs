use crate::parser::ast::{Expr, Rule, RuleType};

#[test]
fn hello_pom() {
    use pom::parser::*;

    // let input = b"abcde";
    // let parser = sym(b'a') * none_of(b"AB") - sym(b'c') + seq(b"de");
    // let output = parser.parse(input);
    // assert_eq!(output, Ok((b'b', vec![b'd', b'e'].as_slice())));

    // let input = b"bde";
    // let parser = none_of(b"AB") + seq(b"de");
    // let output = parser.parse(input);
    // assert_eq!(output, Ok((b'b', vec![b'd', b'e'].as_slice())));

    // let input = b"<>";
    // let parser = sym(b'<') + sym(b'>');
    // let output = parser.parse(input);
    // assert_eq!(output, Ok((b'<', b'>')));

    // let input = b"<nnnn>";
    // let parser = sym(b'<') * none_of(b">").repeat(0..) - sym(b'>');
    // let output = parser.parse(input);
    // assert_eq!(output, Ok(vec![b'n', b'n', b'n', b'n']));

    let input = b"<name bisa>";
    let name = sym(b'<') * none_of(b">").repeat(1..) - sym(b'>');
    let parser = name.convert(String::from_utf8);
    let output = parser.parse(input);
    assert_eq!(output, Ok(String::from("name bisa")));
}

#[test]
fn underscored_name() {
    let input = b"<exclam mark>";
    let output = rule_name().convert(String::from_utf8).parse(input);
    let res = match output {
        Ok(s) => s.replace(" ", "_"),
        _ => String::new(),
    };
    assert_eq!(res, String::from("exclam_mark"));
}

#[test]
fn ada_sisa() {
    let input = b"<exclam markjjj> invis";
    let output = rule_name().convert(String::from_utf8).parse(input);
}

use pom::parser::*;

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn rule_name<'a>() -> Parser<'a, u8, Vec<u8>> {
    sym(b'<') * none_of(b">").repeat(1..) - sym(b'>')
}

/// The assignment operator and everything that comes after it.
fn rule_expr<'a>() -> Parser<'a, u8, Vec<u8>> {
    seq(b"::=") * space() * none_of(b"|").repeat(0..)
}

fn rule_expr_cand<'a>() -> Parser<'a, u8, (Vec<Vec<u8>>, Vec<u8>)> {
    seq(b"::=") * space() * trailing_pipe().repeat(0..) - space() + none_of(b"|").repeat(0..)
}
#[test]
fn cand() {
    let input = b"::= 1 | 2 | 3";
    let output = rule_expr_cand().parse(input);

    let flat = match output {
        Ok((mut ls, e)) => {
            ls.push(e);
            ls
        }
        _ => Vec::new(),
    };

    println!("{:?}", flat);
}

fn construct(first: Vec<u8>, rest: Vec<Vec<u8>>) -> Expr {
    match rest.len() {
        0 => {
            Expr::Str(String::from_utf8(first).unwrap())
        }
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

fn rule_name_expr_cand<'a>() -> Parser<'a, u8, (String, Expr)> {
    rule_name().convert(String::from_utf8) - space() + rule_expr_cand().convert(converter)
}

fn rule_name_expr<'a>() -> Parser<'a, u8, (String, String)> {
    rule_name().convert(String::from_utf8) - space() + rule_expr().convert(String::from_utf8)
}

fn trailing_pipe<'a>() -> Parser<'a, u8, Vec<u8>> {
    none_of(b"|").repeat(0..) - space() - sym(b'|')
}

pub fn parse_rule(input: &[u8]) -> Option<Rule> {
    let parsed = rule_name_expr().parse(input);

    match parsed {
        Ok((name, expr)) => {
            let res = Rule {
                name,
                ty: RuleType::Normal,
                expr: Expr::Str(expr),
            };
            Some(res)
        }
        _ => None,
    }
}
pub fn parse_rule_cand(input: &[u8]) -> Option<Rule> {
    let parsed = rule_name_expr_cand().parse(input);

    match parsed {
        Ok((name, expr)) => {
            let res = Rule {
                name,
                ty: RuleType::Normal,
                expr,
            };
            Some(res)
        }
        _ => None,
    }
}
