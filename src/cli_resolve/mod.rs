mod base64;
mod csv_args;
mod gen_pass;
pub use base64::{AlphabetType, Base64Commands, CodeType};
use clap::{Parser, Subcommand};
pub use csv_args::{Csvcmd, FormatType};
use gen_pass::GenPassOpts;

/*
rcli csv format -i input.csv -o output -r -t json
*/

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
    // 没有子指令 , 只有选项
    #[command(name = "genpass", about = "generate password")]
    Genpassword(GenPassOpts),
    // Base64 指令
    Base64(Base64Commands),
}
