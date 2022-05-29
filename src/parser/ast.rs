use std::fmt;

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub ty: RuleType,
    pub expr: Expr,
}

// display in pest manner i.e. snake_case names. how do you express this in the code?
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {{ {} }}", self.name.replace(" ", "_"), self.expr)
    }
}

#[derive(Debug)]
pub enum RuleType {
    Normal,
}

#[derive(Debug)]
pub enum Expr {
    Str(String),
    Seq(Box<Expr>, Box<Expr>),
    Choice(Box<Expr>, Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Expr::*;
        match self {
            Str(s) => write!(f, "\"{}\"", s),
            Seq(car, cdr) => {
                write!(f, "{} ~ {}", car, cdr)
            }
            Choice(car, cdr) => {
                write!(f, "{} | {}", car, cdr)
            }
        }
    }
}
