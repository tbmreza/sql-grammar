use sql_grammar::parser::reader::*;

#[cfg(test)]
mod tests {
    use sql_grammar::parser::ast::*;
    use super::*;
    #[test]
    fn from_file() {
        use std::fs::{File, OpenOptions};
        use std::io::prelude::*;
        use std::io::BufReader;
        use std::path::Path;

        let inp = File::open(Path::new("input.bnf")).expect("file in the same path as binary");

        let mut out = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("output.pest")
            .expect("std builder doesn't fail");

        for line in BufReader::new(inp).lines() {
            let pest_line = parse_rule(line.unwrap().as_bytes()).unwrap();

            if let Err(e) = writeln!(out, "{}", pest_line) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
    #[test]
    fn simple() {
        let node = Rule {
            name: String::from("question_mark"),
            ty: RuleType::Normal,
            expr: Expr::Str(String::from("?")),
        };

        assert_eq!(node.to_string(), String::from(r#"question_mark = { "?" }"#));
    }
    #[test]
    fn choice_digit() {
        // NOTE The parser side decides how the list is actually constructed.
        // I'm thinking (cons 1 (cons 2 nil)) form.
        let boxed_6 = Box::new(Expr::Str(String::from("6")));
        let boxed_7 = Box::new(Expr::Str(String::from("7")));
        let boxed_8 = Box::new(Expr::Str(String::from("8")));
        let boxed_9 = Box::new(Expr::Str(String::from("9")));
        let l89 = Expr::Choice(boxed_8, boxed_9);
        let l789 = Expr::Choice(boxed_7, Box::new(l89));
        let l6789 = Expr::Choice(boxed_6, Box::new(l789));
        let node = Rule {
            name: String::from("digit"),
            ty: RuleType::Normal,
            expr: l6789,
        };
        assert_eq!(
            node.to_string(),
            String::from(r#"digit = { "6" | "7" | "8" | "9" }"#)
        );
    }
    #[test]
    fn seq_simple() {
        let keyword_drop = Box::new(Expr::Str(String::from("drop")));
        let keyword_table = Box::new(Expr::Str(String::from("table")));
        let node = Rule {
            name: String::from("drop_table"),
            ty: RuleType::Normal,
            expr: Expr::Seq(keyword_drop, keyword_table),
        };
        assert_eq!(
            node.to_string(),
            String::from(r#"drop_table = { "drop" ~ "table" }"#)
        );
    }
}
fn main() {
    println!("{}", parse_rule(b"<digit> ::= 1 | 2").unwrap());
}
