use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use rcli::{
    generate_password, prcocess_base64, process_csv, show, verify_file_exist, Args, Command, Csvcmd,
};
use std::io::{stdin, Read};

fn main() -> Result<()> {
    // 解析命令行参数
    let args = Args::parse();

    // println!("{:?}", args);
    match args.cmd {
        Command::Csv(csvcmd) => {
            match csvcmd {
                Csvcmd::Show(csv_opts) => {
                    // 显示文件内容
                    show(&csv_opts.input)?;
                    Ok(())
                }
                Csvcmd::Format(csv_opts) => {
                    let input = csv_opts.input;
                    let replace = csv_opts.replace;
                    let out_type = csv_opts.output_type;
                    let output = if let Some(file_name) = csv_opts.output {
                        format!("{}.{}", file_name, out_type)
                    } else {
                        format!("output.{}", out_type)
                    };

                    // 验证在非替换情况下是否存在同名文件
                    verify_file_exist(&output, replace);
                    // 读取并转换对应类型并写入文件
                    process_csv(&input, &output, out_type)?;
                    Ok(())
                }
            }
        }
        Command::Genpassword(gen_opts) => {
            generate_password(
                gen_opts.length,
                gen_opts.no_upper,
                gen_opts.no_lower,
                gen_opts.no_number,
                gen_opts.no_symbol,
            );

            Ok(())
        }
        Command::Base64(base64_opts) => {
            let mut reader: Box<dyn Read> = if base64_opts.input == *"-".to_string() {
                eprint!("输入内容之后按两次 ctrl+D 结束输入: ");
                Box::new(stdin())
            } else {
                Box::new(std::fs::File::open(&base64_opts.input)?)
            };

            let mut content = String::new();
            reader.read_to_string(&mut content)?;

            let result = prcocess_base64(base64_opts.way, &content, base64_opts.alphabet)?;
            eprintln!("\n{}", "result:".blue());
            println!("{}", result.green());
            Ok(())
        }
    }
}
