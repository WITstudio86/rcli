use clap::{Parser, Subcommand};
use std::{path::Path, str::FromStr};

#[derive(Debug, Subcommand)]
pub enum Csvcmd {
    // 子指令中的子指令
    #[command(name = "show", about = "show the csv file content")]
    Show(ShowOPts),
    #[command(name = "format", about = "convert csv to json")]
    Format(CsvOpts),
}

// 子指令中的选项
#[derive(Parser, Debug)]
pub struct CsvOpts {
    // 启用短选项和长选项, 定义帮助信息 , 验证路径是否存在
    #[arg(short, long, help = "input file name", value_parser = verify_input_file)]
    pub input: String,
    // 启用短选项和长选项, 定义帮助信息 , 验证文件是否不存在 , 设置默认值
    #[arg(short, long, help = "output file name")]
    pub output: Option<String>, // 如果没有的话在处理的时候设置默认值
    // 定义生成文件是否覆盖已有文件
    #[arg(short, long, help = "replace file or not", default_value_t = false)]
    pub replace: bool,
    // 定义生成什么样的结果文件, 需要实现FromStr trait 才能自动转换
    #[arg(short = 't', long, help = "output file type", default_value = "json")]
    pub output_type: FormatType,
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

#[derive(Debug, Clone, Copy)]
pub enum FormatType {
    Json,
    Yaml,
    Toml,
}
// 为 FormatType 实现 FromStr trait
impl FromStr for FormatType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(FormatType::Json),
            "yaml" => Ok(FormatType::Yaml),
            "toml" => Ok(FormatType::Toml),
            _ => Err(anyhow::Error::msg("format type must be json or yaml")),
        }
    }
}

// 需要实现 Display trait 才能用在输出上
impl std::fmt::Display for FormatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatType::Json => write!(f, "json"),
            FormatType::Yaml => write!(f, "yaml"),
            FormatType::Toml => write!(f, "toml"),
        }
    }
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // 验证输入文件是否存在
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err(format!("file {} not found", filename))
    }
}
