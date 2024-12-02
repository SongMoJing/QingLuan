mod _lib;

use std::collections::LinkedList;
use clap::{Arg, ArgMatches, Command};
use colored::*;
use crate::_lib::base;

const VERSION: &str = "t.0.1";
const NAME: &str = "\"青鸾\" Universal Package Manager.";
const AUTHOR: &str = "PRC.松蓦箐 <Song_Mojing@outlook.com>";

/// ## 启用 ANSI 支持
fn enable_ansi_support() {
    let _ = control::set_virtual_terminal(true);
}

fn main() {
    // 检查参数
    let args = std::env::args();
    if args.len() >= 2 {

    }
}
