use std::{fmt::Display, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Base64Commands {
    #[arg(short, default_value = "encode")]
    pub way: CodeType,
    #[arg(short, long, default_value = "-")]
    pub input: String,
    #[arg(short, long, default_value = "safe")]
    pub alphabet: AlphabetType,
}

#[derive(Clone, Copy, Debug)]
pub enum CodeType {
    Encode,
    Decode,
}

impl Display for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeType::Encode => write!(f, "encode"),
            CodeType::Decode => write!(f, "decode"),
        }
    }
}

impl FromStr for CodeType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "encode" => Ok(CodeType::Encode),
            "decode" => Ok(CodeType::Decode),
            _ => Err("Invalid code type".to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AlphabetType {
    Safe,
    Standard,
}

impl Display for AlphabetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlphabetType::Safe => write!(f, "safe"),
            AlphabetType::Standard => write!(f, "standard"),
        }
    }
}

impl FromStr for AlphabetType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "safe" => Ok(AlphabetType::Safe),
            "standard" => Ok(AlphabetType::Standard),
            _ => Err("Invalid alphabet type".to_string()),
        }
    }
}
