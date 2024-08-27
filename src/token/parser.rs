use super::expr::{ExprListType, Expr, ExprBase};
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

struct ListFrame {
    kind: ExprListType,

    list: Vec<Expr>,

    inv: Vec<Expr>,
}

/// Parser state
struct ParseState<'a> {
    pub file: &'a File,
    pub pos: Pos,

    pub frames: Vec<ListFrame>,
    pub curr: ListFrame,
}

impl<'a> ParseState<'a> {
    /// Create an error at the current position
    fn err(&self, msg: &str) -> ParseError {
        ParseError {
            filename: self.file.name.clone(),
            pos: self.pos.clone(),
            msg: format!("SyntaxError: {}", msg)
        }
    }

    fn expr(&self, base: ExprBase<Expr>) -> Expr {
        Expr {
            pos: self.pos.clone(),
            expr: base,
        }
    }

    /// Flush current inverted to the current list, in reverse order.
    fn flush_inv(&mut self) {
        while let Some(e) = self.curr.inv.pop() {
            self.curr.list.push(e);
        }
    }

    /// Return the closure character of the current frame type.
    fn closure_of_frame(&self) -> char {
        match self.curr.kind {
            ExprListType::Code => ')',
            ExprListType::Array => ']',
            ExprListType::Object => '}',
        }
    }

    /// Close the current frame.
    fn close_frame(&mut self) -> Result<(), ParseError> {
        // If no frame exists, return an error.
        if self.frames.len() == 0 {
            return Err(self.err(&"Unexpected closing parenthesis"));
        }

        let prev_frame = self.frames.pop().unwrap();

        // Flush inv first
        self.flush_inv();

        // Repalce the current frame with the previous frame.
        let old = std::mem::replace(&mut self.curr, prev_frame);

        // Pack the current frame.
        let e = self.expr(ExprBase::List(old.kind, old.list));

        // Push the packed expr to the inv
        self.curr.inv.push(e);

        Ok(())
    }

    /// Skip until the given string appears.
    /// If the string is found, return true.
    fn skip_until_string(&mut self, s: &str) -> bool {
        let mut i = 0;
        while let Some(c) = self.pos.char(&self.file.contents, 0) {
            if self.pos.eq_str(&self.file.contents, 0, s) {
                self.pos.next(&self.file.contents, s.len());
                return true;
            }
            self.pos.next(&self.file.contents, 1);
        }
        false
    }

    /// Skip comments.
    /// The first return is the comment correctly parsed,
    /// and the second return is newline included.
    fn skip_comments(&mut self) -> (bool, bool) {
        match self.pos.char(&self.file.contents, 1) {
            Some('/') => {
                // Line comment
                self.skip_until_string("\n");
                return (true, false)
            },
            Some('*') => {
                let old_ln = self.pos.line;
                // Block comment
                self.skip_until_string("*/");
                return (true, old_ln != self.pos.line)
            },
            _ => return (false, false),
        }
    }

    /// Skip ignores (whitespace, comma, comments, etc.)
    /// Return true if separator (newline or comma) was included.
    fn skip_ignores(&mut self) -> bool {
        let mut sep = false;
        while let Some(c) = self.pos.char(&self.file.contents, 0) {
            match c {
                '\n' | ',' => {
                    sep = true;
                    self.pos.next(&self.file.contents, 1);
                },
                '/' => {
                    // Check '//' and '/* */' which used for comments
                    let (skipped, newline) = self.skip_comments();
                    if skipped {
                        sep = newline;
                    } else {
                        break;
                    }
                },
                '\0'..' ' => {
                    self.pos.next(&self.file.contents, 1);
                },
                _ => break,
            }
        }
        sep
    }

    fn parse_once(&mut self) -> Result<Expr, ParseError> {
    }

    fn parse_string(&mut self) -> Result<Expr, ParseError> {
        self.skip_ignores();
    }
}

pub fn parse_string(f: &File) -> Result<Expr, ParseError> {
    let mut state = ParseState {
        file: f,
        pos: Pos { pos: 0, line: 0, col: 0 },
        frames: Vec::new(),
        curr: ListFrame {
            kind: ExprListType::Code,
            list: Vec::new(),
            inv: Vec::new(),
        },
    };
    state.parse_string()
}
