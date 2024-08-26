use super::file::Pos;

pub enum ExprBase<E> {
    /// Identifier
    Id(String),

    /// Symbol literal
    Sym(String),

    /// Integer literal
    Int(i64),

    /// Float literal
    Float(f64),

    /// String literal
    String(String),

    /// Regular expression literal
    Regexp(String),

    /// Function
    Fn(Vec<E>),

    /// Array
    Array(Vec<E>),

    /// Object
    Object(Vec<(String, E)>),

    /// Comma / newline separator
    Comma,
}

pub struct Expr {
    pub pos: Pos,
    pub expr: ExprBase<Expr>,
}
