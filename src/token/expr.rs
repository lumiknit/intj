use super::file::Pos;

pub enum ExprListType {
	/// Codes
	Code,

	/// Array
	Array,

	/// Object
	Object,
}

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

	/// Expression List
	List(ExprListType, Vec<E>),
}

pub struct Expr {
	pub pos: Pos,
	pub expr: ExprBase<Expr>,
}
