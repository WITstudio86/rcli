use anyhow::Result;
use colored::Colorize;
use rand::seq::SliceRandom;
use std::process;
use zxcvbn::{zxcvbn, Score};

const UPPERCHARS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCHARS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const SYMBOLS: &[u8] = b"!@#$%^&*_";
const NUMBERS: &[u8] = b"123456789";

pub fn generate_password(
    length: u8,
    no_upper: bool,
    no_lower: bool,
    no_symbol: bool,
    no_number: bool,
) {
    // 验证密码
    if let Err(e) = verify_password(no_upper, no_lower, no_symbol, no_number) {
        // 在什么都包含的情况下输出错误及提示信息
        eprintln!("{e}");
        // 终止程序
        process::exit(1);
    }

    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();
    // 备选区域
    let mut chars: Vec<u8> = Vec::new();
    // 已选区域
    let mut password: Vec<u8> = Vec::new();
    // TODO: Implement password generation
    if !no_upper {
        chars.extend(UPPERCHARS);
        // 确保结果中至少有一个大写字母
        password.push(*UPPERCHARS.choose(&mut rng).unwrap());
    }
    if !no_lower {
        chars.extend(LOWERCHARS);
        password.push(*LOWERCHARS.choose(&mut rng).unwrap());
    }
    if !no_symbol {
        chars.extend(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).unwrap());
    }
    if !no_number {
        chars.extend(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).unwrap());
    }

    // 生成剩余的字符
    for _ in 0..(length - password.len() as u8) {
        password.push(*chars.choose(&mut rng).unwrap());
    }

    // 乱序处理
    password.shuffle(&mut rng);
    let password = String::from_utf8(password).unwrap();
    // 代码强度
    let strength = zxcvbn(&password, &[]).score();

    print_password_and_strength(password, strength);
}

fn verify_password(
    no_upper: bool,
    no_lower: bool,
    no_symbol: bool,
    no_number: bool,
) -> Result<(), String> {
    if no_upper && no_lower && no_symbol && no_number {
        return Err(format!(
            "{} No character type selected , {}",
            "Error:".red(),
            "please select at least one character type".yellow()
        ));
    }
    Ok(())
}

fn print_password_and_strength(password: String, strength: Score) {
    match strength {
        Score::Zero => {
            println!("{}", password.red());
            eprintln!("Password strength: {}", strength.to_string().red());
        }
        Score::One => {
            println!("{}", password.red());
            eprintln!("Password strength: {}", strength.to_string().red());
        }
        Score::Four => {
            println!("{}", password.green());
            eprintln!("Password strength: {}", strength.to_string().green());
        }
        _ => {
            println!("{}", password.yellow());
            eprintln!("Password strength: {}", strength.to_string().yellow());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password() {
        generate_password(16, true, false, true, false);
    }

    #[test]
    fn test_password2() {
        generate_password(16, false, false, false, false);
    }
}
