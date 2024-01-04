mod parse;

#[derive(Debug)]
pub enum Number {
	I64(i64),
	F64(f64),
}

#[derive(Debug)]
pub enum Branch {
	IntLiteral(IntLiteral),
	FloatLiteral(FloatLiteral),
	ConstantLiteral(ConstantLiteral),
	FnCall(FnCall),
	Tree(Tree),
}

#[derive(Debug)]
pub struct ConstantLiteral {
	pub value: String,
	pub inline_fn: Option<FnCall>,
}

#[derive(Debug)]
pub struct IntLiteral {
	pub value: i64,
	pub inline_fn: Option<FnCall>,
}

#[derive(Debug)]
pub struct FloatLiteral {
	pub value: f64,
	pub inline_fn: Option<FnCall>,
}

#[derive(Debug)]
pub struct OpLiteral {
	pub value: String,
}

#[derive(Debug)]
pub struct Tree {
	pub op: OpLiteral,
	pub left: Option<Box<Branch>>,
	pub right: Option<Box<Branch>>,
}

#[derive(Debug)]
pub struct FnCall {
	pub name: String,
	pub params: Vec<Tree>,
	pub inline_fn: Option<Box<FnCall>>,
}

#[derive(Debug)]
pub struct Parser {}
