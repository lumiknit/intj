use super::{expr::*, file::File, file::Pos};

use crate::utils;
use std::vec::*;

// Char set

type CharSet = [bool; 127];

const fn new_char_set(bytes: &[u8]) -> CharSet {
	let mut arr = [false; 127];
	let mut i = 0;
	while i < bytes.len() {
		let idx = bytes[i] as usize;
		if idx < 127 {
			arr[idx] = true;
		}
		i += 1;
	}

	arr
}

const fn new_range_set(start: u8, end: u8) -> CharSet {
	let mut arr = [false; 127];
	let mut i = start;
	while i <= end {
		arr[i as usize] = true;
		i += 1;
	}
	arr
}

const fn union_char_set(a: CharSet, b: CharSet) -> CharSet {
	let mut arr = [false; 127];
	let mut i = 0;
	while i < 127 {
		arr[i] = a[i] || b[i];
		i += 1;
	}
	arr
}

const SEPARATORS: CharSet = new_char_set(b"\n,");
const WHITES: CharSet = union_char_set(SEPARATORS, new_range_set(0, 32));
const RESERVED: CharSet = union_char_set(WHITES, new_char_set(b"()[]{}`'\"#:"));
const OP_CHARS: CharSet = new_char_set(b"~!@$%^&*-+=|\\/<>?");
const NON_ID_CHARS: CharSet = union_char_set(RESERVED, OP_CHARS);

// Parse error

pub struct ParseError {
	pub name: String,
	pub pos: Pos,
	pub message: String,
}

type ParseResult<T> = Result<T, ParseError>;

impl ParseError {
	pub fn to_string(&self) -> String {
		format!(
			"{}:{}:{}: {}",
			self.name, self.pos.line, self.pos.col, self.message
		)
	}
}

// Parse state

pub struct ParseState<'src> {
	pub name: String,
	pub src: &'src [u8],
	pub pos: Pos,
}

impl<'src> ParseState<'src> {
	pub fn new(name: String, src: &'src str) -> Self {
		Self {
			name,
			src: src.as_bytes(),
			pos: Pos {
				pos: 0,
				line: 1,
				col: 1,
			},
		}
	}

	pub fn err(&self, message: &str) -> ParseError {
		ParseError {
			name: self.name.clone(),
			pos: self.pos.clone(),
			message: message.to_string(),
		}
	}

	pub fn err_result<T>(&self, message: &str) -> Result<T, ParseError> {
		Err(self.err(message))
	}

	pub fn is_eof(&self) -> bool {
		self.pos.pos >= self.src.len()
	}

	pub fn skip(&mut self, n: usize) {
		let n = std::cmp::min(n, self.src.len() - self.pos.pos);
		for _ in 0..n {
			if self.src[self.pos.pos] == b'\n' {
				self.pos.line += 1;
				self.pos.col = 1;
			} else {
				self.pos.col += 1;
			}
			self.pos.pos += 1;
		}
	}

	pub fn restore(&mut self, p: &Pos) {
		self.pos = p.clone();
	}

	pub fn cur(&self) -> char {
		self.src[self.pos.pos] as char
	}

	/// Extracts a string before the current position
	pub fn cur_str(&self, n: usize) -> String {
		let start = self.pos.pos.saturating_sub(n);
		String::from_utf8_lossy(&self.src[start..self.pos.pos]).to_string()
	}

	/// Extracts a string before the current position
	pub fn cur_from(&self, p: &Pos) -> String {
		String::from_utf8_lossy(&self.src[p.pos..self.pos.pos]).to_string()
	}

	pub fn cur_digit(&self) -> Option<u8> {
		if self.is_eof() {
			None
		} else {
			let c = self.src[self.pos.pos] as char;
			if '0' <= c && c <= '9' {
				Some(c as u8 - b'0')
			} else if 'a' <= c && c <= 'z' {
				Some(c as u8 - b'a' + 10)
			} else if 'A' <= c && c <= 'Z' {
				Some(c as u8 - b'A' + 10)
			} else {
				None
			}
		}
	}

	pub fn char(&self, c: char) -> bool {
		!self.is_eof() && self.src[self.pos.pos] as char == c
	}

	pub fn str(&self, s: &str) -> bool {
		// Compare as bytes
		let bs = s.as_bytes();
		bs == &self.src[self.pos.pos..self.pos.pos + bs.len()]
	}

	pub fn range(&self, f: char, t: char) -> bool {
		!self.is_eof()
			&& f <= self.src[self.pos.pos] as char
			&& self.src[self.pos.pos] as char <= t
	}

	pub fn one_of(&self, set: &CharSet) -> bool {
		if self.is_eof() {
			return false;
		}
		let b = self.src[self.pos.pos] as usize;
		b < 127 && set[b]
	}
}

// INTJ Parser

fn skip_until_newline(s: &mut ParseState) {
	while !s.is_eof() && s.cur() != '\n' {
		s.skip(1);
	}
}

