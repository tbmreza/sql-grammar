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

#[test]
fn expr_string() {
    use super::*;
    let input = b"::=  { $ }";
    let output = rule_expr().convert(String::from_utf8).parse(input);
    let res = match output {
        Ok(s) => s,
        _ => String::new(),
    };
    assert_eq!(res, String::from("{ $ }"));
}

use pom::parser::*;

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

fn rule_name<'a>() -> Parser<'a, u8, Vec<u8>> {
    sym(b'<') * none_of(b">").repeat(1..) - sym(b'>')
}

// The assignment operator and everything that comes after it.
fn rule_expr<'a>() -> Parser<'a, u8, Vec<u8>> {
    seq(b"::=") * one_of(b" \t\r\n").repeat(0..).discard() * any().repeat(0..)
}

fn rule_name_expr<'a>() -> Parser<'a, u8, (String, String)> {
    rule_name().convert(String::from_utf8) - space() + rule_expr().convert(String::from_utf8)
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
