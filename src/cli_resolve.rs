use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::Path;

// 定义命令结构体和相关信息 , 相关信息没有指定值的回从Cargo.toml中读取
#[derive(Parser, Debug)]
#[command(version , about , author , long_about=None)]
pub struct Args {
    // 定义子指令
    #[command(subcommand)]
    pub cmd: Command,
}
// 子指令
#[derive(Subcommand, Debug)]
pub enum Command {
    // name可以不设置 , 会自动从子指令名称中获取(转换为小写)
    #[command(subcommand)]
    Csv(Csvcmd),
}

#[derive(Debug, Subcommand)]
pub enum Csvcmd {
    // 子指令中的子指令
    #[command(name = "show", about = "show the csv file content")]
    Show(ShowOPts),
    #[command(name = "json", about = "convert csv to json")]
    Json(CsvOpts),
}

// 子指令中的选项
#[derive(Parser, Debug)]
pub struct CsvOpts {
    // 启用短选项和长选项, 定义帮助信息 , 验证路径是否存在
    #[arg(short, long, help = "input file name", value_parser = verify_input_file)]
    pub input: String,
    // 启用短选项和长选项, 定义帮助信息 , 验证文件是否不存在 , 设置默认值
    #[arg(short, long, help = "output file name", value_parser = verify_output_file , default_value="assets/output.json")]
    pub output: String,
    // 定义生成文件是否覆盖已有文件
    #[arg(short, long, help = "replace file or not", default_value_t = false)]
    pub replace: bool,
    // default_value_t设置默认值需要设置成正确的类型 , default_value 会自动调用值的`.into` 进行转换
    #[arg(short, long, help = "dilimiter symbol", default_value_t = ',')]
    pub dilimiter: char,
    #[arg(short = 'c', long, help = "show header or not", default_value_t = true)]
    pub header: bool,
}

#[derive(Debug, Parser)]
// 显示指令的选项
pub struct ShowOPts {
    // 显示文件内容
    #[arg(short, long, help = "show file content", value_parser = verify_input_file)]
    pub input: String,
    #[arg(
        short,
        long,
        help = "dilimiter symbol default is ','",
        default_value_t = ','
    )]
    pub dilimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // 验证输入文件是否存在
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("file {} not found", filename))
    }
}
fn verify_output_file(filename: &str) -> Result<String, String> {
    // 验证是.csv 文件
    if filename.ends_with(".json") {
        Ok(filename.to_string())
    } else {
        Err(format!("file {} not end with .csv", filename))
    }
}
