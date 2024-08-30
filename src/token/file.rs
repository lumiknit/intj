// File helper for tokenizer / parser

use std::cmp::min;

#[derive(Debug, Clone)]
pub struct File {
	pub name: String,
	pub contents: String,
}

pub trait FileLoader {
	fn load_file(&mut self, name: &str) -> Result<File, String>;
}

/// STD file loader
pub struct StdFileLoader {}

impl FileLoader for StdFileLoader {
	fn load_file(&mut self, name: &str) -> Result<File, String> {
		match std::fs::read_to_string(name) {
			Ok(contents) => Ok(File {
				name: name.to_string(),
				contents: contents,
			}),
			Err(e) => Err(e.to_string()),
		}
	}
}

/// Position of a token in a file
#[derive(Debug, Clone)]
pub struct Pos {
	pub pos: usize,
	pub line: usize,
	pub col: usize,
}

impl File {
	pub fn find_position(&self, pos: usize) -> Pos {
		let mut line = 0;
		let mut col = 0;
		for (i, c) in self.contents.chars().enumerate() {
			if i == pos {
				break;
			}
			if c == '\n' {
				line += 1;
				col = 0;
			} else {
				col += 1;
			}
		}
		Pos { pos, line, col }
	}
}

impl Pos {
	pub fn is_eof(&self, src: &str) -> bool {
		self.pos >= src.len()
	}

	pub fn next(&mut self, src: &str, mut n: usize) {
		n = min(n, src.len() - self.pos);
		for _ in 0..n {
			if src.chars().nth(self.pos).unwrap() == '\n' {
				self.line += 1;
				self.col = 0;
			} else {
				self.col += 1;
			}
		}
		self.pos += n;
	}

	/// Get the character at the current position + offset
	pub fn char(&self, src: &str, off: usize) -> Option<char> {
		src.chars().nth(self.pos + off)
	}

	/// Compare the current position + offset with a string
	pub fn eq_str(&self, src: &str, off: usize, s: &str) -> bool {
		if self.pos + off + s.len() > src.len() {
			return false;
		}

		// Create a slice of the string
		let src_slice = &src[self.pos + off..(self.pos + off + s.len())];

		// Compare the slice with the string
		return src_slice == s;
	}
}
