use sql_grammar::*;

#[test]
fn identifier() {
    let parsed = SQLParser::parse(Rule::file, "`employee`")
        .expect("unsuccessful parse")
        .next()
        .expect("pest failure");

    parsed.tokens().for_each(|x| println!("{:?}", x));
}

