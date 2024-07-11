use clap::Parser;

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, help = "password length", default_value_t = 8)]
    pub length: u8,
    #[arg(long = "nu", help = "upper case or not", default_value_t = false)]
    pub no_upper: bool,
    #[arg(long = "nl", help = "lower case or not", default_value_t = false)]
    pub no_lower: bool,
    #[arg(long = "ns", help = "symbol char or not", default_value_t = false)]
    pub no_symbol: bool,
    #[arg(long = "nn", help = "number char or not", default_value_t = false)]
    pub no_number: bool,
}
