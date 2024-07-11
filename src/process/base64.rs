use crate::cli_resolve::{AlphabetType, CodeType};
use anyhow::{Error, Result};
use base64::{engine::general_purpose::URL_SAFE, prelude::*};
pub fn prcocess_base64(
    way: CodeType,
    content: &str,
    alphabet: AlphabetType,
) -> Result<String, Error> {
    match way {
        CodeType::Decode => Ok(decode_base64(content, alphabet)?),
        CodeType::Encode => Ok(encode_base64(content, alphabet)?),
    }
}

fn encode_base64(content: &str, alphabet: AlphabetType) -> Result<String, Error> {
    match alphabet {
        AlphabetType::Standard => {
            let encode = BASE64_STANDARD.encode(content);
            Ok(encode)
        }
        AlphabetType::Safe => {
            let encoded = URL_SAFE.encode(content);
            Ok(encoded)
        }
    }
}

fn decode_base64(content: &str, alphabet: AlphabetType) -> Result<String, Error> {
    match alphabet {
        AlphabetType::Standard => {
            let decoded = BASE64_STANDARD.decode(content.trim())?;
            Ok(String::from_utf8(decoded)?)
        }
        AlphabetType::Safe => {
            let decoded = URL_SAFE.decode(content.trim())?;
            Ok(String::from_utf8(decoded)?)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_base64() {
        let content = "hello world";
        let encoded = encode_base64(content, AlphabetType::Standard).unwrap();
        let decoded = decode_base64(&encoded, AlphabetType::Standard).unwrap();
        assert_eq!(content, decoded);
    }
    #[test]
    fn test_base64_safe() {
        let content = "hello world";
        let encoded = encode_base64(content, AlphabetType::Safe).unwrap();
        let decoded = decode_base64(&encoded, AlphabetType::Safe).unwrap();
        assert_eq!(content, decoded);
    }
}
