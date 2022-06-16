use crate::parser::ast::{Rule, RuleType};
use crate::parser::patterns::*;

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

pub fn parse_rule(input: &[u8]) -> Option<Rule> {
    let parsed = rule_name_expr().parse(input);
    // let parsed = rule_name_expr_cand().parse(input);

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
