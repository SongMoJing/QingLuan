mod _lib;

use std::collections::LinkedList;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use crate::_lib::base;
use crate::_lib::file;

const VERSION: &str = "t.0.1";
const NAME: &str = "\"青鸾\" interpreter.";
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
    let _ = control::set_virtual_terminal(true);
}

fn main() {
    #[cfg(windows)]
    enable_ansi_support();

    let mut args = get_args().into_iter();

    let mut file = file::FileWrapper::new(args.next().unwrap());
    if file.works() {
        while let Some(line) = file.next() {
            println!("{}", line);
        }
    } else {
        println!("[{}]: {}", "Error".red(), "File not found.");
        exit(-1);
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
        .arg_required_else_help(true)
        .arg(Arg::new("path")
            .value_name("PATH")
            .help("Script path"))
        .get_matches();

    // 获取路径
    res.push_back(matches.get_one::<String>("path").unwrap().to_string());
    res
}
