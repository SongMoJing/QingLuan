mod _lib;
mod script;

use crate::_lib::base;
use crate::_lib::io;
use crate::script::run;
use clap::{Arg, Command};
use colored::*;
use std::collections::LinkedList;

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

    let mut file = io::FileWrapper::new(args.next().unwrap());
    if file.works() {
        let mut commenting: bool = false;
        while let Some(line) = file.next() {
            if commenting {
                break;
            }
            run::read(line, &commenting);
        }
        println!("注释： {}", commenting);
    } else {
        io::Log::new("e", "File not found.", 0).print();
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
        .arg(Arg::new("path")
            .value_name("PATH")
            .help("Script path"))
        .get_matches();

    // 获取路径
    res.push_back(matches.get_one::<String>("path").unwrap().to_string());
    res
}
