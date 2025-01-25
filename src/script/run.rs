#![allow(dead_code)]
/// ## 解析命令
fn parse(line: String) {
	// let mut tokens = Vec::new();
	let mut chars = line.chars();
	while let Some(_c) = chars.next() {
		// match c {
		// 	' ' => continue,
		// 	// _ => ,
		// }
	}
}

/// ## Token
pub enum Token {
	/// ## 标识符
	Identifier(String),
	/// ## 值类型
	Value(Value),
	/// ## 关键字
	Keyword(String),
	/// ## 运算符
	Operator(String),
}

/// ## 值类型
pub enum Value {
	/// ## 字符
	/// 使用`'`包裹
	Char(char),
	/// ## 短整型
	Short(i32),
	/// ## 整型
	Int(i64),
	/// ## 长整型
	Long(i128),
	/// ## 浮点型
	Float(f32),
	/// ## 双浮点型
	/// 使用`D/d`结尾
	Double(f64),
	/// ## 字符串
	/// 使用`"`包裹
	String(String),
	/// ## 布尔型
	/// `true`或`false`
	Bool(bool),
	/// ## 空值
	/// `null`
	Null,
}

/// ## 类类型
pub enum Class {
	Base(),
	Enum(),
	Interface(),
	Abstract(),
	Record(),
}