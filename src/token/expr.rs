use super::file::Pos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExprListType {
	/// Codes
	Code,

	/// Array
	Array,

	/// Object
	Object,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeneralExprBase<E> {
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

	/// Import
	Import(String, String), // Target name, file name

	/// Assign
	Assign(String, String), // name, docstring

	/// Expression List
	List(ExprListType, Vec<E>),
}

pub type ExprBase = GeneralExprBase<Expr>;

#[derive(Debug, Clone)]
pub struct Expr {
	pub pos: Pos,
	pub expr: ExprBase,
}
