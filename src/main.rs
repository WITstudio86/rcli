use anyhow::Result;
use clap::Parser;
use rcli::{process_csv_to_json, read_cev, show, verify_file_exist, Args, Command, Csvcmd};

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
                }
                Csvcmd::Json(csv_opts) => {
                    let input = csv_opts.input;
                    let replace = csv_opts.replace;
                    let output = csv_opts.output;

                    // 验证在非替换情况下是否存在同名文件
                    verify_file_exist(&input, replace);
                    // 读取 csv
                    let csv_content = read_cev(&input)?;
                    // 转换为 json并写入
                    process_csv_to_json(csv_content, &output)?;
                }
            }
        }
    }
    Ok(())
}
