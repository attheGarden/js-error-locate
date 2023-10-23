use std::{env, fs, process};
use std::fs::File;
use std::io::{ErrorKind, Write};
use sourcemap::SourceMap;

pub struct ErrorInfo {
    pub error_line: u32,
    pub error_column: u32,
    pub sourcemap_name: String
}

impl ErrorInfo {
    pub fn new(mut args: env::Args) -> Result<ErrorInfo, &'static str> {
        args.next();

        let error_line = args.next().unwrap_or_else(|| {
            eprintln!("Didn't get a error line number.");
            process::exit(1);
        }).parse().unwrap();

        let error_column = args.next().unwrap_or_else(|| {
            eprintln!("Didn't get a error column number.");
            process::exit(1);
        }).parse().unwrap();

        let sourcemap_name = args.next().unwrap_or_else(|| {
            eprintln!("Didn't get a sourcemap name.");
            process::exit(1);
        });

        Ok(ErrorInfo {
            error_line,
            error_column,
            sourcemap_name
        })
    }
}


pub fn run(error_info: ErrorInfo) {
    // 2. 读取文件
    let file_content = match fs::read_to_string(error_info.sourcemap_name) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            _ => panic!("Failed to read file")
        }
    };
    let sm = SourceMap::from_slice(file_content.as_bytes()).unwrap();
    // 3. 获取映射处理后的行列
    let token = sm.lookup_token(error_info.error_line - 1, error_info.error_column - 1).unwrap();
    // 原始行列信息
    let original_line = token.get_src_line() + 1;
    let start_line = match original_line > 5 { true => original_line - 5, false => 1 };

    let end_line = original_line + 5;

    // 4. 获取并输出原始代码
    let code = sm.get_source_contents(token.get_src_id()).unwrap();
    let lines: Vec<&str> = code.split("\n").collect();
    let mut file = File::create("output.txt").expect("Failed to create file");
    file.write_all(
        format!("source : {}, line: {} ", token.get_source().unwrap(), original_line)
            .as_bytes()).expect("Failed to write to file");
    file.write_all("\n".as_bytes()).expect("Failed to write to file");
    // 截取部分源码写入
    for (index, line) in lines.iter().enumerate() {
        if start_line < index as u32 && end_line > index as u32 {
            file.write_all(line.as_bytes()).expect("Failed to write to file");
            file.write_all("\n".as_bytes()).expect("Failed to write to file");
        }
    }
}
