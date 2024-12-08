mod _lib;
mod script;

use crate::_lib::{base, io};

use clap::{Arg, Command};
use colored::*;
use std::collections::LinkedList;

/// ## 程序版本
const VERSION: &str = "t.0.1";
/// ## 程序名称
const NAME: &str = "\"青鸾\" interpreter.";
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
    let mut args = get_args().into_iter();
    // 检查开始方式
    if let Some(path) = args.next() {
        let mut file = io::FileWrapper::new(path);
        if file.works() {
            while let Some(line) = file.next() {
                match script::read(line) {
                    script::ResultType::Error(e) => e.print(),
                    script::ResultType::Exit => break,
                    script::ResultType::Next => continue,
                }
            }
        } else {
            io::Log::new("e", format!("File \"{}\" not found.", file.path()).as_str(), 0).print();
        }
    }
    loop {
        let mut line = String::new();
        base::input("> ", &mut line);
        match script::read(line) {
            script::ResultType::Error(e) => e.print(),
            script::ResultType::Exit => break,
            script::ResultType::Next => continue,
        }
    }
    exit(0)
}

/// ## 退出程序
/// 提示用户按任意键继续
fn exit(exit_code: i32) {
    base::system("please");
    std::process::exit(exit_code);
}

/// ## 获取命令行参数
/// 读入操作和必要参数
/// 0. 路径
fn get_args() -> LinkedList<String> {
    let mut res: LinkedList<String> = LinkedList::new();
    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .arg(Arg::new("path")
            .value_name("PATH")
            .help("Script path"))
        .get_matches();

    // 获取路径
    if let Some(path) = matches.get_one::<String>("path") {
        res.push_back(path.to_string());
    }
    res
}
