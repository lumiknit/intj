use crate::utils::string;

fn assert_escape(input: &str, to_be: &str) {
	assert_eq!(string::escape(input), to_be);
}

fn assert_unescape(input: &str, to_be: &str) {
	assert_eq!(string::unescape(input), to_be);
}

fn assert_unescape_escape(input: &str) {
	assert_eq!(string::unescape(string::escape(input).as_str()), input)
}

#[test]
fn test_escape() {
	assert_escape("hello", "hello");
	assert_escape("hello\nworld", "hello\\nworld");
	assert_escape("한글 가나다", "한글 가나다");
	assert_escape(
		"This\'\" \\ print \r\n\t\0",
		"This\\'\\\" \\\\ print \\r\\n\\t\\u0000",
	);
}

#[test]
fn test_unescape() {
	assert_unescape("hello", "hello");
	assert_unescape("hello\\nworld", "hello\nworld");
	assert_unescape("한글 가나다", "한글 가나다");
	assert_unescape(
		"This\\'\\\" \\\\ print \\r\\n\\t\\u0000",
		"This'\" \\ print \r\n\t\0",
	);
	assert_unescape("\\0", "\x00");
	assert_unescape("\\1", "\x01");
	assert_unescape("\\10", "\x08");
	assert_unescape("\\x3", "\x03");
	assert_unescape("\\u1234", "\u{1234}");
}

#[test]
fn test_unescape_escape() {
	assert_unescape_escape(
		r#"
        hello, 가나다, #이 문장 dthis <- doqitn >
        \x0['>]
    "#,
	);
	assert_unescape_escape("abc\x00\x01\x02\x03\x04\x05\x06\x07\x08");
	assert_unescape_escape("abc\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10");
	assert_unescape_escape("abc\x11\x12\x13\x14\x15\x16\x17\x18");
	assert_unescape_escape("abc\x19\x1A\x1B\x1C\x1D\x1E\x1F\x20");
	assert_unescape_escape("abc\x21\x22\x23\x24\x25\x26\x27\x28");
	assert_unescape_escape("abc\x7Faskdl");
}
