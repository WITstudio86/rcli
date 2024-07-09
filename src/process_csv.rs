use anyhow::{Error, Result};
use colored::Colorize;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::Path, process};
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
        println!("{} invalid value {} for '--output <OUTPUT>': {} \n\n For more information, try '--help'.",
            "error:".red(),
            format!("'{}'",filename).yellow(),
            "file already exists".blue()
        );
        // 不继续写入 , 结束程序
        process::exit(1);
    }
}

// 读取 csv 文件
pub fn read_cev(input: &str) -> Result<Vec<Value>, Error> {
    // 将 input 内容解析
    let mut reader = Reader::from_path(input)?;
    // 获取 header , 需要 clone 没有的话就是一个写入引用
    let headers = reader.headers()?.clone();
    // 创建存储器
    let mut zip_vec = Vec::with_capacity(128);
    // 遍历每一个 record
    for record in reader.records() {
        // 解析出值
        let record = record?;
        // 压缩为元组并存入zip_vec , headers 和 record 的长度必须一致 , 会压缩乘 Object 结构体的不同属性和值
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        zip_vec.push(json_value);
    }
    Ok(zip_vec)
}

// 将 csv 转化为 json文件
pub fn process_csv_to_json(records: Vec<Value>, output: &str) -> Result<()> {
    // 转换为 json
    let json_ret = serde_json::to_string_pretty(&records)?;
    // 写入目标文件
    fs::write(output, json_ret)?;
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
