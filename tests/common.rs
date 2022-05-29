use sql_grammar::*;

#[test]
#[ignore]
fn parser_infra() {
    let parsed = SQLParser::parse(Rule::file, "CREATE ")
        .expect("unsuccessful parse")
        .next()
        .expect("pest failure");

    parsed.tokens().for_each(|x| println!("{:?}", x));
}
