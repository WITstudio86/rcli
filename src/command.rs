use std::path::Path;

use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Opts {
    #[command(subcommand)]
    pub sub: Subcommand,
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    /// read csv and convert to other formats
    CSV(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// input file path
    #[arg(short, long , value_parser=is_file_exist)]
    pub input: String,
    /// output file path
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
}

fn is_file_exist(filename: &str) -> Result<String> {
    let file = Path::new(filename);
    if file.exists() {
        Ok(filename.into())
    } else {
        Err(anyhow!("❌: file dose not existed"))
    }
}

#[cfg(test)]
mod test {
    use crate::command::is_file_exist;

    #[test]
    fn test_file_exist() {
        assert!(is_file_exist("fjsadklfjdsakhfudsafhndjukaildfjjkl").is_err())
    }
}
