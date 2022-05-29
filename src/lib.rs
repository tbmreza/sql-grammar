use pest;
pub mod parser;
pub use pest::Parser;

#[cfg(feature = "common")]
mod common {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammars/common.pest"]
    pub struct SQLParser;
}

#[cfg(feature = "mysql")]
mod mysql {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "grammars/mysql.pest"]
    pub struct SQLParser;
}

#[cfg(feature = "common")]
pub use crate::common::*;

#[cfg(feature = "mysql")]
pub use crate::mysql::*;
