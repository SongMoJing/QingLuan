#![allow(dead_code)]

use crate::_lib;
use crate::_lib::io::Log;
use crate::_lib::io::LogType;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::process::{Command, Stdio};
use serde::{Deserialize};
use toml::Value;

pub(crate) mod run;
mod lava_river;

pub fn start(root: String) {
	let mut toml = _lib::io::FileWrapper::new(format!("{}/QingLuan.toml", root));
	// 读取文件内容
	let content: String = toml.read_to_string().unwrap_or_else(|e| match e.kind() {
		ErrorKind::NotFound => {
			Log::new(LogType::Err, "QingLuan.toml 文件读取失败，请检查文件是否存在。", 21).print();
			"null".to_string()
		}
		ErrorKind::PermissionDenied => {
			Log::new(LogType::Err, "QingLuan.toml 文件读取失败，请检查文件权限。", 22).print();
			"null".to_string()
		}
		_ => {
			Log::new(LogType::Err, "QingLuan.toml 产生未知错误。", 20).print();
			"null".to_string()
		}
	});
	let value: Value = content.parse::<Value>().unwrap_or_else(|_| {
		Log::new(LogType::Err, "QingLuan.toml 文件解析失败，请检查文件格式。", 23).print();
		Value::Integer(23)
	});
	// 获取依赖项
	let mut dependencies: HashMap<String, HashMap<String, Vec<String>>> = Default::default();
	if let Some(deps) = value.get("dependencies").and_then(Value::as_table) {
		// 遍历依赖项
		for (name, dep_value) in deps {
			let mut item: HashMap<String, Vec<String>> = HashMap::new();
			match dep_value {
				// 字符串格式
				Value::String(v) => {
					item.insert("version".to_string(), vec![v.to_string()]);
					item.insert("include".to_string(), vec!["*".to_string()]);
				}
				// 内联表格式
				Value::Table(table) => {
					let mut vec: Vec<String> = vec![];
					for (key, val) in table {
						match key.as_str() {
							"version" => {
								if let Value::String(v) = val {
									vec.push(v.to_string());
								} else {
									Log::new(LogType::Err, format!("依赖文件解析失败，dependencies中依赖项 {} 的版本值不是字符串。", name).as_str(), 23).print()
								}
							},
							"include" | "exclude" => {
								if let Value::Array(arr) = val {
									for v in arr {
										if let Value::String(v) = v {
											vec.push(v.to_string());
										} else {
											Log::new(LogType::Err, format!("依赖文件解析失败，dependencies中依赖项 {} 的 {} 值不是字符串。", name, key).as_str(), 23).print()
										}
									}
								}
							},
							_ => Log::new(LogType::Err, format!("依赖文件解析失败，dependencies中依赖项 {} 包含了未知的键 {}。", name, key).as_str(), 23).print()
						}
						item.insert(key.to_string(), vec.to_vec());
						vec.clear();
					}
				}
				_ => Log::new(LogType::Err, format!("依赖文件解析失败，dependencies中依赖项 {} 包含了类型为 {} 的值。", name, dep_value.type_str()).as_str(), 23).print()
			}
			dependencies.insert(name.to_string(), item);
		}
	}
	// 读取配置文件
	let config: Config = toml::from_str(content.as_str()).unwrap_or_else(|_| {
		Log::new(LogType::Err, "QingLuan.toml 文件解析失败，请检查文件格式。", 23).print();
		Config {
			project: Project {
				name: "null".to_string(),
				version: "null".to_string(),
				sdk_edition: "null".to_string(),
				main: "null".to_string(),
				authors: vec![]
			},
			build: Build {
				path: "null".to_string(),
				target: "null".to_string()
			}
		}
	});
	// 调用QingLuanPackageManager获取依赖库信息
	for (package_name, item) in dependencies.iter() {
		let output = Command::new("QingLuanPackageManager")
			.args(&["find", package_name, item.get("version").unwrap().get(0).unwrap()])
			.stdout(Stdio::piped())  // 捕获标准输出
			.stderr(Stdio::piped())  // 捕获错误输出
			.output();
		let mut path: &str;
		match output {
			Ok(output) => {
				if output.status.success() {
					let stdout = String::from_utf8_lossy(&output.stdout);
					let stderr = String::from_utf8_lossy(&output.stderr);
					if !stderr.is_empty() {
						Log::new(LogType::Err, format!("依赖文件解析失败，在读取 {} 依赖库时，包管理器返回了：\n\t {}", package_name, stderr).as_str(), 23).print()
					} else {
						path = stdout.trim();
					}
				} else {
					Log::new(LogType::Err, format!("依赖文件解析失败，在读取 {} 依赖库时，包管理器返回了非零的返回值： {} 。", package_name, output.status.code().unwrap_or_else(|| -1)).as_str(), 23).print()
				}
			}
			Err(_e) => {
				Log::new(LogType::Err, format!("依赖文件解析失败，在读取 {} 依赖库时，包管理器没有响应或超时，请检查包管理器状态。", package_name).as_str(), 23).print()
			}
		}
		// TODO 读取依赖库，并织入LavaRiver
	}

	println!("项目名称：{}", config.project.name);
	println!("项目版本：{}", config.project.version);
	println!("SDK版本：{}", config.project.sdk_edition);
}

#[derive(Debug, Deserialize)]
struct Config {
	project: Project,
	build: Build
}

#[derive(Debug, Deserialize)]
struct Project {
	// 项目名称
	name: String,
	// 项目版本
	version: String,
	// SDK版本
	sdk_edition: String,
	// 入口文件
	main: String,
	// 作者
	authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Build {
	// 路径
	path: String,
	// 构建目标
	target: String,
}
