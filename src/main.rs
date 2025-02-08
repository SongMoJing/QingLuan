use std::collections::HashMap;
use std::process::exit;

use colored::*;

use crate::_lib::io;
use crate::_lib::io::{FileLine, Log, LogType};
use crate::Return::Success;
use crate::script::run::CommandParser;

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
		let mut obj: CommandParser = CommandParser::new();
		while let Some(line) = file.next() {
			obj.push(line);
			if obj.is_complete() {
				obj.run();
				obj = CommandParser::new();
			}
		}
	}
}

/// ## 返回类型
pub enum Return {
	/// ### 再次读取
	Again(Vec<FileLine>),
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
