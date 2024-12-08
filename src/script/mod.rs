pub(crate) mod run;

use clap::builder::Str;
use crate::_lib::io::Log;

/// ## 返回类型
pub enum ResultType {
    Error(Log),
    Next,
    Exit,
}

/// ## 读取命令
pub fn read(line: String) -> ResultType {
    let text = line.trim();
    if text.is_empty() {
        return ResultType::Next;
    }
    let mut tokens: Vec<Token> = Vec::new();
    let mut temp: Vec<char> = Vec::new();
    for token in text.chars() {}

    ResultType::Next
}

/// ## 解析命令
enum Token {
    /// ## 标识符
    Identifier(String),
    /// ## 常量
    Value(Value),
    /// ## 关键字
    Keyword(String),
    /// ## 运算符
    Operator(String),
}

/// ## 值类型
enum Value {
    /// ## 字符
    /// 使用`'`包裹
    Char(char),
    /// ## 短整型
    /// 使用`S/s`结尾
    Short(i16),
    /// ## 整型
    Int(isize),
    /// ## 长整型
    /// 使用`L/l`结尾
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
}