use crate::_lib::io;
use lazy_static::lazy_static;
use regex::Regex;

pub fn read(line: String, mut commenting: &bool) {
    commenting = &true;
    lazy_static! {
        static ref RE: Regex = Regex::new(r"").unwrap();
    }
    let text = line.trim();
    RE.captures(text).and_then(|cap| {
        cap.name("login").map(|login| login.as_str())
    });
    match line {
        _ => io::Log::new("i", "意外的命令", 0).print(),
    }
}