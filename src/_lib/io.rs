use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};
use std::path::Path;

pub struct FileWrapper {
    works: bool,
    file: Option<BufReader<File>>,
}

impl FileWrapper {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path);
        Self {
            works: file.is_ok(),
            file: {
                Some(BufReader::new({
                    match file {
                        Ok(file) => file,
                        Err(_) => return Self { works: false, file: None },
                    }
                }))
            },
        }
    }

    /// ## 对象正常
    pub fn works(&mut self) -> bool {
        self.works
    }

    /// ## 读取下一行
    pub fn next(&mut self) -> Option<String> {
        if self.works {
            self.file.as_mut().unwrap().lines().next().map(|line| line.unwrap())
        } else {
            None
        }
    }
}

use colored::*;
pub struct Log {
    _type: ColoredString,
    _msg: String,
    _stop: Option<i32>,
}

impl Log {
    /// ## 创建新的日志输出
    /// `_type` 错误的类型： e:错误 i:表示信息 w:表示警告 <br />
    /// `_msg` 消息正文 <br />
    /// `stop_key` 是退出程序的代码（如果错误是致命的） <br />如果为`0`则不退出程序，继续运行
    pub fn new(_type: &str, _msg: &str, stop_key: i32) -> Self {
        Self {
            _type: match _type {
                "e" => "Error".red(),
                "i" => "Info ".bold(),
                "w" => "Warn ".yellow(),
                _ => ColoredString::from(_type),
            },
            _msg: _msg.to_string(),
            _stop: if stop_key != 0 { Option::from(stop_key) } else { None } ,
        }
    }

    /// ## 打印日志
    pub fn print(&self) {
        println!("[{}]: {}", self._type, self._msg);
        if !self._stop.is_none() {
            std::process::exit(self._stop.unwrap());
        }
    }
}