use anyhow::Result;
use colored::Colorize;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path, process};

use crate::{cli_resolve::FormatType, to_toml_string};
// 解析 csv 文件的结构体
#[derive(Debug, Deserialize, Serialize)]
// 定义解析的目标名 , 所有设置为首字母大写
#[serde(rename_all = "PascalCase")]
pub struct Record {
    name: String,
    position: String,
    #[serde(rename = "DOB")] // 个别需要单独设置
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit_number: u8,
}

// 验证文件是否存在
pub fn verify_file_exist(filename: &str, replace: bool) {
    // 如果在非替换模式下重名 , painc 程序
    if !replace && Path::new(filename).exists() {
        println!(
            "{} invalid value {} for '--output <OUTPUT>': {} \n\n For more information, try '-r'.",
            "error:".red(),
            format!("'{}'", filename).yellow(),
            "file already exists".blue()
        );
        // 不继续写入 , 结束程序
        process::exit(1);
    }
}

// 将 csv 转化为 对应文件
pub fn process_csv(input: &str, output: &str, format_type: FormatType) -> Result<()> {
    let mut _ret = String::new();

    // 读取 csv 文件
    let mut reader = Reader::from_path(input)?;
    // 遍历 reader 中的所有 record
    let header = reader.headers()?.clone();

    // 判断什么生成类型
    match format_type {
        FormatType::Toml => {
            _ret = to_toml_string(header, &mut reader)?;
        }
        _ => {
            let mut ret_vec = Vec::with_capacity(128);
            for record in reader.records() {
                let record = record?;
                let zipped = header
                    .iter()
                    .zip(record.iter())
                    .collect::<serde_json::Value>();
                ret_vec.push(zipped);
            }
            _ret = serde_json::to_string(&ret_vec)?;
        }
    }

    // 写入目标文件
    fs::write(output, _ret)?;
    println!("写入完成");
    Ok(())
}

// 打印 csv::reader 的 record
macro_rules! print_record {
    ($record:expr,$color:ident) => {
        print!("{} ", "|".yellow());
        for i in $record.iter() {
            // 40个字符
            print!("{:30}{}  ", i.$color(), "|".yellow());
        }
        println!("\n{}", "-".to_string().repeat(165).yellow());
    };
}

// 将 csv 打印到终端
pub fn show(filename: &str) -> Result<()> {
    let mut reader = Reader::from_path(filename)?;
    let header = reader.headers()?.clone();
    print_record!(header, blue);
    for record in reader.records() {
        print_record!(record?, green);
    }
    Ok(())
}
