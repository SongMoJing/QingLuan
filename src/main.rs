use std::collections::HashMap;
use std::process::exit;

use colored::*;

use crate::_lib::io;
use crate::_lib::io::{FileLine, Log, LogType};
use crate::Return::{Again, Success};
use crate::script::run::Value;

mod _lib;
mod script;

/// ## 程序版本
const VERSION: &str = "t.0.1";
/// ## 程序名称
const NAME: &str = "\"青鸾\" 解释器";
/// ## 程序作者
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
	let _ = control::set_virtual_terminal(true);
}

fn main() {
	#[cfg(windows)]
	enable_ansi_support();
	// 获得参数
	let mut args = get_args();
	// 检查开始方式
	if args.get("scriptPath").is_some() {
		let mut file = io::FileWrapper::new(args.remove("scriptPath").unwrap());
		if !file.works() {
			Log::new(LogType::Err, format!("未找到文件：“{}”。", file.path()).as_str(), -1).print();
		}
		let mut commands: Vec<FileLine> = Vec::new();
		while let Some(line) = file.next() {
			commands.push(line);
			let res = read(commands);
			if let Success = res {
				for command in commands {
					println!("{}", command.data().green());
				}
				commands.clear();
			}
		}
	}
}

/// ## 读取命令
pub fn read(lines: Vec<FileLine>) -> Return {
	let mut commands = Vec::new();
	for line in lines {
		let text = line.data().trim().to_string();
		let mut flag = false;
		let mut index = 0;
		for c in text.chars() {
			if !flag && c == '#' {
				break;
			} else if c == '"' {
				flag = !flag;
			}
			index += 1;
		}
		let text = text[..index].to_string();
		if text.is_empty() {
			continue;
		}
		commands.push(FileLine::new(line.number(), text));
	}

	// if text.is_empty() {
	// 	Again(String::new())
	// } else if !commands..ends_with(";") && !text.ends_with("}") {
	// 	Again(text.to_string())
	// } else {
		Success
	// }
}

/// ## 返回类型
pub enum Return {
	/// ### 再次读取
	Again(String),
	/// ### 成功
	Success,
}


/// ## 获取命令行参数
/// 读入操作和必要参数
/// 0. 路径
fn get_args() -> HashMap<String, String> {
	// 打印帮助
	fn help() {
		println!("帮助");
		println!("{} 版本：{}", NAME, VERSION);
		println!("{}\n", AUTHOR);
		println!("用法：");
		println!("   QingLuan [参数] [选项]");
		println!("参数：");
		println!("   [必填] <脚本路径> 填入用于解释的脚本路径");
		println!("选项：");
		println!("   <-h | --help>     获取帮助");
		exit(0);
	}
	// 获取参数
	let mut res: HashMap<String, String> = HashMap::new();
	let arg = std::env::args().nth(1);
	if let Some(option) = arg {
		if option == "-h" || option == "--help" {
			help();
		} else {
			res.insert("scriptPath".to_string(), option);
			return res;
		}
	}
	help();
	res
}
