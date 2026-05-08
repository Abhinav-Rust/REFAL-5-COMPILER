
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    SVar(String),
    TVar(String),
    EVar(String),
    Ident(String),
    Number(i64),
    StringLit(String),
    Bracket(Vec<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub pattern: Vec<Expr>,
    pub conditions: Vec<Condition>,
    pub result: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub pattern: Vec<Expr>,
    pub result: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub name: String,
    pub is_entry: bool,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<Function>,
}
