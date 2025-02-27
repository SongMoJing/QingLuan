use std::collections::HashMap;
use std::process::exit;

use colored::*;

use crate::_lib::io::{Log, LogType};

mod _lib;
mod script;

/// ## 程序版本
const VERSION: &str = "t.0.1";
/// ## 程序名称
const NAME: &str = "\"青鸾\" 编译器";
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
	let args = get_args();
	// 检查开始方式
	if let Some(root_path) = args.get("projectPath") {
		// 查看projectPath是否存在
		if std::fs::metadata(args.get("projectPath").unwrap()).is_ok() {
			// 读取QingLuan.toml文件
			script::start(root_path.to_string());
		} else {
			Log::new(LogType::Err, format!("路径 {} 不存在。", args.get("projectPath").unwrap()).as_str(), 21).print();
		}
	}
}

/// ## 获取命令行参数
/// 读入操作和必要参数<br>
/// scriptPath. 路径
fn get_args() -> HashMap<String, String> {
	let mut res: HashMap<String, String> = HashMap::new();

	// 打印帮助
	fn help() {
		println!("{}", "帮助".yellow());
		println!("   {} 版本：{}", NAME.green(), VERSION.green());
		println!("   {}", AUTHOR);
		println!("{}", "用法：".yellow());
		println!("   QingLuanCompile [参数] [选项]");
		println!("{}", "参数：".yellow());
		println!("   [必填] <脚本路径> 填入用于解释的脚本路径");
		println!("{}", "选项：".yellow());
		println!("   <-h | --help>     获取帮助");
		println!("   <-c | --compile>  打包方式 可选：debug|release");
		exit(0);
	}

	// 设置编译方式
	fn set_compile_mode(res: &mut HashMap<String, String>, mode: &str) {
		res.insert("compile".to_string(), mode.to_string());
	}

	// 获取参数
	let mut args = std::env::args();
	if args.len() < 2 {
		help();
	} else {
		args.next();
	}

	while let Some(arg) = args.next() {
		if arg.starts_with("-") {
			match arg.as_str() {
				// 显示帮助
				"-h" | "--help" => help(),
				// 编译方式
				"-c" | "--compile" => {
					if let Some(path) = args.next() {
						match path.as_str() {
							"debug" => set_compile_mode(&mut res, "debug"),
							"release" => set_compile_mode(&mut res, "release"),
							_ => {
								Log::new(LogType::Err, format!("未知的编译方式 {}。", path).as_str(), 12).print();
							}
						}
					} else {
						set_compile_mode(&mut res, "debug");
						Log::new(LogType::Warn, "缺少编译方式，默认为Debug", 0).print();
					}
				}
				// 未知目标
				_ => {
					Log::new(LogType::Err, format!("未知的选项 {}。", arg).as_str(), 11).print();
				}
			}
		} else {
			if res.contains_key("projectPath") {
				Log::new(LogType::Warn, format!("重复的参数 {}，只采用首次选择。", arg).as_str(), 0).print();
			} else {
				res.insert("projectPath".to_string(), arg);
			}
		}
	}

	// 检查
	if !res.contains_key("projectPath") {
		let current_dir = std::env::current_dir().unwrap_or_else(|_| {
			Log::new(LogType::Err, "无法获取当前目录", 13).print();
			std::path::PathBuf::from(".")
		});
		res.insert("projectPath".to_string(), current_dir.display().to_string());
	}
	if !res.contains_key("compile") {
		set_compile_mode(&mut res, "debug");
	}

	res
}

