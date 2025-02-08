#![allow(dead_code)]

use crate::_lib::io::{FileLine, Log, LogType};

/// ## 命令解析器
pub struct CommandParser {
	commands: Vec<FileLine>,
}

impl CommandParser {
	/// ## 新建一个命令解析器
	pub fn new() -> CommandParser {
		CommandParser {
			commands: Vec::new(),
		}
	}

	/// ## 判断命令是否完整
	pub fn is_complete(&self) -> bool {
		true
	}

	/// ## 添加命令
	pub fn push(&mut self, command: FileLine) {
		Log::new(LogType::Info, format!("行{} 有{}", command.number(), command.data()).as_str(), 0).print();
		self.commands.push(command);
	}

	/// ## 运行命令
	pub fn run(&mut self) {
		Log::new(LogType::Info, "执行完毕", 0).print();
	}
}
