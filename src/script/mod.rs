pub(crate) mod run;

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
    // if text.is_empty() {
        ResultType::Next
    // }
    // ResultType::Next
    // TODO: 语法解析树
}

/// ## 解析命令
enum Token {
    /// ## 关键字
    Keyword(String),
    /// ## 标识符
    Identifier(String),
    /// ## 常量
    Number(String),
    /// ## 运算符
    Operator(char),
}

/// ## 值类型
enum Value {
    /// 字符
    Char(char),
    /// 短整型
    Short(i16),
    /// 整型
    Int(isize),
    /// 长整型
    Long(i128),
    /// 浮点型
    Float(f32),
    /// 双浮点型
    Double(f64),
    /// 字符串
    String(String),
    /// 布尔型
    Bool(bool),
}