/// Skip whitespaces and comments
/// Return None if no newline/separator skipped
/// Return Some(str) if some newline/separators skipped
/// The element in some is docstring. (If exists)
fn skip_ignores(s: &mut ParseState) -> Result<Option<String>, ParseError> {
	let mut newline: bool = false;
	let mut ds: Vec<String> = vec![];

	while !s.is_eof() {
		if s.one_of(&SEPARATORS) {
			// Newline
			newline = true;
			s.skip(1);
		} else if s.str("///") {
			s.skip(3);
			// Docstring
			let begin = s.pos.pos;
			skip_until_newline(s);
			let contents = s.cur_str(begin);
			ds.push(contents);
		} else if s.str("//") || s.str("#!") {
			// Line Comment
			skip_until_newline(s);
		} else if s.str("/*") {
			// Find the end of the comment
			loop {
				if s.is_eof() {
					return s.err_result("Unclosed comment");
				}
				if s.str("*/") {
					s.skip(2);
					break;
				}
				s.skip(1);
			}
		} else if !s.one_of(&WHITES) {
			break;
		}
	}

	Ok(if !ds.is_empty() {
		Some(ds.join("\n"))
	} else if newline {
		Some(String::new())
	} else {
		None
	})
}

/// Parse number
/// Possible formats:
/// - Decimal integer: [-+]?[0-9]+
/// - Decimal float: [-+]?[0-9]+.[0-9]*
/// - Hexadecimal integer: 0x[0-9a-fA-F]+
/// - Binary integer: 0b[01]+
/// - Octal integer: 0o[0-7]+
fn parse_num(s: &mut ParseState) -> Option<Expr> {
	let pos = s.pos.clone();
	let mut sign = 1;
	let mut base = 10;

	// Check sign
	if s.char('-') {
		sign = -1;
		s.skip(1);
	} else if s.char('+') {
		s.skip(1);
	}

	// Check base
	if s.str("0x") {
		base = 16;
		s.skip(2);
	} else if s.str("0b") {
		base = 2;
		s.skip(2);
	} else if s.str("0o") {
		base = 8;
		s.skip(2);
	}

	if !s.cur_digit().is_some_and(|x| x < base) {
		s.restore(&pos);
		return None;
	}

	let mut num = 0;
	while let Some(d) = s.cur_digit() {
		if d >= base {
			break;
		}
		num = num * base as i64 + d as i64;
		s.skip(1);
	}

	let expr = if s.char('.') {
		// Float
		s.skip(1);
		let mult = 1.0 / base as f64;
		let mut frac = 0.0;
		let mut scale = 1.0;
		while let Some(d) = s.cur_digit() {
			if d >= base {
				break;
			}
			scale *= mult;
			frac += d as f64 * scale;
			s.skip(1);
		}
		ExprBase::Float(sign as f64 * (num as f64 + frac))
	} else {
		ExprBase::Int(sign * num)
	};

	Some(Expr { pos, expr })
}

/// Parse string
fn parse_string(s: &mut ParseState) -> ParseResult<Expr> {
	if s.is_eof() {
		return s.err_result("Unexpected EOF");
	}

	let pos = s.pos.clone();

	// Get opening quotes
	let open = s.cur();
	s.skip(1);

	let start_pos = s.pos.clone();

	loop {
		if s.is_eof() {
			return s.err_result("Unclosed string literal");
		}
		if s.char(open) {
			break;
		}
		if s.char('\\') {
			// Escape
			s.skip(1);
			if s.is_eof() {
				return s
					.err_result("Unexpected EOF, expecting escape character");
			}
		}
		s.skip(1);
	}

	let contents = utils::string::unescape(s.cur_from(&start_pos).as_str());

	s.skip(1);

	Ok(Expr {
		pos,
		expr: ExprBase::String(contents),
	})
}

fn parse_exprs(s: &mut ParseState) -> ParseResult<std::vec::Vec<Expr>> {
	let mut exprs = vec![];
	let mut pended_exprs = vec![];
	let mut docstring = String::new();

	loop {
		// Check ignores
		if let Some(ds) = skip_ignores(s)? {
			docstring = ds;

			// Push pended exprs in reverse order
			for expr in pended_exprs.drain(..).rev() {
				exprs.push(expr);
			}
		}

		if s.is_eof() {
			break;
		}

		match s.cur() {
			'\'' | '"' => {
				if let Ok(expr) = parse_string(s) {
					exprs.push(expr);
				} else {
					return s.err_result("Unexpected token");
				}
			}
			_ => {
				if let Some(expr) = parse_num(s) {
					exprs.push(expr);
				} else {
					return s.err_result("Unexpected token");
				}
			}
		}
	}

	// Push pended exprs in reverse order
	for expr in pended_exprs.drain(..).rev() {
		exprs.push(expr);
	}

	Ok(exprs)
}

pub fn parse(name: String, source: String) -> ParseResult<Expr> {
	let mut s = ParseState::new(name, &source);
	let es = parse_exprs(&mut s)?;
	println!("{:?}", es);
	if es.len() != 1 {
		return s.err_result("Expected a single expression");
	}
	Ok(es.into_iter().next().unwrap())
}
