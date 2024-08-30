/// Escape a given string based on C/javascript escape rules.
pub fn escape(s: &str) -> String {
	let mut escaped = String::new();
	for chr in s.chars() {
		match chr {
			'\\' => escaped.push_str("\\\\"),
			'\"' => escaped.push_str("\\\""),
			'\'' => escaped.push_str("\\\'"),
			'\n' => escaped.push_str("\\n"),
			'\r' => escaped.push_str("\\r"),
			'\t' => escaped.push_str("\\t"),
			'\x08' => escaped.push_str("\\b"), // backspace
			'\x0C' => escaped.push_str("\\f"), // form feed
			_ if chr.is_control() => {
				let hex = format!("\\u{:04x}", chr as u32);
				escaped.push_str(&hex);
			}
			_ => escaped.push(chr),
		}
	}
	escaped
}

fn handle_hex_escape(
	iter: &mut std::iter::Peekable<std::str::Chars>,
	digits: usize,
) -> Option<char> {
	let hex_slice: String = iter.by_ref().take(digits).collect();
	if let Ok(code_point) = u32::from_str_radix(&hex_slice, 16) {
		std::char::from_u32(code_point)
	} else {
		None
	}
}

/// Unescape a given string based on C/C++/javascript escape rules.
pub fn unescape(s: &str) -> String {
	let mut unescaped = String::new();
	let mut iter = s.chars().peekable();

	while let Some(chr) = iter.next() {
		if chr != '\\' {
			unescaped.push(chr);
			continue;
		}

		// Handle the escape sequences
		if let Some(next) = iter.next() {
			match next {
				'n' => unescaped.push('\n'),   // Newline
				'r' => unescaped.push('\r'),   // Carriage return
				't' => unescaped.push('\t'),   // Horizontal tab
				'b' => unescaped.push('\x08'), // Backspace
				'f' => unescaped.push('\x0C'), // Form feed
				'a' => unescaped.push('\x07'), // Alert (bell)
				'v' => unescaped.push('\x0B'), // Vertical tab
				'0'..='7' => {
					// Handle octal escape (e.g., \012 or \4)
					let mut octal_digits = String::new();
					octal_digits.push(next); // Collect the first octal digit
					for _ in 0..2 {
						// We will add up to 2 more digits
						if let Some(&c) = iter.peek() {
							if !c.is_digit(8) {
								break;
							}
							octal_digits.push(c);
							iter.next(); // consume the character
						}
					}
					u32::from_str_radix(&octal_digits, 8)
						.ok()
						.and_then(std::char::from_u32)
						.map(|c| unescaped.push(c));
				}
				'x' => {
					handle_hex_escape(&mut iter, 2).map(|c| unescaped.push(c));
				}
				'u' => {
					handle_hex_escape(&mut iter, 4).map(|c| unescaped.push(c));
				}
				_ => unescaped.push(next),
			}
		}
	}

	unescaped
}
