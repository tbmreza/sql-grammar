use pest;
pub use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammars/common.pest"]
pub struct SQLParser;
