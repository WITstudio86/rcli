use anyhow::Result;
use clap::Parser;
use rcli::{generate_password, process_csv, show, verify_file_exist, Args, Command, Csvcmd};

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
        }
    }
    Ok(())
}
