pub(crate) mod run;

/// ## 控制符
pub enum Control {
	/// ## 左大括号
	/// `{`
	LeftBrace,
	/// ## 右大括号
	/// `}`
	RightBrace,
	/// ## 左中括号
	/// `[`
	LeftBracket,
	/// ## 右中括号
	/// `]`
	RightBracket,
	/// ## 左小括号
	/// `(`
	LeftParenthesis,
	/// ## 右小括号
	/// `)`
	RightParenthesis,
	/// ## 分号
	/// `;`
	Semicolon,
	/// ## 冒号
	/// `:`
	Colon,
	/// ## 点
	/// `.`
	Dot,
	/// ## 逗号
	/// `,`
	Comma,
	/// ## 问号
	/// `?`
	QuestionMark,
}

/// ## 值
pub enum Value {
	Char(char),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	U128(u128),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	I128(i128),
	F32(f32),
	F64(f64),
	/// ## 字符串
	String(String),
	/// ## 布尔型
	Bool(bool),
}

/// ## 类
pub enum Class {
	Base (ClassBase),
	Enum (ClassEnum),
	Interface (ClassInterface),
	Abstract (ClassAbstract)
}

/// ### 基类
pub struct ClassBase {
	pub attr: Vec<Attribute>,
	pub name: String,
	pub father: Option<ClassBase|ClassAbstract>,
	pub implements: Vec<ClassInterface>,
	pub function: Vec<Function>,
}

/// ### 枚举
pub struct ClassEnum {
	pub attr: Vec<Attribute>,
	pub name: String,
	pub father: Option<Class>,
	pub enums: Vec<Enum>,
	pub function: Vec<Function>,
}

/// ### 接口
pub struct ClassInterface {
	pub attr: Vec<Attribute>,
	pub name: String,
	pub father: Option<Class>,
	pub abstract_function: Vec<AbstractFunction>,
}

/// ### 抽象类
pub struct ClassAbstract {
	pub attr: Vec<Attribute>,
	pub name: String,
	pub father: Option<Class>,
	pub abstract_function: Vec<Function>,
	pub function: Vec<Function>,
}

/// ## 属性
pub enum Attribute {
	/// ## 公开可见
	Public,
	/// ## 仅同项目内可见
	Project,
	/// ## 仅包内可见
	Package,
	/// ## 仅类内可见
	Private,
	/// ## 静态
	Static,
	/// ## 只读
	Final
}

/// ## 抽象方法
pub enum AbstractFunction {}

/// ## 方法
pub enum Function {}