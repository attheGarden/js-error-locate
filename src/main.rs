use std::{env, process};
use js_error_locate::ErrorInfo;

fn main() {
    // 1.获取参数
    let error_info = ErrorInfo::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    js_error_locate::run(error_info);
}
