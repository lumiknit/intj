use super::expr::Expr;
use super::file::{File, Pos};

/// Parser error.
#[derive(Debug)]
pub struct ParseError {
    pub filename: String,
    pub pos: Pos,
    pub msg: String,
}

impl ParseError {
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}: {}", self.filename, self.pos.line, self.pos.col, self.msg)
    }
}

/// Parser state
struct ParseState<'a> {
    pub file: &'a File,
    pub pos: Pos,
}

impl<'a> ParseState<'a> {
    /// Create an error at the current position
    fn err(&self) -> ParseError {
        ParseError {
            filename: self.file.name.clone(),
            pos: self.pos.clone(),
            msg: "Syntax error".to_string(),
        }
    }

    fn skip_whites(&mut self) -> bool {
    }

    fn parse_string(&mut self) -> Result<Expr, ParseError> {
        unimplemented!("parse_string")
    }
}

pub fn parse_string(f: &File) -> Result<Expr, ParseError> {
    let mut state = ParseState {
        file: f,
        pos: Pos { pos: 0, line: 0, col: 0 },
    };
    state.parse_string()
}
