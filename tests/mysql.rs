use sql_grammar::*;

#[test]
#[ignore]
fn identifier() {
    let parsed = SQLParser::parse(Rule::file, "`employee`")
        .expect("unsuccessful parse")
        .next()
        .expect("pest failure");

    parsed.tokens().for_each(|x| println!("{:?}", x));
}

